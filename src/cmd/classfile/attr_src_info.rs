use super::attr_info::Attrinfo;
use super::const_pool::Constantpool;

pub struct AttrSrcInfo {
    pub cp: Constantpool,
    pub src_index: u16,
}

impl Attrinfo for AttrSrcInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {}
}
