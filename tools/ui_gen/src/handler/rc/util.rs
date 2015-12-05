



//convert IDC_RADIO_WORK to radio_work
pub fn ctrl_id_to_name(id: &str)->String {
	if id.starts_with("IDC_") {
		id[4..].to_lowercase()
	}else{
		id.to_lowercase()
	}
}


pub fn dlg_id_to_name(id: &str)->String {
	if id.starts_with("IDD_") {
		id[4..].to_lowercase()
	}else{
		id.to_lowercase()
	}
}

pub fn id_to_name(id: &str)->String {
	if id.starts_with("IDD_") || id.starts_with("IDC_") {
		id[4..].to_lowercase()
	}else{
		id.to_lowercase()
	}
}

//from librustc
pub fn to_camel_case(s: &str) -> String {
    s.split('_').flat_map(|word| word.chars().enumerate().map(|(i, c)|
        if i == 0 {
            c.to_uppercase().collect::<String>()
        } else {
            c.to_lowercase().collect()
        }
    )).collect::<Vec<_>>().concat()
}