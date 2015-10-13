
use std::path::PathBuf;
use super::Control;
use std::fs::{self,File};
use std::io::Write;
use std::collections::BTreeMap;
use super::util::*;
//use std::fmt;

#[derive(Debug)]
pub struct Dialog {
	id: String,
    ctrls: Vec<Control>,
    children: BTreeMap<String,Box<Dialog>>,
    path: String,	//path in handler,like:t.main_dialog.about_dlg
}

impl Dialog {
	pub fn new(id: &str)->Dialog{
		Dialog{
			id: id.trim().to_string(),
			ctrls: Vec::new(),
			children: BTreeMap::new(),
			path: String::new(),
		}
	}

	/// called in construction stage, add child of the tree path
	pub fn add_child(&mut self,id: &str, d: Box<Dialog>){
		println!("add child,parent:{},child:{}", self.id,d.id);
		self.children.insert(id.to_string(), d);
	}

	/// parse every line in begin ... end as a control
	pub fn parser_ctrls(&mut self,data: &str){
		let ctrls = data.trim();
		let lines: Vec<&str> = ctrls.lines().collect();
		for l in &lines {
			let tl = l.trim();
			//self.parse_ctrl(tl);
			self.add_ctrl(Control::new(tl));
		}
		//println!("dialog: {}\n{:?}",self.id, self.ctrls);
	}

	/// construct a tree structure as the given path
	pub fn make_path(&mut self, p: &mut Vec<String>) -> Option<&mut Self> {
		if let Some(last) = p.pop() {
			let d = self.children.get_mut(&last[..]).expect("dlg should exist");
			return d.make_path(p);
		}else{
			Some(self)
		}
	}

	/// make_path of dialog in leave can't be called,so use this function to set path of all dialog
	pub fn set_path(&mut self,p: &Vec<String>) {
		let jp = p.join(".");
		self.path.push_str(&jp[..]);
		self.path.push_str(&dlg_id_to_name(&self.id[..]));
		println!("path vec:{:?},path:{}", p,self.path);
	}

	/// recusive print 
	pub fn print(&self,depth: i32) {
		for i in (0..depth) {
			print!("    ");
		}
		println!("{}", self.id);
		for (_,c) in &self.children {
			c.print(depth+1);
		}
	}

	/// simply add a parsed control
	fn add_ctrl(&mut self,c: Control){
		self.ctrls.push(c);
	}
}

impl Dialog {
	pub fn write_file(&self,mod_file: &mut File,cur_path: &mut PathBuf) {
		//get name first:use as file name and child dir name
		let name = dlg_id_to_name(&self.id[..]);
		let sub_dir_name = format!("sub_{}",name);
		let name = &name[..];
		
		
		if self.children.len() > 0 {
			//enter child path
			cur_path.push(sub_dir_name.clone());

			//create child dir first
			fs::create_dir_all(cur_path.as_path().clone()).expect("create dir fail");
			// create mod.rs for child directory,they append mod and it's sub mod to this file
			let mut child_mod_path = cur_path.clone();
			child_mod_path.push("mod.rs");
			let mut child_mod_file = File::create(child_mod_path.as_path()).unwrap();

			//recursive write
			for (_,c) in &self.children {
				c.write_file(&mut child_mod_file,cur_path);
			}

			//leave child path
			cur_path.pop();
		}
		
		//append mod of this Dialog and it's child dialogs  to the end of the mod.rs in current directory
		self.append_mod_file(mod_file);
		//write this Dialog
		cur_path.push(format!("{}.rs",name));
		//create dir if not exist
		fs::create_dir_all(cur_path.as_path().parent().unwrap().clone()).expect("create dir fail");
		//write current file
		let mut f = File::create(cur_path.clone()).unwrap();
		//write use statements
		writeln!(f,"#![allow(dead_code)]").unwrap();
		writeln!(f,"use wtl::*;").unwrap();
		writeln!(f,"use ui::consts::*;").unwrap();
		if self.children.len() > 0 {
			writeln!(f,"use super::{}::*;",sub_dir_name).unwrap();
		}
		//struct name should be camel case
		let camel_name = to_camel_case(name);
		self.write_declaration(&camel_name[..],&mut f);
		self.write_impl(&camel_name[..],&mut f);
		cur_path.pop();			//delete file name
	}

	/*
	write mod sub_xxx in the end of mod.rs in current directory
	write this mod at the end of the mod file
	*/
	//write child dialogs in the mod.rs of sub
	fn append_mod_file(&self,f: &mut File) {
		let mod_name = dlg_id_to_name(&self.id[..]);
		writeln!(f,"mod {};",mod_name).unwrap();
		writeln!(f,"pub use self::{}::*;",mod_name).unwrap();

		if self.children.len() > 0 {
			writeln!(f,"mod sub_{};",mod_name).unwrap();
			writeln!(f,"pub use self::sub_{}::*;",mod_name).unwrap();
		}
	}

	/// write default handler that binds all controls
	pub fn write_binding_file(&self,mut system_binding_path: &mut PathBuf,dlg_names: &mut Vec<String>) {

		// recursive write,all binding files in the same directory
		for (_,c) in &self.children {
				c.write_binding_file(system_binding_path,dlg_names);
		}

		// no ctrls,return
		if self.ctrls.len() == 0{
			return;
		}

		let name  = dlg_id_to_name(&self.id[..]);
		system_binding_path.push(format!("{}.rs",name));
		let mut f = File::create(system_binding_path.as_path().clone()).unwrap();
		system_binding_path.pop();

		writeln!(f,"use ui::Root;").unwrap();
		writeln!(f,"use user32;").unwrap();
		writeln!(f,"use winapi::*;").unwrap();
		writeln!(f,"use ui::consts::*;").unwrap();
		//only root dialog write this
		//writeln!(f,"\t\tr.main_dialog.this_msg().on_close(|_,_|{\r\n\t\t\tunsafe{user32::PostQuitMessage(0)};\r\n\t\t});").unwrap();
		
		writeln!(f,"pub fn register_handler(r: &mut Root) {{").unwrap();
		writeln!(f,"\tr.{}.this_msg().on_init_dialog(|_,t|{{",self.path).unwrap();
		writeln!(f,"\t\tt.{}.this.CenterWindow(0 as HWND);",self.path).unwrap();
		writeln!(f,"\t\tlet this = &t.main_dialog.this;").unwrap();
		for c in &self.ctrls {
			c.write_binding(&self.path,&mut f);
		}

		writeln!(f,"\t}}).set_system_priority(0);").unwrap();
		writeln!(f,"}}").unwrap();

		dlg_names.push(name.to_string());
		//self.append_binding_mod_file(binding_mod_file);
	}
}

impl Dialog {
	fn write_declaration(&self,name: &str,f: &mut File) {
		writeln!(f,"pub struct {}<T> {{", name).unwrap();
		writeln!(f,"\tpub this: Dialog<T>,").unwrap();
		
		//declaration of child dialogs
		for (id,_) in &self.children {
			let name = dlg_id_to_name(id);
			writeln!(f,"\tpub {}: {}<T>,",name,to_camel_case(&name[..])).unwrap();
		}

		//declaration of ctrls
		for c in &self.ctrls {
			//writeln!(f,"\tpub {}",c.write());
			c.write_declaration(f);
		}

		writeln!(f,"}}").unwrap();
	}

	fn write_impl(&self,name: &str,f: &mut File) {
		writeln!(f,"impl<T> {}<T> {{",name).unwrap();
		self.write_new(name,f);
		self.write_create_dialog(f);
		self.write_msg(f);
		writeln!(f,"}}").unwrap();
	}
}

impl Dialog {
	fn write_new(&self,name: &str,f: &mut File){
		writeln!(f,"\tpub fn new()->{}<T>{{",name).unwrap();
		writeln!(f,"\t\t{}{{",name).unwrap();
		writeln!(f,"\t\t\tthis: Dialog::new({}),",self.id).unwrap();
		
		//net instance of child dialogs
		for (id,_) in &self.children {
			let name = dlg_id_to_name(id);
			writeln!(f,"\t\t\t{}: {}::new(),",name,to_camel_case(&name[..])).unwrap();
		}

		//new instance of ctrls
		for c in &self.ctrls {
			//writeln!(f,"\tpub {}",c.write());
			c.write_new(f);
		}

		writeln!(f,"\t\t}}").unwrap();
		writeln!(f,"\t}}").unwrap();
	}

	fn write_create_dialog(&self,f: &mut File) {
		writeln!(f,"\tpub fn create(&mut self,r: *mut T){{").unwrap();
		writeln!(f,"\t\tself.this.Create3(r);").unwrap();
		for (id,_) in &self.children {
			writeln!(f,"\t\tself.{}.create(r);",dlg_id_to_name(id)).unwrap();
		}
		writeln!(f,"\t}}").unwrap();
	}

	// fn write_attach_control(&self,f: &File) {

	// }

	fn write_msg(&self,f: &mut File) {
		writeln!(f,"\tpub fn this_msg(&mut self)->DlgMsg<T> {{").unwrap();
		writeln!(f,"\t\tself.this.msg_handler()").unwrap();
		writeln!(f,"\t}}").unwrap();

		for c in &self.ctrls {
			c.write_msg(f);
		}
	}
}
