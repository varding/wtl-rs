
use super::Name;

pub enum CtrlType {
	RadioBtn,
    Button,
    ComboBox,
    Edit,
    Static,
    GroupBox,
    ListBox,
    ListView,
    TreeView,
    TabView,
    UnKnow,
}

pub struct Control {
	pub tp: CtrlType,
	name: Name,
}

impl Control {
	pub fn parse(line: &str)->Option<Control> {
		let parts: Vec<&str> = line.trim().split_whitespace().collect();

		if parts.len() < 2 {
		    panic!("splite line error:{}",line);
		}

		let rst = &(parts[1..].join(" "));
		match parts[0] {
			"PUSHBUTTON"=>Some(Self::new_push_button(rst)),
			"LISTBOX"=>Some(Self::new_list_box(rst)),
			"COMBOBOX"=>Some(Self::new_combobox(rst)),
			"EDITTEXT"=>Some(Self::new_edit_text(rst)),
			"LTEXT"=>Some(Self::new_ltext(rst)),
			"GROUPBOX"=>Some(Self::new_group_box(rst)),
			"CONTROL"=>Self::parse_control(rst),
			_=>None,
		}
	}
}

impl Control {
    fn new_radio_button(id: &str, style: &Vec<String>)->Control {
		//(id.to_string(), style.clone())
		Control{
			tp: CtrlType::Button,
			name: Name::radio_btn(id),
		}
	}

	//CONTROL         "",IDC_TREE_SELECTED_DLGS,"SysTreeView32",TVS_HASBUTTONS | TVS_HASLINES | TVS_LINESATROOT | TVS_SHOWSELALWAYS | WS_BORDER | WS_HSCROLL | WS_TABSTOP,202,19,154,112
	fn new_tree_view(id: &str, style: &Vec<String>)->Control {
		//(id.to_string(), style.clone())
		Control{
			tp: CtrlType::TreeView,
			name: Name::tree_view(id),
		}
	}

	//CONTROL         "",IDC_MAIN_TAB,"SysTabControl32",WS_TABSTOP,7,7,498,226
	fn new_tab_view(id: &str, style: &Vec<String>)->Control {
		//(id.to_string(), style.clone())
		Control{
			tp: CtrlType::TabView,
			name: Name::tab_view(id),
		}
	}
	/////////////////////////////////////
	//named controls
	//PUSHBUTTON      "open",IDC_BTN_OPEN_UDP_PORT,91,16,50,14
	fn new_push_button(line: &str)->Control {
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 2);
		//(v[1].to_string(), Vec::new())
		Control{
			tp: CtrlType::Button,
			name: Name::button(v[1]),
		}
	}

	//LISTBOX         IDC_LST_ALL_DLGS,7,19,142,112,LBS_NOINTEGRALHEIGHT | WS_VSCROLL | WS_TABSTOP
	fn new_list_box(line: &str)->Control {
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 6);
		//(v[0].to_string(), v[5].split("|").map(|s|s.trim().to_string()).collect())
		Control{
			tp: CtrlType::ListBox,
			name: Name::list_box(v[0]),
		}
	}

	//COMBOBOX        IDC_COMBO_NODE_ID,49,82,48,353,CBS_DROPDOWN | CBS_SORT | WS_VSCROLL | WS_TABSTOP
	fn new_combobox(line: &str)->Control {
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 6);
		//(v[0].to_string(), v[5].split("|").map(|s|s.trim().to_string()).collect())
		Control{
			tp: CtrlType::ComboBox,
			name: Name::combox(v[0]),
		}
	}

	//EDITTEXT        IDC_EDT_UDP_PORT,48,16,40,14,ES_AUTOHSCROLL | ES_NUMBER
	fn new_edit_text(line: &str)->Control {
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 6);
		//(v[0].to_string(), v[5].split("|").map(|s|s.trim().to_string()).collect())
		Control{
			tp: CtrlType::Edit,
			name: Name::edit(v[0]),
		}
	}

	//LTEXT           "port",IDC_STATIC,21,67,25,8
	fn new_ltext(line: &str)->Control {
		let l = line.trim();
		//FIXME: string may contain comma,so this should be improved
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 2);
		//(v[1].to_string(), Vec::new())
		
		Control{
			tp: CtrlType::Static,
			name: Name::ltext(v[1]),
		}
	}

	//GROUPBOX        "config",IDC_STATIC,7,7,158,28
	fn new_group_box(line: &str)->Control {
		let l = line.trim();
		let v:Vec<&str> = l.split(',').collect();
		assert!(v.len() >= 2);
		//(v[1].to_string(), Vec::new())
		Control{
			tp: CtrlType::GroupBox,
			name: Name::ltext(v[1]),
		}
	}

		//CONTROL         "work",IDC_RADIO_WORK,"Button",BS_AUTORADIOBUTTON,17,188,32,10
	//CONTROL         "",IDC_TREE_SELECTED_DLGS,"SysTreeView32",TVS_HASBUTTONS | TVS_HASLINES | TVS_LINESATROOT | TVS_SHOWSELALWAYS | WS_BORDER | WS_HSCROLL | WS_TABSTOP,202,19,154,112
	fn parse_control(data: &str)->Option<Control>{
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
						return Some(Self::new_radio_button(v[1],&style))
					}
				}
				None
			}
			"SysTreeView32"=>{
				println!("tree view");
				Some(Self::new_tree_view(v[1],&style))
			}
			"SysTabControl32"=>{
				println!("tab view");
				Some(Self::new_tab_view(v[1],&style))
			}
			_=>{
				None
			}
		}
	}
}

impl Control {
	/// write file
    pub fn name_for_file(&self)->Option<&Name> {
		match self.tp {
			CtrlType::Button|CtrlType::ListBox|CtrlType::ComboBox|CtrlType::Edit|CtrlType::TreeView=>{
				Some(&self.name)
			},
			CtrlType::Static|CtrlType::GroupBox=>{
				//only id!=static can be display
				if self.name.var_name != "static" {
					//writeln!(f,"\tpub {}: CStatic,",id).unwrap();
					Some(&self.name)
				}else{
					None
				}
			},
			_=>{
				None
			},
		}
	}

	pub fn get_id(&self)->&str {
		&self.name.id
	}
}