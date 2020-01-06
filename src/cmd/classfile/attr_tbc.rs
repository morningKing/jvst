use super::attr_info::Attrinfo;

pub struct AttrTbcInfo {
    pub name: String,
    pub length: u32,
    pub info: Vec<u8>,
}

impl Attrinfo for AttrTbcInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {}
}
