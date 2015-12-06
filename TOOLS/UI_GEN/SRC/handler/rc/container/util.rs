
use std::fs;
use std::path::PathBuf;

//convert IDC_RADIO_WORK to radio_work
// pub fn ctrl_id_to_name(id: &str)->String {
// 	if id.starts_with("IDC_") {
// 		id[4..].to_lowercase()
// 	}else{
// 		id.to_lowercase()
// 	}
// }


// pub fn dlg_id_to_name(id: &str)->String {
// 	if id.starts_with("IDD_") {
// 		id[4..].to_lowercase()
// 	}else{
// 		id.to_lowercase()
// 	}
// }

pub fn mkdir(p: &PathBuf){
    fs::create_dir_all(p.as_path().clone()).expect("create dir fail");
}