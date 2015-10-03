use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    //link_res("./src/res/ui.rc");
    link_res("./src/design/utf8/ui.rc");
}

fn link_res(rc_path:&str){
  let p = Path::new(&rc_path);
  let rc_name = p.file_name().expect("path error").to_str().expect("parse error");
  let out_dir = env::var("OUT_DIR").ok().expect("can't find out_dir");

  let obj_path = &format!("{}/{}.rc.o", out_dir,rc_name);

  let lib_path = &format!("{}/lib{}.a",out_dir,rc_name);

  Command::new("windres").args(&[&*rc_path,  "-o"])
                        .arg(obj_path)
                        .status().unwrap();

  Command::new("ar").args(&["crus", lib_path, obj_path])
                    .current_dir(&Path::new(&out_dir))
                    .status().unwrap();

  println!("cargo:rustc-link-search=native={}", out_dir);
  println!("cargo:rustc-link-lib=static={}",rc_name);
}