use super::attr_info::Attrinfo;

pub struct AttrLocalVarTabInfo {
    pub local_var_table: Vec<LocalVarTabEntry>,
}

pub struct LocalVarTabEntry {
    pub start_pc: u16,
    pub length: u16,
    pub nm_index: u16,
    pub desc_index: u16,
    pub index: u16,
}

impl Attrinfo for AttrLocalVarTabInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {}
}
