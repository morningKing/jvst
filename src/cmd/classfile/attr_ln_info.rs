use super::attr_info::Attrinfo;

pub struct AttrLineNmInfo {
    pub ln: Vec<LineNmEntry>,
}

pub struct LineNmEntry {
    pub start_pc: u16,
    pub line_nm: u16,
}

impl Attrinfo for AttrLineNmInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {}
}
