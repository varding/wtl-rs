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

    //check if rustc is x86
    //let c = 
    // let rustc_out = Command::new("rustc").arg("-vV").output().unwrap().stdout;
    // let rustc_str = String::from_utf8_lossy(&rustc_out);
    // let lines: Vec<&str> = rustc_str.lines().collect();

    // let is_x64_rustc = false;
    // for l in lines {
    //   if l.starts_with("host") {
    //     if let Some(p) = l.find("x86_64") {
    //       is_x64_rustc = true;
    //       panic!("x64");
    //     }
    //   }
    // }
    
    //check if built by appveyor
    // if let Ok(a) = env::var("appveyor") {
    //   panic!("in appveyor");
    // }else{

    // }
    //panic!("{:?}", env::var("mytarget"));
    let obj_path = &format!("{}/{}.rc.o", out_dir,rc_name);

    let lib_path = &format!("{}/lib{}.a",out_dir,rc_name);

    // if is_x64_rustc {

    // }
    Command::new("windres").arg("-F").arg("pe-x86-64")
                .arg(obj_path)
                .status().unwrap();

    // Command::new("windres").args(&[&*rc_path,  "-o"])
    //                       .arg(obj_path)
    //                       .status().unwrap();

    Command::new("ar").args(&["crus", lib_path, obj_path])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static={}",rc_name);
}

/*
link for msvc
1. use msvc tools->cmd,cd to the design directory(where .rc file was located)
2. use RC command to build .rc file: RC /fo design.lib design.rc
3. call link_res_msvc in build.rs to link design.lib
*/
fn link_res_msvc() {
    //don't add .lib extension
    println!("cargo:rustc-link-lib=static={}","./src/design/design");  
}