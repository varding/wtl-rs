use std::fs::File;
use std::io::Write;
use handler::rc::util::*;
use super::inner::Inner;

#[derive(Debug)]
pub enum Control {
    RadioBtn(Inner),
    Button(Inner),
    ComboBox(Inner),
    Edit(Inner),
    Static(Inner),
    GroupBox(Inner),
    ListBox(Inner),
    ListView(Inner),
    TreeView(Inner),
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
			Control::Button(Inner::new_push_button(take_rest("PUSHBUTTON", line)))
		}else if line.starts_with("LISTBOX") {
			Control::ListBox(Inner::new_list_box(take_rest("LISTBOX", line)))
		}else if line.starts_with("COMBOBOX") {
			Control::ComboBox(Inner::new_combobox(take_rest("COMBOBOX", line)))
		}else if line.starts_with("EDITTEXT") {
			Control::Edit(Inner::new_edit_text(take_rest("EDITTEXT", line)))
		}else if line.starts_with("LTEXT") {
			Control::Static(Inner::new_ltext(take_rest("LTEXT", line)))
		}else if line.starts_with("GROUPBOX") {
			Control::GroupBox(Inner::new_group_box(take_rest("GROUPBOX", line)))
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
						return Control::RadioBtn(Inner::new_radio_button(v[1], &style));
					}
				}
				return Control::UnKnow(data.to_string());
			}
			"SysTreeView32"=>{
				println!("tree view");
				return Control::TreeView(Inner::new_tree_view(v[1], &style));
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

	/// dialog path in the tree
	pub fn write_binding(&self,path: &String,f: &mut File){
		match *self {
			Control::Button(ref c)=>{
				let id = ctrl_id_to_name(&c.id[..]);
				writeln!(f,"\t\tt.{}.{}.Attach(this.GetDlgItem({}));",path,id,c.id).unwrap();
			}
			Control::ListBox(ref c)=>{
				//writeln!(f,"\t\t\t{}: CListBox::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
				let id = ctrl_id_to_name(&c.id[..]);
				writeln!(f,"\t\tt.{}.{}.Attach(this.GetDlgItem({}));",path,id,c.id).unwrap();
			}
			Control::ComboBox(ref c)=>{
				//writeln!(f,"\t\t\t{}: CComboBox::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
				let id = ctrl_id_to_name(&c.id[..]);
				writeln!(f,"\t\tt.{}.{}.Attach(this.GetDlgItem({}));",path,id,c.id).unwrap();
			}
			Control::Edit(ref c)=>{
				//writeln!(f,"\t\t\t{}: CEdit::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
				let id = ctrl_id_to_name(&c.id[..]);
				writeln!(f,"\t\tt.{}.{}.Attach(this.GetDlgItem({}));",path,id,c.id).unwrap();
			}
			Control::TreeView(ref c)=>{
				//writeln!(f,"\t\t\t{}: CTreeViewCtrlEx::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
				let id = ctrl_id_to_name(&c.id[..]);
				writeln!(f,"\t\tt.{}.{}.Attach(this.GetDlgItem({}));",path,id,c.id).unwrap();
			}
			Control::Static(ref c)=>{
				//only id!=static can be display
				let id = ctrl_id_to_name(&c.id[..]);
				if id != "static" {
					//writeln!(f,"\t\t\t{}: CStatic::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
					writeln!(f,"\t\tt.{}.{}.Attach(this.GetDlgItem({}));",path,id,c.id).unwrap();
				}
			}
			Control::GroupBox(ref c)=>{
				//only id!=static can be display
				let id = ctrl_id_to_name(&c.id[..]);
				if id != "static" {
					//writeln!(f,"\t\t\t{}: CGroupBox::new(),",ctrl_id_to_name(&c.id[..])).unwrap();
					writeln!(f,"\t\tt.{}.{}.Attach(this.GetDlgItem({}));",path,id,c.id).unwrap();
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
}

