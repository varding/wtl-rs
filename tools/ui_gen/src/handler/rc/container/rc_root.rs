
use regex::Regex;
use std::path::PathBuf;
use std::collections::BTreeMap;
use std::fs::{self,File};
use std::io::Write;
use winapi::WORD;
use handler::rc::util::*;
use super::{Container,Name};

pub struct RcRoot {
    root: Container,
    consts: BTreeMap<String,WORD>,
}

impl RcRoot {
	pub fn new()->RcRoot{
		RcRoot{
			root: Container::with_name(Name::root()),
			consts: BTreeMap::new(),
		}
	}

	/// consts from the resouce.h,these consts will be used in ui mod and handler mod
	pub fn set_consts(&mut self,c: BTreeMap<String,WORD>) {
		self.consts = c;
	}

	/// parse dialog using the give text from rc_file
	pub fn parse_dialog(&mut self,id: &str, data: &str){
		let n = Name::dialog(id);
		let mut dlg = Container::with_name(n);
		let re_begin = Regex::new(r"\sBEGIN\s").unwrap();
		if let Some(begin_pos) = re_begin.find(data) {
			let ctrl_begin = begin_pos.1;
			dlg.parse_ctrls(&data[ctrl_begin..data.len() - 3]);	//delte "END"
		}
		self.root.add_child(id, Box::new(dlg));
	}

	/// construct a tree structure as the given path,and return all it's container
	pub fn make_path(&mut self, selected_item_name: &str, p: &mut Vec<String>)->Vec<String> {
		//select: IDD_ABOUT_DLG, p: [IDD_MAIN_DLG,Root]
		println!("direct parent path:{:?}", p);
		// pop "root" first
		let r = p.pop().expect("root should be pushed");
		assert!(r == "Root");
		if p.len() == 0 {
			//select: IDD_MAIN_DLG, p: []
			// dlg already in rc_root,so return containers of direct children and set path 
			p.push(selected_item_name.to_string());
			let mut dlg = self.root.from_path(p).expect("container not exist");
			//p.pop().unwrap();
			//actually it is empty
			//dlg.set_path(p);
			return dlg.direct_child_container();
		}

		//[about_dlg,main_dlg]
		//p is reverse order,so p.pop returns root_dlg_name of the selected_item to insert
		//let root_dlg_name = p.pop().unwrap();

		//the selected dlg in listbox,so remove it from rc_root first
		let mut selected_dlg = self.root.delete_child(selected_item_name);//self.root.children.remove(selected_item_name).expect("dlg in root should exist");
		
		let ret = selected_dlg.direct_child_container();
		//use p as path of child
		//select: IDD_ABOUT_DLG, p: [IDD_MAIN_DLG]
		//selected_dlg.set_path(p);

		let direct_parent_dlg = self.root.from_path(p).expect("container shold exist");
		//let root_dlg = self.dlgs.get_mut(&root_dlg_name).expect("dlg should exist");
		//let mut direct_parent_dlg = root_dlg.make_path(p).expect("parent dlg should be found");
		direct_parent_dlg.add_child(selected_item_name,selected_dlg);

		ret
	}

	/// delete a give path,and return all containers(only dlgs)
	pub fn delete_path(&mut self,p: Vec<String>)->Vec<String> {
		Vec::new()
	}

	/// print the tree structure
	pub fn print(&self) {
		//println!("Root");
		self.root.print(0);
	}
}


impl RcRoot {
	pub fn write_files(&mut self) {

    	
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
		writeln!(f,"{}",MSG_LOOP_FILE).unwrap();
	}
}

impl RcRoot {
    	/// system binding for all controls in dialog
	fn write_binding_file(&self, mut system_binding_path: PathBuf) {
		let mut dlg_names: Vec<String> = Vec::new();
		// for (_,d) in &self.dlgs {
		// 	d.write_binding_file(&mut system_binding_path,&mut dlg_names);
		// }
		self.root.write_binding_file(&mut system_binding_path,&mut dlg_names);

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
