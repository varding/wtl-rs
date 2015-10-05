
#[derive(Debug)]
struct Ctrl {
	id : String,
    //class: String,
    style: Vec<String>,
}

impl Ctrl {
	//CONTROL         "work",IDC_RADIO_WORK,"Button",BS_AUTORADIOBUTTON,17,188,32,10
	// pub fn new_control(line: &str)->Ctrl{
	// 	let l = line.trim();
	// 	let v:Vec<&str> = l.split(',').collect();
	// 	// for token in &v {
	// 	// 	println!("{}", token);
	// 	// }
	// 	assert!(v.len() >= 4);
	// 	Ctrl{
	// 		id: v[1].to_string(),
	// 		class: v[2].replace("\"","").trim().to_string(),
	// 		style: v[3].split("|").map(|s|s.trim().to_string()).collect(),
	// 	}
	// }

	fn new_radio_button(id: &str, style: &Vec<String>)->Ctrl{
		Ctrl{
			id: id.to_string(),
			//class: "Radio".to_string(),
			style: style.clone(),
		}	
	}

	//PUSHBUTTON      "open",IDC_BTN_OPEN_UDP_PORT,91,16,50,14
	fn new_push_button(line: &str)->Ctrl{
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 2);
		Ctrl{
			id: v[1].to_string(),
			//class: "PUSHBUTTON".to_string(),
			style: Vec::new(),
		}
	}

	//COMBOBOX        IDC_COMBO_NODE_ID,49,82,48,353,CBS_DROPDOWN | CBS_SORT | WS_VSCROLL | WS_TABSTOP
	fn new_combobox(line: &str)->Ctrl{
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 6);
		Ctrl{
			id: v[0].to_string(),
			//class: "COMBOBOX".to_string(),
			style: v[5].split("|").map(|s|s.trim().to_string()).collect(),
		}
	}

	//EDITTEXT        IDC_EDT_UDP_PORT,48,16,40,14,ES_AUTOHSCROLL | ES_NUMBER
	fn new_edit_text(line: &str)->Ctrl{
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 6);
		Ctrl{
			id: v[0].to_string(),
			//class: "EDITTEXT".to_string(),
			style: v[5].split("|").map(|s|s.trim().to_string()).collect(),
		}
	}

	//LTEXT           "port",IDC_STATIC,21,67,25,8
	fn new_ltext(line: &str)->Ctrl{
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 2);
		Ctrl{
			id: v[1].to_string(),
			//class: "LTEXT".to_string(),
			style: Vec::new(),
		}
	}

	//GROUPBOX        "config",IDC_STATIC,7,7,158,28
	fn new_group_box(line: &str)->Ctrl{
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 2);
		Ctrl{
			id: v[1].to_string(),
			//class: "GROUPBOX".to_string(),
			style: Vec::new(),
		}
	}
}

#[derive(Debug)]
pub enum Control {
    RadioBtn(Ctrl),
    Button(Ctrl),
    ComboBox(Ctrl),
    Edit(Ctrl),
    Static(Ctrl),
    GroupBox(Ctrl),
    ListBox(Ctrl),
    ListView(Ctrl),
    TreeView(Ctrl),
    UnKnow(String),
}

fn take_rest<'a>(start: &str, line: &'a str)->&'a str{
	&line[start.len()..]
}

impl Control {
	pub fn new(line: &str)->Control{
		if line.starts_with("CONTROL") {
			Self::parse_control(take_rest("CONTROL",line))
		}else if line.starts_with("PUSHBUTTON") {
			Control::Button(Ctrl::new_push_button(take_rest("PUSHBUTTON", line)))
		}else if line.starts_with("COMBOBOX") {
			Control::ComboBox(Ctrl::new_combobox(take_rest("COMBOBOX", line)))
		}else if line.starts_with("EDITTEXT") {
			Control::Edit(Ctrl::new_edit_text(take_rest("EDITTEXT", line)))
		}else if line.starts_with("LTEXT") {
			Control::Static(Ctrl::new_ltext(take_rest("LTEXT", line)))
		}else if line.starts_with("GROUPBOX") {
			Control::GroupBox(Ctrl::new_group_box(take_rest("GROUPBOX", line)))
		}else{
			Control::UnKnow(line.to_string())
		}
	}

	//CONTROL         "work",IDC_RADIO_WORK,"Button",BS_AUTORADIOBUTTON,17,188,32,10
	fn parse_control(data: &str)->Control{
		let l = data.trim();
		let v:Vec<&str> = l.split(',').collect();
		// for token in &v {
		// 	println!("{}", token);
		// }
		assert!(v.len() >= 4);
		let style = v[3].split("|").map(|s|s.trim().to_string()).collect();

		for s in &style {
			if s == "BS_AUTORADIOBUTTON"{
				
				return Control::RadioBtn(Ctrl::new_radio_button(v[1], &style));
			}
		}
		Control::UnKnow(data.to_string())
	}


	pub fn write(&self)->String{
		match *self {
			Control::Button(ref c)=>{
				format!("{}: Button,",convert_id(&c.id[..]))
			}
			Control::Edit(ref c)=>{
				format!("{}: Edit,",convert_id(&c.id[..]))
			}
			Control::TreeView(ref c)=>{
				format!("{}: TreeViewEx,",convert_id(&c.id[..]))
			}
			Control::ComboBox(ref c)=>{
				format!("{}: ComboBox,",convert_id(&c.id[..]))
			}
			_=>{
				String::new()
			}
		}
	}
}


fn convert_id(id: &str)->String {
	if id.starts_with("IDC_") {
		id[4..].to_lowercase()
	}else{
		id.to_lowercase()
	}
}