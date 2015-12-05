
use std::fs::{File,self};
use std::io::{Read,Write};
use std::path::{Path,PathBuf};
use std::slice;
use std::collections::BTreeMap;
use winapi::WORD;

use regex::Regex;

use super::RcRoot;
pub struct RcFile;

impl RcFile {
	fn decode(&self, buf: Vec<u8>) -> String {
		//println!("BOM:{:?}", &buf[0..2]);
		// if buf[0..2] == [0xFE,0xFF] {
		// 	println!("big endian ucs-2");
		// 	String::new()
		// }else 
		if buf[0..2] == [0xFF,0xFE] {
			println!("encoding:little endian ucs-2");
			let mut it = buf.into_iter();
			//delete bom
			it.next();it.next();
			let new_buf = it.collect::<Vec<_>>();
			let ptr = new_buf.as_ptr() as *const u16;
			let s = unsafe{slice::from_raw_parts(ptr, new_buf.len()/2)};
			String::from_utf16_lossy(s)
		}else{
			println!("assume utf8 encoding");
			//let ptr = buf.as_ref();
			String::from_utf8(buf).unwrap()
		}
	}

	fn replace_header(&self, txt: &String) -> String{
		txt.replace("resource.h","ui.h")
	}

	fn save_as_utf8(&self,name: &str, new_name: &str, txt: &String){
		let p = Path::new(name);
		//let file_stem = p.file_stem().unwrap();
		//let file_ext = p.extension().unwrap();
		
		let mut new_path = PathBuf::from(p.parent().unwrap());
		//println!("{:?}", new_path);
		//new_path.push("utf8");
		new_path.push(new_name);
		//println!("{:?}", new_path);
		
		//create dir first
		fs::create_dir_all(new_path.as_path().parent().unwrap().clone()).unwrap();

		let mut new_file = File::create(new_path).unwrap();
		new_file.write(txt.as_ref()).expect("save utf8 fail");
	}

	fn read_file(&self, name: &str) -> Option<Vec<u8>> {
		let p = Path::new(name);
		match File::open(p) {
		    Ok(mut f) => {
		    	let mut buf: Vec<u8> = Vec::new();
				f.read_to_end(&mut buf).unwrap();
				if buf.len() < 2 {
					println!("invalid rc file");
					None
				}else{
					Some(buf)
				}
		    },
		    Err(e) => {
		    	println!("open file err:{:?}",e);
		    	None
		    },
		}
	}

	pub fn parse_rc(&self, name: &str) ->(RcRoot,Vec<String>) {
		let buf: Vec<u8>;
		if let Some(b) = self.read_file(name) {
			buf = b;
		}else{
			return (RcRoot::new(),Vec::new());
		}

		let txt = self.decode(buf);
		
		let ret = self.extract_rc(&txt);

		//replace resource.h to ui.h in rc file
		let new_txt = self.replace_header(&txt);
		self.save_as_utf8(name, "utf8\\ui.rc", &new_txt);
		ret
	}

	pub fn parse_header(&self, name: &str) -> BTreeMap<String,WORD> {
		let buf: Vec<u8> = self.read_file(name).expect("resource.h should exist");

		let txt = self.decode(buf);
		let ret = self.extract_header(&txt);
		self.save_as_utf8(name, "utf8\\ui.h", &txt);
		ret
	}

	fn extract_rc(&self, txt: &String)->(RcRoot,Vec<String>) {
		let mut r = RcRoot::new();
		let re_begin = Regex::new(r"\w+\s+DIALOGEX").unwrap();
		let re_end = Regex::new(r"\sEND\s").unwrap();
		let mut all_dlg_ids = Vec::<String>::new();
		for pos in re_begin.find_iter(txt) {
			let (start,_) = pos;

			let s = &txt[start..];
			let dlg_id = s.split_whitespace().next().unwrap();

			if let Some(end_pos) = re_end.find(s) {
				let block_end = end_pos.0 + 4;
				let dlg_block = &s[..block_end];
				//println!("{}", dlg_block);
				//println!("block end:{}\n", block_end + start);
				r.parse_dialog(dlg_id,dlg_block);
				all_dlg_ids.push(dlg_id.to_string());
			}else{
				println!("not block end?");
			}
		}
		(r,all_dlg_ids)
	}

	//#define IDC_LST_ALL_DLGS                1001
	fn extract_header(&self, txt: &String) -> BTreeMap<String,WORD> {
		let mut consts: BTreeMap<String,WORD> = BTreeMap::new();
		let re = Regex::new(r"#define\s+(\w+)\s+(\d+)").unwrap();
		for cap in re.captures_iter(txt) {
			//println!("pub const {}: WORD = {};", cap.at(1).unwrap_or(""),cap.at(2).unwrap_or(""));
			let id = cap.at(1).unwrap_or("").to_string();
			if id.starts_with("_APS_NEXT") {
				continue;
			}
			consts.insert(id, cap.at(2).unwrap_or("0").parse::<WORD>().unwrap());
		}
		consts
	}
	//comments and strings must be deleted first,especially strings.they may contain key words like:BEGIN,END
	// fn replace_string(&self,txt: &mut String) {

	// }

	// fn delete_comments(&self,txt: &mut String) {

	// }
}