use std::fs::File;
use std::io::Write;
use super::util::*;
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

	////////////////////
	// common controls
	//CONTROL         "work",IDC_RADIO_WORK,"Button",BS_AUTORADIOBUTTON,17,188,32,10
	fn new_radio_button(id: &str, style: &Vec<String>)->Ctrl{
		Ctrl{
			id: id.to_string(),
			//class: "Radio".to_string(),
			style: style.clone(),
		}	
	}

	//CONTROL         "",IDC_TREE_SELECTED_DLGS,"SysTreeView32",TVS_HASBUTTONS | TVS_HASLINES | TVS_LINESATROOT | TVS_SHOWSELALWAYS | WS_BORDER | WS_HSCROLL | WS_TABSTOP,202,19,154,112
	fn new_tree_view(id: &str, style: &Vec<String>)->Ctrl {
		Ctrl{
			id: id.to_string(),
			style: style.clone(),
		}
	}

	/////////////////////////////////////
	//named controls
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

	//LISTBOX         IDC_LST_ALL_DLGS,7,19,142,112,LBS_NOINTEGRALHEIGHT | WS_VSCROLL | WS_TABSTOP
	fn new_list_box(line: &str)->Ctrl {
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 6);
		Ctrl{
			id: v[0].to_string(),
			//class: "PUSHBUTTON".to_string(),
			style: v[5].split("|").map(|s|s.trim().to_string()).collect(),
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
		//FIXME: string may contain comma,so this should be improved
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
		}else if line.starts_with("LISTBOX") {
			Control::ListBox(Ctrl::new_list_box(take_rest("LISTBOX", line)))
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
	//CONTROL         "",IDC_TREE_SELECTED_DLGS,"SysTreeView32",TVS_HASBUTTONS | TVS_HASLINES | TVS_LINESATROOT | TVS_SHOWSELALWAYS | WS_BORDER | WS_HSCROLL | WS_TABSTOP,202,19,154,112
	fn parse_control(data: &str)->Control{
		let l = data.trim();
		let v:Vec<&str> = l.split(',').collect();
		// for token in &v {
		// 	println!("{}", token);
		// }
		assert!(v.len() >= 4);
		let style = v[3].split("|").map(|s|s.trim().to_string()).collect();

		let class_name = v[2].replace("\"","");
		//class
		match class_name.trim() {
			"Button"=>{
				for s in &style {
					if s == "BS_AUTORADIOBUTTON"{
						return Control::RadioBtn(Ctrl::new_radio_button(v[1], &style));
					}
				}
				return Control::UnKnow(data.to_string());
			}
			"SysTreeView32"=>{
				println!("tree view");
				return Control::TreeView(Ctrl::new_tree_view(v[1], &style));
			}
			_=>{
				return Control::UnKnow(data.to_string());
			}
		}
	}
}

impl Control {
	pub fn write_declaration(&self,f: &mut File){
		match *self {
			Control::Button(ref c)=>{
				writeln!(f,"\tpub {}: CButton,",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::ListBox(ref c)=>{
				writeln!(f,"\tpub {}: CListBox,",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::ComboBox(ref c)=>{
				writeln!(f,"\tpub {}: CComboBox,",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::Edit(ref c)=>{
				writeln!(f,"\tpub {}: CEdit,",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::TreeView(ref c)=>{
				writeln!(f,"\tpub {}: CTreeViewCtrlEx,",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::Static(ref c)=>{
				//only id!=static can be display
				let id = ctrl_id_to_name(&c.id[..]);
				if id != "static" {
					writeln!(f,"\tpub {}: CStatic,",id).unwrap();
				}
			}
			Control::GroupBox(ref c)=>{
				//only id!=static can be display
				let id = ctrl_id_to_name(&c.id[..]);
				if id != "static" {
					writeln!(f,"\tpub {}: CGroupBox,",id).unwrap();
				}
			}
			Control::UnKnow(ref s)=>{
				writeln!(f,"\t// {}: UnKnow,",s).unwrap();
			}
			_=>{
				//String::new()
				//writeln!(f,"// pub {}")
			}
		}
	}

	pub fn write_new(&self,f: &mut File){
		match *self {
			Control::Button(ref c)=>{
				writeln!(f,"\t\t\t{}: CButton::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::ListBox(ref c)=>{
				writeln!(f,"\t\t\t{}: CListBox::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::ComboBox(ref c)=>{
				writeln!(f,"\t\t\t{}: CComboBox::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::Edit(ref c)=>{
				writeln!(f,"\t\t\t{}: CEdit::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::TreeView(ref c)=>{
				writeln!(f,"\t\t\t{}: CTreeViewCtrlEx::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::Static(ref c)=>{
				//only id!=static can be display
				let id = ctrl_id_to_name(&c.id[..]);
				if id != "static" {
					writeln!(f,"\t\t\t{}: CStatic::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
				}
			}
			Control::GroupBox(ref c)=>{
				//only id!=static can be display
				let id = ctrl_id_to_name(&c.id[..]);
				if id != "static" {
					writeln!(f,"\t\t\t{}: CGroupBox::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
				}
			}
			Control::UnKnow(..)=>{
				//writeln!(f,"\t\t\t// {}: UnKnow,",s).unwrap();
			}
			_=>{
				//String::new()
				//writeln!(f,"// pub {}")
			}
		}
	}

	pub fn write_msg(&self,f: &mut File){
		match *self {
			Control::Button(ref c)=>{
				writeln!(f,"\tpub fn {}_msg(&mut self)->BtnMsg<T> {{",ctrl_id_to_name(&c.id[..])).unwrap();
				writeln!(f,"\t\tself.this.btn_handler({})",c.id).unwrap();
				writeln!(f,"\t}}").unwrap();
				//writeln!(f,"\t\t\t{}: Button::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::ListBox(ref c)=>{
				//writeln!(f,"\t\t\t{}: ListBox::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::ComboBox(ref c)=>{
				//writeln!(f,"\t\t\t{}: ComboBox::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::Edit(ref c)=>{
				//writeln!(f,"\t\t\t{}: Edit::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::TreeView(ref c)=>{
				//writeln!(f,"\t\t\t{}: TreeViewEx::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
			}
			Control::Static(ref c)=>{
				//only id!=static can be display
				// let id = ctrl_id_to_name(&c.id[..]);
				// if id != "static" {
				// 	writeln!(f,"\tpub {}: Static::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
				// }
			}
			Control::GroupBox(ref c)=>{
				//only id!=static can be display
				// let id = ctrl_id_to_name(&c.id[..]);
				// if id != "static" {
				// 	writeln!(f,"\t\t\t{}: GroupBox::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
				// }
			}
			Control::UnKnow(..)=>{
				//writeln!(f,"\t\t\t// {}: UnKnow,",s).unwrap();
			}
			_=>{
				//String::new()
				//writeln!(f,"// pub {}")
			}
		}
	}
}

