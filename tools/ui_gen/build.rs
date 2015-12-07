use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    link_res_msvc();
    //link_res("./src/design/utf8/ui.rc");
}

fn link_res(rc_path:&str){
    let p = Path::new(&rc_path);
    let rc_name = p.file_name().expect("path error").to_str().expect("parse error");
    let out_dir = env::var("OUT_DIR").ok().expect("can't find out_dir");

    let obj_path = &format!("{}/{}.rc.o", out_dir,rc_name);

    let lib_path = &format!("{}/lib{}.a",out_dir,rc_name);

    // Command::new("windres").arg("-F").arg("pe-x86-64")
    //             .arg(obj_path)
    //             .status().unwrap();

    Command::new("windres").arg("-F").arg("pe-x86-64")
                          .args(&[&*rc_path,  "-o"])
                          .arg(obj_path)
                          .status().unwrap();

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
    // Command::new("Rc").arg("-Fo").arg("./src/design/design.lib")
    //                   //.args(&[&*rc_path,  "-o"])
    //                   .arg("./src/design/design.rc")
    //                   .status().unwrap();

    println!("cargo:rustc-link-lib=static={}","./src/design/design");  
}