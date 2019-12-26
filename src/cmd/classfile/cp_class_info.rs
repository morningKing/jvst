use super::clz_reader;
use super::cp_info::CpInfo;

pub struct CpClassinfo {
    pub class_index: u16, //只存索引值
}

impl CpInfo for CpClassinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let class_index = 0;
        let class_index = clz_reader::read_u16(data, class_index, index);
        self.class_index = class_index;
        println!("classinfo : {}", self.class_index);
        *index
    }
}
