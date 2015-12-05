
use std::collections::BTreeMap;
use super::{Name,Control,CtrlType};
use std::path::PathBuf;
use std::io::Write;
use std::fs::{self,File};

enum ContainerType {
	None,
    Dialog,
    TabView,
}

pub struct Container {
	tp: ContainerType,
 	pub name: Name,
    path: String,           //path in ui::Root,it is required by binding file to access a control or a dialog
 	children: BTreeMap<String,Box<Container>>, //all it's children
 	ctrls: Vec<Control>,
}


impl Container {
    pub fn new(id: String,wtl_name: &'static str,msg_name: &'static str)->Container {
    	Container {
    		tp: ContainerType::None,
    		name: Name::new(id, wtl_name, msg_name, ""),
            path: String::new(),       //default path is in root(empty)
    		children: BTreeMap::new(),
    		ctrls: Vec::new(),
    	}
    }

    pub fn with_name(name: Name)->Container {
        Container{
            tp: ContainerType::None,
            name: name,
            path: String::new(),       //default path is in root(empty)
            children: BTreeMap::new(),
            ctrls: Vec::new(),
        }
    }
}

// only used when container is a dialog
impl Container {
    /// parse all controls of this dialog
    pub fn parse_ctrls(&mut self,data: &str){
        let ctrls = data.trim();
        let lines: Vec<&str> = ctrls.lines().collect();
        for l in &lines {
            let tl = l.trim();
            if let Some(c) = Control::parse(tl) {
                //if the control is tabview,add as a container
                //if c.tp == CtrlType::TabView {
                if let CtrlType::TabView = c.tp {
                    let ctrl_id = c.get_id();
                    let tab_name = Name::tab_view(ctrl_id);
                    let child_container = Container::with_name(tab_name);
                    self.add_child(ctrl_id, Box::new(child_container));
                }
                self.add_ctrl(c);                
            }
        }
        //println!("dialog: {}\n{:?}",self.id, self.ctrls);
    }

    /// simply add a parsed control
    fn add_ctrl(&mut self,c: Control){
        self.ctrls.push(c);
    }
}


impl Container {

    /// called in construction stage, add child of the tree path
    pub fn add_child(&mut self,id: &str, d: Box<Container>){
        println!("add child,parent:{},child:{}", self.name.id,d.name.id);
        self.children.insert(id.to_string(), d);
    }

    /// select opertation: delete one child for the given name and return it's value(move to another place)
    pub fn delete_child(&mut self,id: &str)->Box<Container> {
        self.children.remove(id).expect("dlg in root should exist")
    }

    /// unselect opertiaon:delete all children and put them to a vector(then put back to root_dlg)
    pub fn delete_children(&mut self, c: &mut Vec<(String,Box<Container>)>) {
        for (_, child) in &mut self.children {
            child.delete_children(c);
        }

        let k: Vec<_> = self.children.keys().cloned().collect();

        for name in k {
            let bc = self.children.remove(&name).unwrap();
            c.push((name, bc));
        }
        //self.children.remove(key)
    }

    /// make_path of dialog in leave can't be called,so use this function to set path of all dialog
    // pub fn set_path(&mut self,p: &mut Vec<String>) {
    //     p.push(self.name.id);
    //     self.path = p.iter().map(|s|{
    //         dlg_id_to_name(s)
    //     }).collect().join(".");
    //     //self.path.push_str(&jp);
    //     //self.path.push_str(self.name.var_name);
    //     println!("path vec:{:?},path:{}", p,self.path);
    // }

    /// get node of the given path
    pub fn from_path(&mut self,p: &mut Vec<String>)->Option<&mut Self> {
        if let Some(child_name) = p.pop() {
            let d = self.children.get_mut(&child_name).expect("container should exist");
            return d.from_path(p);
        }else{
            Some(self)
        }
    }

    /// all containers of direct children,it is used for tree_view display all children container of a dlg
    pub fn direct_child_container(&self)->Vec<String> {
        self.children.iter().map(|(_,child)|{
            child.name.id.clone()
        }).collect()
    }

    /// recusive print 
    pub fn print(&self,depth: i32) {
        for i in 0..depth {
            print!("    ");
        }
        println!("{}", self.name.id);
        for (_,c) in &self.children {
            c.print(depth+1);
        }
    }
}

macro_rules! tpl_head {
    () => ("
#![allow(dead_code)]
use wtl::*;
use ui::consts::*;
{sub_mod}
");
    (sub) => ("
use super::sub_{var_name}::*;");
}

/// file operation
impl Container {
    fn mkdir(&self,p: &PathBuf){
        fs::create_dir_all(p.as_path().clone()).expect("create dir fail");
    }

    /// write decl,new,create,msg to file
    pub fn write_file(&self,mod_file: &mut File,cur_path: &mut PathBuf) {
        let sub_dir_name = format!("sub_{}",self.name.var_name);
        
        if self.children.len() > 0 {
            //enter child path
            cur_path.push(sub_dir_name.clone());

            //create child dir first
            self.mkdir(&cur_path);

            // create mod.rs for child directory,they append mod and it's sub mod to this file
            let mut child_mod_path = cur_path.clone();
            child_mod_path.push("mod.rs");
            let mut child_mod_file = File::create(child_mod_path.as_path()).unwrap();

            //recursive write
            for (_,c) in &self.children {
                c.write_file(&mut child_mod_file,cur_path);
            }

            //leave child path
            cur_path.pop();
        }
        
        //append mod of this Dialog and it's child dialogs  to the end of the mod.rs in current directory
        self.append_mod_file(mod_file);

        //create dir if not exist
        //fs::create_dir_all(cur_path.as_path().unwrap().clone()).expect("create dir fail");
        self.mkdir(&cur_path);

        //write this Dialog
        cur_path.push(format!("{}.rs",self.name.var_name));
        
        //write current file
        let mut f = File::create(cur_path.clone()).unwrap();

        let sub = self.children.iter().map(|(_,child)|{
            format!(tpl_head!(sub),var_name=child.name.var_name)
        }).collect::<Vec<_>>().concat();

        f.write_all(&format!(tpl_head!(),sub_mod=sub).as_bytes());
        f.write_all(&self.format_decl().as_bytes());
        f.write_all(&self.foramt_impl().as_bytes());

        cur_path.pop();         //delete file name
    }

    /// add xx.rs file as a mod in mod.rs
    fn append_mod_file(&self,f: &mut File) {
        // let mod_name = dlg_id_to_name(&self.id[..]);
        // writeln!(f,"mod {};",mod_name).unwrap();
        // writeln!(f,"pub use self::{}::*;",mod_name).unwrap();

        // if self.node.children.len() > 0 {
        //     writeln!(f,"mod sub_{};",mod_name).unwrap();
        //     writeln!(f,"pub use self::sub_{}::*;",mod_name).unwrap();
        // }
    }

    /// write default handler that binds all controls
    pub fn write_binding_file(&self,mut system_binding_path: &mut PathBuf,dlg_names: &mut Vec<String>) {

        // // recursive write,all binding files in the same directory
        // for (_,c) in &self.node.children {
        //         c.write_binding_file(system_binding_path,dlg_names);
        // }

        // // no ctrls,return
        // if self.ctrls.len() == 0{
        //     return;
        // }

        // let name  = dlg_id_to_name(&self.id[..]);
        // system_binding_path.push(format!("{}.rs",name));
        // let mut f = File::create(system_binding_path.as_path().clone()).unwrap();
        // system_binding_path.pop();

        // writeln!(f,"use ui::Root;").unwrap();
        // writeln!(f,"use user32;").unwrap();
        // writeln!(f,"use winapi::*;").unwrap();
        // writeln!(f,"use ui::consts::*;").unwrap();
        // //only root dialog write this
        // //writeln!(f,"\t\tr.main_dialog.this_msg().on_close(|_,_|{\r\n\t\t\tunsafe{user32::PostQuitMessage(0)};\r\n\t\t});").unwrap();
        
        // writeln!(f,"pub fn register_handler(r: &mut Root) {{").unwrap();
        // writeln!(f,"\tr.{}.this_msg().on_init_dialog(|_,t|{{",self.path).unwrap();
        // writeln!(f,"\t\tt.{}.this.CenterWindow(0 as HWND);",self.path).unwrap();
        // writeln!(f,"\t\tlet this = &t.main_dialog.this;").unwrap();
        // for c in &self.ctrls {
        //     c.write_binding(&self.path,&mut f);
        // }

        // writeln!(f,"\t}}).set_system_priority(0);").unwrap();
        // writeln!(f,"}}").unwrap();

        // dlg_names.push(name.to_string());
        // //self.append_binding_mod_file(binding_mod_file);
    }
}

macro_rules! tpl_decl{
	(container)=>("
pub struct {tp_name}<T> {{
	pub this: {wtl_name}<T>,
	{children_decl}
	{ctrl_decl}
}}");
(child)=>("
	pub {var_name}: {tp_name},");
}

macro_rules! tpl_impl {
    () => ("
impl<T> {tp_name}<T> {{
	{new}
	{create}
	{msg}
}}");
}

//.rs file contains decl,impl and a mod.rs file
impl Container {
	fn format_decl(&self)->String {
		let mut ctrl_decl = String::new();

		//ctrl decl if exist
		if let ContainerType::Dialog = self.tp {
			ctrl_decl = self.ctrls.iter().map(|ctrl|{
				if let Some(ref n) = ctrl.name_for_file() {
					format!(tpl_decl!(child),var_name=n.var_name,tp_name=n.type_name)
				}else{
					String::new()
				}
			}).collect::<Vec<_>>().concat();
		}

		//children decl
		let children_decl = self.children.iter().map(|(_,child)|{
			format!(tpl_decl!(child),var_name=child.name.var_name,tp_name=child.name.type_name)
		}).collect::<Vec<_>>().concat();

		format!(tpl_decl!(container),tp_name=self.name.type_name,wtl_name=self.name.wtl_name,children_decl=children_decl,ctrl_decl=ctrl_decl)
    }

    fn foramt_impl(&self)->String {
    	let new = self.format_new();
    	let create = self.format_create();
    	let msg = self.format_msg();
    	format!(tpl_impl!(),tp_name=self.name.type_name,new=new,create=create,msg=msg)
    }

    fn format_mod(&self)->String {
        String::new()
    }
}

macro_rules! tpl_new {
    (container) => ("
	pub fn new()->{tp_name}<T>{{
		{tp_name}{{
			this: {wtl_name}::new({id}),
			{children_new}
		}}
	}}");
	(child) => ("
			{var_name}: {wtl_name}::new(),");
}

//only dialog has create method
macro_rules! tpl_create {
    (dlg) => ("
    pub fn create(&mut self,r: *mut T){{
		self.this.Create3(r);
		{children_create}
	}}");
	(tab_view) => ("
	pub fn create(&mut self,r: *mut T){{
		{children_create}
	}}");
	(child_dlg) => ("
		self.{var_name}.create(r);");
}

macro_rules! tpl_msg {
    (container) => ("
    pub fn this_msg(&mut self)->{msg_name}<T> {{
		self.this.msg_handler()
	}}

	{ctrl_msg}
	");
	(ctrl)=> ("
	pub fn {var_name}_msg(&mut self)->{msg_name}<T> {{
		self.this.{handler_name}({id})
	}}");
}
//impl contains new,create,msg
impl Container {
    fn format_new(&self)->String {
    	let children_new = self.children.iter().map(|(_,child)|{
    		format!(tpl_new!(child),var_name=child.name.var_name,wtl_name=child.name.wtl_name)
    	}).collect::<Vec<_>>().concat();
    	format!(tpl_new!(container),id=self.name.id,tp_name=self.name.type_name,wtl_name=self.name.wtl_name,children_new=children_new)
    }

    fn format_create(&self)->String {
    	let children_create = self.children.iter().map(|(_,child)|{
    		//if child is dialog,call {var_name}.create(r);
    		match child.tp {
    			ContainerType::Dialog=>{
    				format!(tpl_create!(child_dlg),var_name=child.name.var_name)
    			},
    			_=>String::new(),
    		}
    	}).collect::<Vec<_>>().concat();

    	match self.tp {
    		ContainerType::Dialog=>{
    			format!(tpl_create!(dlg),children_create=children_create)
    		},
    		ContainerType::TabView=>{
    			format!(tpl_create!(tab_view),children_create=children_create)
    		},
    		_=>String::new(),
    	}
    }

    fn format_msg(&self)->String {
    	let ctrl_msg = self.ctrls.iter().map(|ctrl|{
    		if let Some(n) = ctrl.name_for_file() {
    			format!(tpl_msg!(ctrl),var_name=n.var_name,msg_name=n.msg_name,handler_name=n.handler_name,id=n.id)
    		}else{
    			String::new()
    		}
    	}).collect::<Vec<_>>().concat();

    	format!(tpl_msg!(container),msg_name=self.name.msg_name,ctrl_msg=ctrl_msg)
    }
}