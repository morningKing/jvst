use super::attr_info::Attrinfo;
use super::const_pool::Constantpool;

pub struct AttrExpInfo {
    pub exp_index: Vec<u16>,
}

impl Attrinfo for AttrExpInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {}
}
