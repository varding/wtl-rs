

use std::collections::BTreeMap;
use super::Dialog;

pub struct Tab {
    pub dlgs: BTreeMap<String,Box<Dialog>>,
}