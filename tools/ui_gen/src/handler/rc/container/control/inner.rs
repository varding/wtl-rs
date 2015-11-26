


#[derive(Debug)]
pub struct Inner {
	pub id : String,
    //class: String,
    style: Vec<String>,
}

impl Inner {
	//CONTROL         "work",IDC_RADIO_WORK,"Button",BS_AUTORADIOBUTTON,17,188,32,10
	// pub fn new_control(line: &str)->Inner{
	// 	let l = line.trim();
	// 	let v:Vec<&str> = l.split(',').collect();
	// 	// for token in &v {
	// 	// 	println!("{}", token);
	// 	// }
	// 	assert!(v.len() >= 4);
	// 	Inner{
	// 		id: v[1].to_string(),
	// 		class: v[2].replace("\"","").trim().to_string(),
	// 		style: v[3].split("|").map(|s|s.trim().to_string()).collect(),
	// 	}
	// }

	////////////////////
	// common controls
	//CONTROL         "work",IDC_RADIO_WORK,"Button",BS_AUTORADIOBUTTON,17,188,32,10
	pub fn new_radio_button(id: &str, style: &Vec<String>)->Inner{
		Inner{
			id: id.to_string(),
			//class: "Radio".to_string(),
			style: style.clone(),
		}	
	}

	//CONTROL         "",IDC_TREE_SELECTED_DLGS,"SysTreeView32",TVS_HASBUTTONS | TVS_HASLINES | TVS_LINESATROOT | TVS_SHOWSELALWAYS | WS_BORDER | WS_HSCROLL | WS_TABSTOP,202,19,154,112
	pub fn new_tree_view(id: &str, style: &Vec<String>)->Inner {
		Inner{
			id: id.to_string(),
			style: style.clone(),
		}
	}

	/////////////////////////////////////
	//named controls
	//PUSHBUTTON      "open",IDC_BTN_OPEN_UDP_PORT,91,16,50,14
	pub fn new_push_button(line: &str)->Inner{
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 2);
		Inner{
			id: v[1].to_string(),
			//class: "PUSHBUTTON".to_string(),
			style: Vec::new(),
		}
	}

	//LISTBOX         IDC_LST_ALL_DLGS,7,19,142,112,LBS_NOINTEGRALHEIGHT | WS_VSCROLL | WS_TABSTOP
	pub fn new_list_box(line: &str)->Inner {
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 6);
		Inner{
			id: v[0].to_string(),
			//class: "PUSHBUTTON".to_string(),
			style: v[5].split("|").map(|s|s.trim().to_string()).collect(),
		}
	}

	//COMBOBOX        IDC_COMBO_NODE_ID,49,82,48,353,CBS_DROPDOWN | CBS_SORT | WS_VSCROLL | WS_TABSTOP
	pub fn new_combobox(line: &str)->Inner{
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 6);
		Inner{
			id: v[0].to_string(),
			//class: "COMBOBOX".to_string(),
			style: v[5].split("|").map(|s|s.trim().to_string()).collect(),
		}
	}

	//EDITTEXT        IDC_EDT_UDP_PORT,48,16,40,14,ES_AUTOHSCROLL | ES_NUMBER
	pub fn new_edit_text(line: &str)->Inner{
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 6);
		Inner{
			id: v[0].to_string(),
			//class: "EDITTEXT".to_string(),
			style: v[5].split("|").map(|s|s.trim().to_string()).collect(),
		}
	}

	//LTEXT           "port",IDC_STATIC,21,67,25,8
	pub fn new_ltext(line: &str)->Inner{
		let l = line.trim();
		//FIXME: string may contain comma,so this should be improved
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 2);
		Inner{
			id: v[1].to_string(),
			//class: "LTEXT".to_string(),
			style: Vec::new(),
		}
	}

	//GROUPBOX        "config",IDC_STATIC,7,7,158,28
	pub fn new_group_box(line: &str)->Inner{
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 2);
		Inner{
			id: v[1].to_string(),
			//class: "GROUPBOX".to_string(),
			style: Vec::new(),
		}
	}

	// fn split_by_comma(&self,data: &str)->Vec<&str> {
	// 	let mut state = 0;		//0=>normal,1=>string
	// 	let mut start = 0;
	// 	let mut end = 0;
	// 	let v: Vec<&str> = Vec::new();
	// 	for d in data {
	// 		if state == 0 {
	// 			if d == '\"' {
	// 				state = 1;
	// 			}
	// 		}else {
	// 			if d == '\"' {
	// 				state = 0;
	// 			}else if d == ',' {

	// 			}
	// 		}
	// 		end += 1;
	// 	}
	// }
}