
use std::path::PathBuf;
use super::Control;
use std::fs::{self,File};
use std::io::Write;
use std::collections::HashMap;
//use std::fmt;

#[derive(Debug)]
pub struct Dialog {
	id: String,
    ctrls: Vec<Control>,
    children: HashMap<String,Box<Dialog>>,
}

// impl fmt::Display for Dialog {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let r = write!(f, "{}", self.id);
//         for (id,_) in &self.children {
//         	println!("{}", id);
//         }
//         r
//     }
// }

impl Dialog {
	pub fn new(id: &str)->Dialog{
		Dialog{
			id: id.to_string(),
			ctrls: Vec::new(),
			children: HashMap::new(),
		}
	}

	fn add_ctrl(&mut self,c: Control){
		self.ctrls.push(c);
	}

	pub fn add_child(&mut self,id: &str, d: Box<Dialog>){
		println!("add child,parent:{},child:{}", self.id,d.id);
		self.children.insert(id.to_string(), d);
	}

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

	pub fn make_path(&mut self, p: &mut Vec<String>) -> Option<&mut Self> {
		if let Some(last) = p.pop() {
			let d = self.children.get_mut(&last[..]).expect("dlg should exist");
			return d.make_path(p);
		}else{
			Some(self)
		}
	}

	pub fn print(&self,depth: i32) {
		for i in (0..depth) {
			print!("    ");
		}
		println!("{}", self.id);
		for (_,c) in &self.children {
			c.print(depth+1);
		}
	}
}

impl Dialog {
	pub fn write_file(&self,cur_path: &mut PathBuf) {
		//get name first:use as file name and child dir name
		let id = self.id.trim();
		let name = if id.starts_with("IDD_") {
			id[4..].to_lowercase()
		}else{
			id[..].to_lowercase()
		};
		let name = &name[..];

		//enter child path
		cur_path.push(name);
		
		//recursive write
		for (_,c) in &self.children {
			c.write_file(cur_path);
		}
		//leave child path
		cur_path.pop();
		
		cur_path.push(format!("{}.rs",name));
		//create dir if not exist
		fs::create_dir_all(cur_path.as_path().parent().unwrap().clone()).expect("create dir fail");

		//write current file
		let mut f = File::create(cur_path.clone()).unwrap();
		self.write_declaration(name,&mut f);
		self.write_impl(name,&mut f);
		cur_path.pop();			//delete file name
	}
}

impl Dialog {
	fn write_declaration(&self,name: &str,f: &mut File) {
		writeln!(f,"pub struct {} {{", name).unwrap();
		for c in &self.ctrls {
			//writeln!(f,"\tpub {}",c.write());
			c.write_file(f);
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
		writeln!(f,"\t}}").unwrap();
	}

	fn write_create_dialog(&self,f: &mut File) {

	}

	// fn write_attach_control(&self,f: &File) {

	// }

	fn write_msg(&self,f: &mut File) {

	}	
}