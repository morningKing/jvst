use super::attr_info::Attrinfo;
use super::clz_reader;

pub struct AttrConst {
    pub const_index: u16,
}

impl Attrinfo for AttrConst {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {
        self.const_index = clz_reader::read_u16(data, self.const_index, index);
    }
}
