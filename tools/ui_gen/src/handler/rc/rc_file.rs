
use std::fs::File;
use std::io::{Read,Write};
use std::path::{Path,PathBuf};
use std::slice;
//use std::ffi::OsStr;

use regex::Regex;

use super::Root;
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

		let mut new_file = File::create(new_path).unwrap();
		new_file.write(txt.as_ref()).expect("save utf8 fail");
	}

	fn read_file(&self, name: &str) -> Option<Vec<u8>> {
		let p = Path::new(name);
		if let Ok(mut f) = File::open(p) {
			let mut buf: Vec<u8> = Vec::new();
			f.read_to_end(&mut buf).unwrap();
			if buf.len() < 2 {
				println!("invalid rc file");
				None
			}else{
				Some(buf)
			}
		}else{
			None
		}
	}

	pub fn parse_rc(&self, name: &str) ->Vec<String> {
		let buf: Vec<u8>;
		if let Some(b) = self.read_file(name) {
			buf = b;
		}else{
			return Vec::new();
		}

		let txt = self.decode(buf);
		
		let ret = self.extract_rc(&txt);

		//replace resource.h to ui.h in rc file
		let new_txt = self.replace_header(&txt);
		self.save_as_utf8(name, "utf8\\ui.rc", &new_txt);
		ret
	}

	pub fn parse_header(&self, name: &str) {
		let buf: Vec<u8>;
		if let Some(b) = self.read_file(name) {
			buf = b;
		}else{
			return;
		}

		let txt = self.decode(buf);
		self.save_as_utf8(name, "utf8\\ui.h", &txt);
	}

	fn extract_rc(&self, txt: &String)->Vec<String> {
		let mut dlg_ids: Vec<String> = Vec::new();
		let mut r = Root::new();
		//let re_id = Regex::new(r"\w+\s+DIALOGEX").unwrap();
		let re_begin = Regex::new(r"\w+\s+DIALOGEX").unwrap();
		let re_end = Regex::new(r"\sEND\s").unwrap();
		for pos in re_begin.find_iter(txt) {
			let (start,_) = pos;
			//println!("new block, from: {} to: {}", start,end);

			let s = &txt[start..];
			dlg_ids.push(s.split_whitespace().next().unwrap().to_string());

			if let Some(end_pos) = re_end.find(s) {
				let block_end = end_pos.0 + 4;
				let dlg_block = &s[..block_end];
				//println!("{}", dlg_block);
				//println!("block end:{}\n", block_end + start);
				r.parse_dialog(dlg_block);
			}else{
				println!("not block end?");
			}
		}
		dlg_ids
	}

	//comments and strings must be deleted first,especially strings.they may contain key words like:BEGIN,END
	// fn replace_string(&self,txt: &mut String) {

	// }

	// fn delete_comments(&self,txt: &mut String) {

	// }
}