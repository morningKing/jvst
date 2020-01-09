use super::clz_reader;
use super::cp_info::CpInfo;
use std::any::Any;

pub struct CpNameTypeInfo {
    pub name_index: u16, //名称索引
    pub desc_index: u16, //修饰词索引
}

impl CpInfo for CpNameTypeInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let name_index = 0;
        let name_index = clz_reader::read_u16(data, name_index, index);
        self.name_index = name_index;
        let desc_index = 0;
        let desc_index = clz_reader::read_u16(data, desc_index, index);
        self.desc_index = desc_index;
        println!("nameTypeinfo : {},{}", self.name_index, self.desc_index);
        *index
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
