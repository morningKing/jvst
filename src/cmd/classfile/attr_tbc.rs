use super::attr_info::AttrInfo;
use super::clz_reader;

pub struct AttrTbcInfo {
    pub name: String,
    pub length: u32,
    pub info: Vec<u8>,
}

impl AttrInfo for AttrTbcInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {
        clz_reader::read_u8s(data, &mut self.info, index, self.length);
    }
}
