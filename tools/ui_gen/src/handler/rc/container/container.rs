use std::path::PathBuf;
use std::io::Write;
use std::fs::{self,File};
use std::collections::BTreeMap;
use super::{Name,Control,CtrlType,util};

pub enum ContainerType {
    Root,
    Dialog,
    TabView,
}

pub struct Container {
	tp: ContainerType,
 	name: Name,
 	children: BTreeMap<String,Box<Container>>, //all it's children
 	ctrls: Vec<Control>,
}

impl Container {
    pub fn new(id: String,wtl_name: &'static str,msg_name: &'static str,tp: ContainerType)->Container {
    	Container {
    		tp: tp,
    		name: Name::new(id, wtl_name, msg_name, ""),
    		children: BTreeMap::new(),
    		ctrls: Vec::new(),
    	}
    }

    pub fn with_name(name: Name,tp: ContainerType)->Container {
        Container{
            tp: tp,
            name: name,
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
                if let CtrlType::TabView = c.tp {
                    let ctrl_id = c.get_id();
                    let tab_name = Name::tab_view(ctrl_id);
                    let child_container = Container::with_name(tab_name,ContainerType::TabView);
                    self.add_child(Box::new(child_container));
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
    pub fn add_child(&mut self, d: Box<Container>){
        println!("add child,parent:{},child:{}", self.name.id,d.name.id);
        self.children.insert(d.name.id.to_string(), d);
    }

    /// select opertation: delete one child for the given name and return it's value(move to another place)
    pub fn delete_child(&mut self,id: &str)->Box<Container> {
        self.children.remove(id).expect("dlg in root should exist")
    }

    /// unselect opertiaon:delete all children and put them to a vector(then put back to root_dlg)
    pub fn delete_children(&mut self, c: &mut Vec<Box<Container>>) {
        for (_, child) in &mut self.children {
            child.delete_children(c);
        }

        let k: Vec<_> = self.children.keys().cloned().collect();

        for name in k {
            let bc = self.children.remove(&name).unwrap();
            c.push(bc);
        }
    }

    /// get container of the given path
    pub fn from_path(&mut self,mut p: &[String])->Option<&mut Self> {
        if p.len() > 0 {
            let child_name = &p[0];
            p = &p[1..];
            let d = self.children.get_mut(child_name).expect("container should exist");
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
    () => 
("#![allow(dead_code)]
use wtl::*;
use ui::consts::*;
{sub_mod}");
    (sub) => 
("use super::{sub_dir_name}::*;");
}
/// file operation
impl Container {
    /// write decl,new,create,msg to file
    pub fn write_file(&self,mod_file: &mut File,cur_path: &mut PathBuf) {
        let sub_dir_name = format!("sub_{}",self.name.var_name);
        
        if self.children.len() > 0 {
            //enter child path
            cur_path.push(sub_dir_name.clone());
            //create child dir first
            util::mkdir(&cur_path);

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
        util::mkdir(&cur_path);

        //write this Dialog
        cur_path.push(format!("{}.rs",self.name.var_name));
        
        //write current file
        let mut f = File::create(cur_path.clone()).unwrap();
        cur_path.pop();         //delete file name

        let mut sub = String::new();
        if self.children.len() > 0 {
            sub = format!(tpl_head!(sub),sub_dir_name=sub_dir_name);
        }

        f.write_all(&format!(tpl_head!(),sub_mod=sub).as_bytes());
        f.write_all(&self.format_decl().as_bytes());
        f.write_all(&self.foramt_impl().as_bytes());
    }
}

macro_rules! tpl_binding {
    () => 
("use ui::Root;
use user32;
use winapi::*;
use ui::consts::*;
{use_children_mod}
pub fn register_handler(r: &mut Root) {{
    {this_handler}
    {children_register}
}}");
    (dlg) => ("
    r{ui_path}.this_msg().on_init_dialog(|_,t|{{
        t{ui_path}.this.CenterWindow(0 as HWND);
        let this = &t{ui_path}.this;
        {ctrl_binding}
    }}).set_system_priority(0);");
    (ctrl) => ("
        t{ui_path}.{var_name}.Attach(this.GetDlgItem({id}));");
    (call_child) => ("
        {var_name}::register_handler(r);
        ");
    (use_child) =>("
use super::{var_name};");
    (mod_file) => ("
mod {var_name};");
}
impl Container {
    /// write default handler that binds all controls
    /// p: file path, ui_path: container in the ui tree path
    pub fn write_binding_file(&self,p: &mut PathBuf,ui_path_vec: &mut Vec<String>) {
        //root name can be "r" or "t" in the ui tree,so write it in template,here use empty string instead
        if let ContainerType::Root = self.tp {
            ui_path_vec.push(String::new());
            self.write_binding_mod(p);
        }else{
            ui_path_vec.push(self.name.var_name.clone());
        }

        //path container or ctrl in the ui tree
        let ui_path = ui_path_vec.join(".");
        //write child binding first
        for (_,child) in &self.children {
            child.write_binding_file(p,ui_path_vec);
        }
        ui_path_vec.pop().unwrap();

        //all container call register handler of all it's children
        let children_register = self.children.iter().map(|(_,child)|{
            format!(tpl_binding!(call_child),var_name=child.name.var_name)
        }).collect::<Vec<_>>().concat().trim().to_string();

        //add use in file
        let use_children_mod = self.children.iter().map(|(_,child)|{
            format!(tpl_binding!(use_child),var_name=child.name.var_name)
        }).collect::<Vec<_>>().concat().trim().to_string();
        //different container will give it's own impl
        let mut this_handler = String::new();
        match self.tp {
            ContainerType::Dialog=>{
                let ctrl_binding = self.format_ctrl_binding(&ui_path);
                this_handler = format!(tpl_binding!(dlg),ui_path=ui_path,ctrl_binding=ctrl_binding);
            }
            _=>{}
        }

        //create file
        p.push(format!("{}.rs",self.name.var_name));
        let mut f = File::create(p.as_path().clone()).unwrap();
        p.pop();

        //only root dialog write this
        //writeln!(f,"\t\tr.main_dialog.this_msg().on_close(|_,_|{\r\n\t\t\tunsafe{user32::PostQuitMessage(0)};\r\n\t\t});").unwrap();
        //write file
        let content = format!(tpl_binding!(),use_children_mod=use_children_mod,this_handler=this_handler,children_register=children_register);
        f.write_all(content.as_bytes());
    }

    fn write_binding_mod(&self,p: &mut PathBuf) {
        let mut mod_decl = Vec::new();
        self.format_binding_mod(&mut mod_decl);
        let content = mod_decl.concat();

        p.push("mod.rs".to_string());
        let mut f = File::create(p.as_path().clone()).unwrap();
        p.pop();

        f.write_all("pub use self::root::*;\r\n".as_bytes());
        f.write_all(content.as_bytes());
    }

    // write all binding file names to mod.rs
    fn format_binding_mod(&self,mod_decl: &mut Vec<String>) {
        mod_decl.push(format!(tpl_binding!(mod_file),var_name=self.name.var_name));
        for (_,child) in &self.children {
            child.format_binding_mod(mod_decl);
        }
    }

    // binding of controls
    fn format_ctrl_binding(&self,ui_path: &str)-> String {
        //ctrl decl if exist
        self.ctrls.iter().map(|ctrl|{
            if let Some(ref n) = ctrl.name_for_file() {
                format!(tpl_binding!(ctrl),ui_path=ui_path,var_name=n.var_name,id=n.id)
            }else{
                String::new()
            }
        }).collect::<Vec<_>>().concat().trim().to_string()
    }
}

macro_rules! tpl_mod {
    () => 
("mod {var_name};
pub use self::{var_name}::*;
{sub_mod}");
    (child) => 
("mod sub_{var_name};
pub use self::sub_{var_name}::*;");
    (root) =>
("
mod consts;
pub use self::consts::*;
mod message_loop;
pub use self::message_loop::MessageLoop;
mod handler;
pub use self::handler::*;");
}
impl Container {
    /// add xx.rs file as a mod in mod.rs
    fn append_mod_file(&self,f: &mut File) {
        let mut sub_mod = String::new();
        if self.children.len() > 0 {
            sub_mod = format!(tpl_mod!(child),var_name=self.name.var_name);
        }
        let mod_file = format!(tpl_mod!(),var_name=self.name.var_name,sub_mod=sub_mod);
        f.write_all(mod_file.as_bytes());

        //this is ui/mod.rs, add handler mod
        if let ContainerType::Root = self.tp {
            f.write_all(tpl_mod!(root).as_bytes());
        }
    }
}

macro_rules! tpl_decl{
	(container)=>("
pub struct {tp_name}<T> {{
	pub this: {wtl_name}<T>,
	{children_decl}
	{ctrl_decl}
}}");
    (root)=>("
pub struct Root {{
    {children_decl}
}}");
    (child)=>("
	pub {var_name}: {tp_name}<T>,");
    (root_child)=>("
    pub {var_name}: {tp_name}<Root>,");
    (ctrl)=>("
    pub {var_name}: {wtl_name},");
}
//.rs file contains decl,impl and a mod.rs file
impl Container {
	fn format_decl(&self)->String {
        match self.tp {
            ContainerType::Root=> {
                //children decl
                let children_decl = self.children.iter().map(|(_,child)|{
                    format!(tpl_decl!(root_child),var_name=child.name.var_name,tp_name=child.name.type_name)
                }).collect::<Vec<_>>().concat().trim().to_string();

                return format!(tpl_decl!(root),children_decl=children_decl);
            }
            _=> {
                //children decl
                let children_decl = self.children.iter().map(|(_,child)|{
                    format!(tpl_decl!(child),var_name=child.name.var_name,tp_name=child.name.type_name)
                }).collect::<Vec<_>>().concat().trim().to_string();

                //control declaration
                let ctrl_decl = self.ctrls.iter().map(|ctrl|{
                    if let Some(ref n) = ctrl.name_for_file() {
                        format!(tpl_decl!(ctrl),var_name=n.var_name,wtl_name=n.wtl_name)
                    }else{
                        String::new()
                    }
                }).collect::<Vec<_>>().concat().trim().to_string();

                return format!(tpl_decl!(container),
                    tp_name=self.name.type_name,
                    wtl_name=self.name.wtl_name,
                    children_decl=children_decl,
                    ctrl_decl=ctrl_decl);
            }
        }
    }
}

macro_rules! tpl_impl {
    () => ("
impl<T> {tp_name}<T> {{
    {new}
    {create}
    {msg}
}}");
    (root) => ("
impl Root {{
    {new}
    {create}
}}");
}
impl Container {
    fn foramt_impl(&self)->String {
    	let new = self.format_new();
    	let create = self.format_create();
        match self.tp {
            ContainerType::Root=> {
                return format!(tpl_impl!(root),new=new,create=create);
            }
            _=> {
                let msg = self.format_msg();
                return format!(tpl_impl!(),tp_name=self.name.type_name,new=new,create=create,msg=msg);
            }
        }
    }
}

macro_rules! tpl_new {
    (container) => ("
	pub fn new()->{tp_name}<T>{{
		{tp_name}{{
			this: {wtl_name}::new({id}),
			{children_new}
            {ctrl_new}
		}}
	}}");
    (root) => ("
    pub fn new()->Root{{
        Root{{
            {children_new}
        }}
    }}");
	(child) => ("
			{var_name}: {tp_name}::new(),");
    (ctrl) => ("
            {var_name}: {wtl_name}::new(),");
}
//impl contains new,create,msg
impl Container {
    fn format_new(&self)->String {
    	let children_new = self.children.iter().map(|(_,child)|{
    		format!(tpl_new!(child),var_name=child.name.var_name,tp_name=child.name.type_name)
    	}).collect::<Vec<_>>().concat().trim().to_string();
        match self.tp {
            ContainerType::Root=> {
                return format!(tpl_new!(root),children_new=children_new);
            }
            _=>{
                let ctrl_new = self.ctrls.iter().map(|ctrl|{
                    if let Some(n) = ctrl.name_for_file() {
                        format!(tpl_new!(ctrl),var_name=n.var_name,wtl_name=n.wtl_name)
                    }else{
                        String::new()
                    }
                }).collect::<Vec<_>>().concat().trim().to_string();

                return format!(tpl_new!(container),
                    id=self.name.id,
                    tp_name=self.name.type_name,
                    wtl_name=self.name.wtl_name,
                    ctrl_new=ctrl_new,
                    children_new=children_new);
            }
        }
    	
    }
}

//only dialog has create method
macro_rules! tpl_create {
    (dlg) => ("
    pub fn create(&mut self,r: *mut T){{
        self.this.Create3(r);
        {children_create}
    }}");
    (root) => ("
    pub fn create(&mut self){{
        let r = self as *mut _ ;
        {children_create}
    }}");
    (tab_view) => ("
    pub fn create(&mut self,r: *mut T){{
        {children_create}
    }}");
    (child_dlg) => ("
        self.{var_name}.create(r);");
}
impl Container {
    fn format_create(&self)->String {
    	let children_create = self.children.iter().map(|(_,child)|{
    		//if child is dialog,call {var_name}.create(r);
    		match child.tp {
    			ContainerType::Dialog=>{
    				format!(tpl_create!(child_dlg),var_name=child.name.var_name)
    			},
    			_=>String::new(),
    		}
    	}).collect::<Vec<_>>().concat().trim().to_string();

    	match self.tp {
    		ContainerType::Dialog=>{
    			format!(tpl_create!(dlg),children_create=children_create)
    		}
    		ContainerType::TabView=>{
    			format!(tpl_create!(tab_view),children_create=children_create)
    		}
    		ContainerType::Root=> {
                format!(tpl_create!(root),children_create=children_create)
            }
    	}
    }
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
impl Container {
    // this func will not be called when self.tp is Root,so there no match statment
    fn format_msg(&self)->String {
    	let ctrl_msg = self.ctrls.iter().map(|ctrl|{
    		if let Some(n) = ctrl.name_for_file() {
    			format!(tpl_msg!(ctrl),var_name=n.var_name,msg_name=n.msg_name,handler_name=n.handler_name,id=n.id)
    		}else{
    			String::new()
    		}
    	}).collect::<Vec<_>>().concat().trim().to_string();

    	format!(tpl_msg!(container),msg_name=self.name.msg_name,ctrl_msg=ctrl_msg)
    }
}