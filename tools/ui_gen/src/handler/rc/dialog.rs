
use std::path::PathBuf;
use super::Control;
use std::fs::{self,File};
use std::io::Write;
pub struct Dialog {
	id: String,
    ctrls: Vec<Control>,
    children: Vec<Box<Dialog>>,
}

impl Dialog {
	pub fn new(id: &str)->Dialog{
		Dialog{
			id: id.to_string(),
			ctrls: Vec::new(),
			children: Vec::new(),
		}
	}

	fn add_ctrl(&mut self,c: Control){
		self.ctrls.push(c);
	}

	pub fn add_child(&mut self,d: Dialog){
		self.children.push(Box::new(d));
	}

	pub fn parser_dialog(&mut self,data: &str){
		let ctrls = data.trim();
		let lines: Vec<&str> = ctrls.lines().collect();
		for l in &lines {
			let tl = l.trim();
			//self.parse_ctrl(tl);
			self.add_ctrl(Control::new(tl));
		}
		//println!("dialog: {}\n{:?}",self.id, self.ctrls);
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
		//create dir if not exist
		fs::create_dir_all(cur_path.clone()).expect("create dir fail");
		//recursive write
		for c in &self.children {
			c.write_file(cur_path);
		}
		//leave child path
		cur_path.pop();
		
		cur_path.set_file_name(format!("{}.rs",name));
		//write current file
		let mut f = File::create(cur_path.clone()).unwrap();
		self.write_declaration(name,&mut f);
		self.write_impl(name,&mut f);
		cur_path.pop();			//delete file name
	}
}

impl Dialog {
	fn write_declaration(&self,name: &str,f: &mut File) {
		println!("pub struct {} {{", name);
		for c in &self.ctrls {
			println!("\tpub {}",c.write());
		}
		println!("}}");
	}

	fn write_impl(&self,name: &str,f: &mut File) {
		write!(f,"impl<T> {}<T> {{",name);
		self.write_new(name,f);
		self.write_create_dialog(f);
		self.write_msg(f);
		write!(f,"}}");
	}	
}

impl Dialog {
	fn write_new(&self,name: &str,f: &mut File){
		write!(f,"\tpub fn new()->{}<T>{{",name);
		write!(f,"}}");


		write!(f,"}}");
	}

	fn write_create_dialog(&self,f: &mut File) {

	}

	// fn write_attach_control(&self,f: &File) {

	// }

	fn write_msg(&self,f: &mut File) {

	}	
}