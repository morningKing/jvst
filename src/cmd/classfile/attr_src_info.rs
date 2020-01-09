use super::attr_info::AttrInfo;
use super::clz_reader;
use super::const_pool::Constantpool;

pub struct AttrSrcInfo<'a> {
    pub cp: &'a Constantpool,
    pub src_index: u16,
}

impl<'a> AttrInfo for AttrSrcInfo<'a> {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {
        self.src_index = clz_reader::read_u16(data, self.src_index, index);
    }
}

impl<'a> AttrSrcInfo<'a> {
    pub fn read_file_name(&self) -> String {
        let mut file_name = String::from("");
        self.cp.get_utf8(self.src_index, &mut file_name);
        file_name
    }
}
