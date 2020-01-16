use super::clz_reader;
use super::cp_info::CpInfo;
use std::any::Any;

pub struct CpStringInfo {
    pub string_index: u16, // 只存索引值
}

impl CpInfo for CpStringInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let string_index = clz_reader::read_u16(data, index);
        self.string_index = string_index;
        println!("stringinfo : {}", self.string_index);
        *index
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
