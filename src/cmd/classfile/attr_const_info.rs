use super::attr_info::Attrinfo;

pub struct AttrConst {
    pub const_index: u16,
}

impl Attrinfo for AttrConst {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {}
}
