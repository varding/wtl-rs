
pub struct Name {
    pub id: String,				//id in resource.h		ID_MAIN_DIALOG
    pub var_name: String,		//variable name: main_dialog
    pub type_name: String,		//type name: MainDialog
    pub wtl_name: &'static str,		//wtl name: Dialog,Button,TabView
    pub msg_name: &'static str,		//msg name: DlgMsg,BtnMsg
    pub handler_name: &'static str,	//handler name: self.this.btn_handler(IDC_BTN_SELECT)
}

fn id_to_name(id: &str)->String {
    if id.starts_with("IDD_") || id.starts_with("IDC_") {
        id[4..].to_lowercase()
    }else{
        id.to_lowercase()
    }
}

//from librustc
fn to_camel_case(s: &str) -> String {
    s.split('_').flat_map(|word| word.chars().enumerate().map(|(i, c)|
        if i == 0 {
            c.to_uppercase().collect::<String>()
        } else {
            c.to_lowercase().collect()
        }
    )).collect::<Vec<_>>().concat()
}

/// var_name and type_name
fn var_type_name(id: &str)->(String,String) {
    let var_name = id_to_name(id);
    let tp_name = to_camel_case(&var_name);
    (var_name,tp_name)
}

impl Name {
    pub fn new(id: String,wtl_name: &'static str,msg_name: &'static str,handler_name: &'static str)-> Name {
        let (var_name, type_name) = var_type_name(&id);
    	Name {
    		id: id,
    		var_name: var_name,
    		type_name: type_name,
    		wtl_name: wtl_name,
    		msg_name: msg_name,
    		handler_name: handler_name,
    	}
    }
}

/// container
impl Name {
    pub fn root()->Name {
        Name {
            id: "Root".to_string(),
            var_name: "root".to_string(),
            type_name: "Root".to_string(),
            wtl_name: "Root",
            msg_name: "",
            handler_name: "",
        }
    }

    pub fn dialog(id: &str)->Name {
        let (var_name, type_name) = var_type_name(id);
        Name {
            id: id.to_string(),
            var_name: var_name,
            type_name: type_name,
            wtl_name: "Dialog",
            msg_name: "DlgMsg",
            handler_name: "",
        }
    }

    pub fn tab_view(id: &str)->Name {
        let (var_name, type_name) = var_type_name(id);
        Name {
            id: id.to_string(),
            var_name: var_name,
            type_name: type_name,
            wtl_name: "CTabView",
            msg_name: "",
            handler_name: "",
        }
    }
}

/// control
impl Name {
    pub fn radio_btn(id: &str)->Name {
        let (var_name, type_name) = var_type_name(id);
        Name {
            id: id.to_string(),
            var_name: var_name,
            type_name: type_name,
            wtl_name: "CButton",
            msg_name: "BtnMsg",
            handler_name: "btn_handler",
        }
    }

    pub fn button(id: &str)->Name {
        let (var_name, type_name) = var_type_name(id);
        Name {
            id: id.to_string(),
            var_name: var_name,
            type_name: type_name,
            wtl_name: "CButton",
            msg_name: "BtnMsg",
            handler_name: "btn_handler",
        }
    }

    pub fn tree_view(id: &str)->Name {
        let (var_name, type_name) = var_type_name(id);
        Name {
            id: id.to_string(),
            var_name: var_name,
            type_name: type_name,
            wtl_name: "CTreeViewCtrlEx",
            msg_name: "TreeMsg",
            handler_name: "tree_handler",
        }
    }

    // pub fn tab_view(id: &str)->Name {
    //     let (var_name, type_name) = var_type_name(id);
    //     Name {
    //         id: id.to_string(),
    //         var_name: var_name,
    //         type_name: type_name,
    //         wtl_name: "CTabCtrl",
    //         msg_name: "TabMsg",
    //         handler_name: "tab_handler",
    //     }
    // }

    pub fn list_box(id: &str)->Name {
        let (var_name, type_name) = var_type_name(id);
        Name {
            id: id.to_string(),
            var_name: var_name,
            type_name: type_name,
            wtl_name: "CListBox",
            msg_name: "LstBxMsg",
            handler_name: "lstbx_handler",
        }
    }

    pub fn combox(id: &str)->Name {
        let (var_name, type_name) = var_type_name(id);
        Name {
            id: id.to_string(),
            var_name: var_name,
            type_name: type_name,
            wtl_name: "CComboBox",
            msg_name: "CmbBoxMsg",
            handler_name: "cmbbox_handler",
        }
    }

    pub fn edit(id: &str)->Name {
        let (var_name, type_name) = var_type_name(id);
        Name {
            id: id.to_string(),
            var_name: var_name,
            type_name: type_name,
            wtl_name: "CEdit",
            msg_name: "EdtMsg",
            handler_name: "edt_handler",
        }
    }

    pub fn ltext(id: &str)->Name {
        let (var_name, type_name) = var_type_name(id);
        Name {
            id: id.to_string(),
            var_name: var_name,
            type_name: type_name,
            wtl_name: "CStatic",
            msg_name: "StcMsg",
            handler_name: "stc_handler",
        }
    }
}