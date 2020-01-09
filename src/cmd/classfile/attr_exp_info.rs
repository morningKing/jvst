use super::attr_info::AttrInfo;
use super::clz_reader;

pub struct AttrExpInfo {
    pub exp_index: Vec<u16>,
}

impl AttrInfo for AttrExpInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {
        clz_reader::read_u16s(data, &mut self.exp_index, index);
    }
}
