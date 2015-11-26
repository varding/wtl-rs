use regex::Regex;
use super::Dialog;
use std::path::PathBuf;
use std::collections::BTreeMap;
use winapi::WORD;
use std::fs::{self,File};
use std::io::Write;
use handler::rc::util::*;

#[derive(Debug)]
pub struct RcRoot {
    pub dlgs: BTreeMap<String,Box<Dialog>>,
    consts: BTreeMap<String,WORD>,
}

impl RcRoot {
	pub fn new()->RcRoot{
		RcRoot{
			dlgs: BTreeMap::new(),
			consts: BTreeMap::new(),
		}
	}

	/// consts from the resouce.h,these consts will be used in ui mod and handler mod
	pub fn set_consts(&mut self,c: BTreeMap<String,WORD>) {
		self.consts = c;
	}

	/// parse dialog using the give text from rc_file
	pub fn parse_dialog(&mut self,id: &str, data: &str){
		let mut dlg = Box::new(Dialog::new(id));
		let re_begin = Regex::new(r"\sBEGIN\s").unwrap();
		if let Some(begin_pos) = re_begin.find(data) {
			let ctrl_begin = begin_pos.1;
			dlg.parser_ctrls(&data[ctrl_begin..data.len() - 3]);	//delte "END"
		}
		self.dlgs.insert(id.to_string(), dlg);
	}

	/// construct a tree structure as the given path
	pub fn make_path(&mut self, id: &str, p: &mut Vec<String>) {
		//assert!(p.pop() = Some("Root".to_string()));
		let r = p.pop().expect("root should be pushed");
		assert!(r == "Root");

		println!("p:{:?}", p);
		if p.len() == 0 {
			return;
		}
		//println!("{:?}", p);
		// if p.len() == 1 {
		// 	//dialogs that belongs to the root
		// 	assert!(self.dlgs.contains_key(&p[0]));
		// 	return;
		// }

		let dlg_name = p.pop().unwrap();
		let mut child = self.dlgs.remove(id).expect("dlg in root should exist");
		child.set_path(p);
		let dlg = self.dlgs.get_mut(&dlg_name[..]).expect("dlg should exist");
		let mut direct_parent_dlg = dlg.make_path(p).expect("parent dlg should be found");
		direct_parent_dlg.add_child(id,child);
	}

	/// print the tree structure
	pub fn print(&self) {
		println!("Root");
		for (_,d) in &self.dlgs {
			d.print(1);
		}
	}
}

impl RcRoot {
	/// write the parsed content to files in ui directory and handler directory
	pub fn write_files(&mut self){
		let mut cur_path = PathBuf::from(".\\ui");
		//all default bindings write in the same directory
		let mut system_binding_path = PathBuf::from(".\\handler\\system");
		println!("cur path: {:?}", cur_path);

		fs::create_dir_all(cur_path.as_path().clone()).expect("create dir fail");
		fs::create_dir_all(system_binding_path.as_path().clone()).expect("create dir fail");

		//these files in the root of ui
		self.write_consts_file(cur_path.clone());
		self.write_message_loop_file(cur_path.clone());
		self.write_root_file(cur_path.clone());
		self.write_root_mod_file(cur_path.clone());

		//all children of root save in the ui/root dir
		cur_path.push("sub_root");
		fs::create_dir_all(cur_path.as_path().clone()).expect("create dir fail");

		// path for child dlgs of root
		let mut dlg_path: Vec<String> = Vec::new();
		//child mod.rs
		let mut child_mod_path = cur_path.clone();
		child_mod_path.push("mod.rs");
		let mut child_mod_file = File::create(child_mod_path.as_path()).unwrap();
		for (n,d) in &mut self.dlgs {
			d.write_file(&mut child_mod_file,&mut cur_path);

			//call set_path of the children in the root
			//dlg_path.push(dlg_id_to_name(&n[..]));
			d.set_path(&dlg_path);
			//dlg_path.pop().unwrap();
		}

		//binding files
		self.write_binding_file(system_binding_path);
	}

	/// write consts to ui\consts.rs
	fn write_consts_file(&self, mut cur_path: PathBuf) {
		cur_path.push("consts.rs");
		//create dir if not exist
		
		let mut f = File::create(cur_path.as_path().clone()).unwrap();
		writeln!(f,"#![allow(dead_code)]").unwrap();
		writeln!(f,"use winapi::WORD;").unwrap();
		for (id,value) in &self.consts {
			writeln!(f,"pub const {}: WORD = {};",id,value).unwrap();
		}
	}

	/// write ui\mod.rs
	fn write_root_mod_file(&self, mut cur_path: PathBuf) {
		cur_path.push("mod.rs");
		let mut f = File::create(cur_path.as_path()).unwrap();
		writeln!(f,"{}",ROOT_MOD_FILE).unwrap();
	}

	/// message loop 
	fn write_message_loop_file(&self, mut cur_path: PathBuf) {
		//const s: &'static str = "use winapi::*;\r\nuse user32::*;\r\n\r\npub struct MessageLoop;\r\n\r\nimpl MessageLoop {\r\n\tpub fn run(){\r\n\t\tlet mut msg = MSG{hwnd:0 as HWND,message:0,wParam:0,lParam:0,time:0,pt:POINT{x:0,y:0}};\r\n\t\tunsafe{\r\n\t\t\twhile GetMessageW( &mut msg, 0 as HWND, 0, 0 ) > 0 {\r\n\t\t\t\tTranslateMessage(&msg);\r\n\t\t\t\tDispatchMessageW(&msg);\r\n\t\t\t}\r\n\t\t}\r\n\t}\r\n}";
		cur_path.push("message_loop.rs");

		let mut f = File::create(cur_path.as_path().clone()).unwrap();
		writeln!(f,"{}",MSG_LOOP_FILE).unwrap();;
	}

	/// write ui\root.rs
	fn write_root_file(&self, mut cur_path: PathBuf) {
		cur_path.push("root.rs");
		let mut f = File::create(cur_path.as_path().clone()).unwrap();
		writeln!(f,"#![allow(dead_code)]").unwrap();
		writeln!(f,"use wtl::*;").unwrap();
		writeln!(f,"use ui::consts::*;").unwrap();
		writeln!(f,"use super::sub_root::*;").unwrap();

		//let camel_name = to_camel_case(name);
		self.write_declaration(&mut f);
		self.write_impl(&mut f);

		// writeln!(f,"pub struct Root {{");
		// for (id,_) in &self.dlgs {
		// 	writeln!(f,"").unwrap();
		// }
		// writeln!(f,"}}");
	}

	/// system binding for all controls in dialog
	fn write_binding_file(&self, mut system_binding_path: PathBuf) {
		let mut dlg_names: Vec<String> = Vec::new();
		for (_,d) in &self.dlgs {
			d.write_binding_file(&mut system_binding_path,&mut dlg_names);
		}

		// write handler\system\mod.rs
		system_binding_path.push("mod.rs");
		let mut sys_mod_file = File::create(system_binding_path.as_path()).unwrap();
		system_binding_path.pop();

		writeln!(sys_mod_file,"use ui::Root;").unwrap();

		for n in &dlg_names {
			writeln!(sys_mod_file,"mod {};",n).unwrap();
		}

		writeln!(sys_mod_file,"pub fn register_handler(r: &mut Root) {{").unwrap();
		for n in &dlg_names {
			writeln!(sys_mod_file,"\t{}::register_handler(r);",n).unwrap();
		}
		writeln!(sys_mod_file,"}}").unwrap();

		// write handler\mod.rs
		system_binding_path.pop();  //parent path
		system_binding_path.push("mod.rs");
		let mut mod_file = File::create(system_binding_path.as_path()).unwrap();
		writeln!(mod_file,"pub mod system;").unwrap();
	}
}

const ROOT_MOD_FILE: &'static str = r"
pub use self::message_loop::MessageLoop;
pub use self::root::Root;
pub use self::consts::*;

mod message_loop;
mod root;
mod sub_root;
pub mod consts;
";

const MSG_LOOP_FILE: &'static str = r"
use winapi::*;
use user32::*;

pub struct MessageLoop;

impl MessageLoop {
	pub fn run(){
		let mut msg = MSG{hwnd:0 as HWND,message:0,wParam:0,lParam:0,time:0,pt:POINT{x:0,y:0}};
		unsafe{
			while GetMessageW( &mut msg, 0 as HWND, 0, 0 ) > 0 { 
				TranslateMessage(&msg); 
				DispatchMessageW(&msg); 
			}
		}
	}
}";

//codes below copied from dialog
impl RcRoot {
	fn write_declaration(&self,f: &mut File) {
		writeln!(f,"pub struct Root {{",).unwrap();
		//writeln!(f,"\tpub this: Dialog<T>,").unwrap();
		
		//declaration of child dialogs
		for (id,_) in &self.dlgs {
			let name = dlg_id_to_name(id);
			writeln!(f,"\tpub {}: {}<Root>,",name,to_camel_case(&name[..])).unwrap();
		}

		writeln!(f,"}}").unwrap();
	}

	fn write_impl(&self,f: &mut File) {
		writeln!(f,"impl Root {{").unwrap();
		self.write_new(f);
		self.write_create_dialog(f);
		//self.write_msg(f);
		writeln!(f,"}}").unwrap();
	}
}

impl RcRoot {
	fn write_new(&self,f: &mut File){
		writeln!(f,"\tpub fn new()->Root{{").unwrap();
		writeln!(f,"\t\tRoot{{").unwrap();
		//writeln!(f,"\t\t\tthis: Dialog::new({}),",self.id).unwrap();

		//net instance of child dialogs
		for (id,_) in &self.dlgs {
			let name = dlg_id_to_name(id);
			writeln!(f,"\t\t\t{}: {}::new(),",name,to_camel_case(&name[..])).unwrap();
		}

		writeln!(f,"\t\t}}").unwrap();
		writeln!(f,"\t}}").unwrap();
	}

	fn write_create_dialog(&self,f: &mut File) {
		writeln!(f,"\tpub fn create(&mut self){{").unwrap();
		writeln!(f,"\t\tlet r = self as *mut _ ;").unwrap();
		for (id,_) in &self.dlgs {
			writeln!(f,"\t\tself.{}.create(r);",dlg_id_to_name(id)).unwrap();
		}
		writeln!(f,"\t}}").unwrap();
	}
}

/*
ui structure

mod.rs    Rcroot.rs  			Rcroot(dir)
							  / | \
				   main_dlg.rs  main_dlg(dir)  ...(other child of Rcroot and their child dir)
				   				/ | \
				   			 children of main_dlg

child dialogs are stored in the directory with the same name of the dialog
*/