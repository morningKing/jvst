use super::cp_info::CpInfo;

pub struct CpStringinfo {
    pub index: u16, // 只存索引值
}

impl CpInfo for CpStringinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: u32) -> u32 {
        // self.var = 0.0;
        0
    }
}
