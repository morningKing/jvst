use super::attr_info::Attrinfo;

pub struct AttrSynInfo {}

impl Attrinfo for AttrSynInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {}
}