
use std::fs::File;
use std::io::{Read,Write};
use std::path::{Path,PathBuf};
//use std::io::Result;
use std::slice;
use std::ffi::OsStr;

use regex::Regex;

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
		let file_stem = p.file_stem().unwrap();
		let file_ext = p.extension().unwrap();
		
		let mut new_path = PathBuf::from(p.parent().unwrap());
		//println!("{:?}", new_path);
		//new_path.push("utf8");
		new_path.push(new_name);
		//println!("{:?}", new_path);

		let mut new_file = File::create(new_path).unwrap();
		new_file.write(txt.as_ref());
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

	pub fn parse_rc(&self, name: &str) {
		let buf: Vec<u8>;
		if let Some(b) = self.read_file(name) {
			buf = b;
		}else{
			return;
		}

		let txt = self.decode(buf);
		
		self.extract_rc(&txt);

		//replace resource.h to ui.h in rc file
		let new_txt = self.replace_header(&txt);
		self.save_as_utf8(name, "utf8\\ui.rc", &new_txt);
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

	pub fn extract_rc(&self, txt: &String){
		let re = Regex::new(r"(\w+) DIALOGEX").unwrap();
		//let text = "2012-03-14, 2013-01-01 and 2014-07-05";
		// for cap in re.captures_iter(txt) {
		//     println!("name:{}", cap.at(1).unwrap_or(""));
		// }
		// let i = (&txt[..]).match_indices(&re).collect::<Vec<_>>();

		// println!("{:?}", i);

		for pos in re.find_iter(txt) {
			//
			let (start,end) = pos;
			println!("new block, from: {} to: {}", start,end);

			let s = &txt[start..];
			if let Some(block_end) =  s.find("END") {
				println!("{}", &s[..block_end+3]);
				println!("block end:{}\n", block_end + start);
			}else{
				println!("not block end?");
			}
		}
	}
}

// fn load_header(name: &str) {
// 	let p = Path::new(name);
// 	let f = File::open(p);
// 	if let Ok(mut f1) = f {
// 		let mut buf: Vec<u8> = Vec::new();
// 		f1.read_to_end(&mut buf).unwrap();
// 		if buf.len() < 2 {
// 			println!("invalid rc file");
// 			return;
// 		}
// 		println!("BOM:{:?}", &buf[0..2]);
// 		if buf[0..2] == [0xFE,0xFF] {
// 			println!("big endian ucs-2");
// 		}else if buf[0..2] == [0xFF,0xFE] {
// 			println!("encoding:little endian ucs-2");
// 			let mut it = buf.into_iter();
// 			//delete bom
// 			it.next();it.next();
// 			let rc_buf: Vec<u8> = it.collect();
// 			let ptr = rc_buf.as_ptr() as *const u16;
// 			let s = unsafe{slice::from_raw_parts(ptr, rc_buf.len()/2)};
// 			let txt = String::from_utf16_lossy(s);

// 			let file_stem = p.file_stem().unwrap();
// 			let file_ext = p.extension().unwrap();
// 			//let tmp_path = p.parent().unwrap();
			
// 			let mut tmp_path = PathBuf::from(p.parent().unwrap());
// 			tmp_path.push("utf8");
// 			//tmp_path.set_file_name("ui.h");
// 			tmp_path.push("ui.h");
// 			println!("{:?}", tmp_path);

// 			let mut tmp_file = File::create(tmp_path).unwrap();
// 			tmp_file.write(txt.as_ref());
// 			//println!("{}",txt);
// 		}
// 	}
// }




    // println!("Hello, world!");
    // load_rc("K:\\software\\pc\\rust\\wtl-rs\\tools\\ui_gen\\src\\design\\design.rc","ui.rc");
    // //load_rc("K:\\software\\pc\\rust\\wtl-rs\\tools\\ui_gen\\src\\ui_gen\\mhc2.rc","mhc.rc");
    // //load_rc("K:\\software\\pc\\rust\\wtl-rs\\tools\\ui_gen\\src\\ui_gen\\z.rc","z.rc");
    
    // load_header("K:\\software\\pc\\rust\\wtl-rs\\tools\\ui_gen\\src\\design\\resource.h");