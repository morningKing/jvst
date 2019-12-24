use super::cp_info::CpInfo;

pub struct CpNameTypeinfo {
    pub name_index: u16, //名称索引
    pub desc_index: u16, //修饰词索引
}

impl CpInfo for CpNameTypeinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: u32) {}
}
