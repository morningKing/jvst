use super::cp_info::CpInfo;
pub struct CpUTF8info {
    pub var: String,
}

impl CpInfo for CpUTF8info {
    fn read_inf(&mut self, data: &Vec<u8>, index: u32) -> u32 {
        // self.var = 0.0;
        0
    }
}
