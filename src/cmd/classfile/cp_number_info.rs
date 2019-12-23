use super::cp_info::CpInfo;
pub struct CpIntinfo {
    pub var: i32,
}

impl CpInfo for CpIntinfo {
    fn read_inf(&self) {
        println!("{}", self.var);
    }
}

pub struct CpFloatinfo {
    pub var: f32,
}
impl CpInfo for CpFloatinfo {
    fn read_inf(&self) {
        println!("{}", self.var);
    }
}

pub struct CpLonginfo {
    pub var: i64,
}

impl CpInfo for CpLonginfo {
    fn read_inf(&self) {
        println!("{}", self.var);
    }
}

pub struct CpDoubleinfo {
    pub var: f64,
}

impl CpInfo for CpDoubleinfo {
    fn read_inf(&self) {
        println!("{}", self.var);
    }
}
