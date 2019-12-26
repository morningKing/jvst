use super::clz_reader;
use super::cp_info::CpInfo;
pub struct CpIntinfo {
    pub var: i32,
}

impl CpInfo for CpIntinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let tmp: i32 = 0;
        let tmp = clz_reader::read_i32(data, tmp, index);
        self.var = tmp;
        println!("int var : {}", self.var);
        *index
    }
}

pub struct CpFloatinfo {
    pub var: f32,
}

impl CpInfo for CpFloatinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let tmp: f32 = 0.0;
        let tmp = clz_reader::read_f32(data, tmp, index);
        self.var = tmp;
        println!("float var : {}", self.var);
        *index
    }
}

pub struct CpLonginfo {
    pub var: i64,
}

impl CpInfo for CpLonginfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let tmp: i64 = 0;
        let tmp = clz_reader::read_i64(data, tmp, index);
        self.var = tmp;
        println!("long var : {}", self.var);
        *index
    }
}

pub struct CpDoubleinfo {
    pub var: f64,
}

impl CpInfo for CpDoubleinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let tmp: f64 = 0.0;
        let tmp = clz_reader::read_f64(data, tmp, index);
        self.var = tmp;
        println!("double var : {}", self.var);
        *index
    }
}
