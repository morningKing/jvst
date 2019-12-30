use super::clz_reader;
use super::cp_info::CpInfo;
use std::any::Any;

pub struct CpUTF8info {
    pub var: String,
}

impl CpInfo for CpUTF8info {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let mut var = String::from("");
        let len = 0;
        let len = clz_reader::read_u16(data, len, index) as u32;
        var = clz_reader::read_utf8(data, var, index, len);
        self.var = var;
        println!("utf8info : {}", self.var);
        *index
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
