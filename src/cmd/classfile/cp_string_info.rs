use super::clz_reader;
use super::cp_info::CpInfo;

pub struct CpStringinfo {
    pub string_index: u16, // 只存索引值
}

impl CpInfo for CpStringinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let string_index = 0;
        let string_index = clz_reader::read_u16(data, string_index, index);
        self.string_index = string_index;
        println!("stringinfo : {}", self.string_index);
        *index
    }
}
