use regex::Regex;
use super::Dialog;
use std::path::PathBuf;

pub struct RcRoot {
    dlgs: Vec<Dialog>,
}

impl RcRoot {
	pub fn new()->RcRoot{
		RcRoot{
			dlgs: Vec::new(),
		}
	}

	pub fn parse_dialog(&mut self,data: &str){
		let re_dlg_name = Regex::new(r"(\w+)\s+DIALOGEX").unwrap();
		let mut dlg: Dialog;
		if let Some(cap) = re_dlg_name.captures(data) {
			dlg = Dialog::new(cap.at(1).unwrap());
		}else{
			return;
		}

		let re_begin = Regex::new(r"\sBEGIN\s").unwrap();
		if let Some(begin_pos) = re_begin.find(data) {
			let ctrl_begin = begin_pos.1;
			dlg.parser_dialog(&data[ctrl_begin..data.len() - 3]);	//delte "END"
		}
	}
}


impl RcRoot {
	pub fn write_files(&self){
		let mut cur_path = PathBuf::from(".");
		for d in &self.dlgs {
			d.write_file(&mut cur_path);
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