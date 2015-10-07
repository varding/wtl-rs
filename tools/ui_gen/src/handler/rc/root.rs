use regex::Regex;
use super::Dialog;
use std::path::PathBuf;
use std::collections::HashMap;
use winapi::WORD;
use std::fs::{self,File};
use std::io::Write;

#[derive(Debug)]
pub struct RcRoot {
    pub dlgs: HashMap<String,Box<Dialog>>,
    consts: HashMap<String,WORD>,
}

// impl fmt::Display for RcRoot {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     	let r = write!(f,"Root");
//         for (id,_) in &self.dlgs {
//         	println!("{}", id);
//         }
//         r
//     }
// }

impl RcRoot {
	pub fn new()->RcRoot{
		RcRoot{
			dlgs: HashMap::new(),
			consts: HashMap::new(),
		}
	}

	pub fn set_consts(&mut self,c: HashMap<String,WORD>) {
		self.consts = c;
	}

	pub fn parse_dialog(&mut self,id: &str, data: &str){
		let mut dlg = Box::new(Dialog::new(id));
		let re_begin = Regex::new(r"\sBEGIN\s").unwrap();
		if let Some(begin_pos) = re_begin.find(data) {
			let ctrl_begin = begin_pos.1;
			dlg.parser_ctrls(&data[ctrl_begin..data.len() - 3]);	//delte "END"
		}
		self.dlgs.insert(id.to_string(), dlg);
	}

	//set the 
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

		let child = self.dlgs.remove(id).expect("dlg in root should exist");

		let dlg = self.dlgs.get_mut(&dlg_name[..]).expect("dlg should exist");
	
		let mut direct_parent_dlg = dlg.make_path(p).expect("parent dlg should be found");

		direct_parent_dlg.add_child(id,child);
	}

	pub fn print(&self) {
		println!("Root");
		for (_,d) in &self.dlgs {
			d.print(1);
		}
	}
}


impl RcRoot {
	pub fn write_files(&self){
		let mut cur_path = PathBuf::from(".\\ui");
		println!("cur path: {:?}", cur_path);

		self.write_consts(cur_path.clone());

		for (_,d) in &self.dlgs {
			d.write_file(&mut cur_path);
		}
	}

	fn write_consts(&self, mut cur_path: PathBuf) {
		cur_path.push("consts.rs");
		//create dir if not exist
		fs::create_dir_all(cur_path.as_path().parent().unwrap().clone()).expect("create dir fail");

		let mut f = File::create(cur_path.as_path().clone()).unwrap();
		writeln!(f,"use winapi::WORD;").unwrap();;
		for (id,value) in &self.consts {
			writeln!(f,"pub const {}: WORD = {};",id,value).unwrap();
		}
	}

	pub fn write_Rcroot_file(&self) {

	}

	pub fn write_mod_file(&self) {

	}

	pub fn write_message_loop(&self) {
		
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