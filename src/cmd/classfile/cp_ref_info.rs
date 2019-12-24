use super::cp_info::CpInfo;

pub struct CpFieldRefinfo {
    pub class_index: u16,
    pub name_type_index: u16,
}

impl CpInfo for CpFieldRefinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: u32) -> u32 {
        0
    }
}

pub struct CpIfaceMethodinfo {
    pub class_index: u16,
    pub name_type_index: u16,
}

impl CpInfo for CpIfaceMethodinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: u32) -> u32 {
        // self.var = 0.0;
        0
    }
}

pub struct CpMethodRefinfo {
    pub class_index: u16,
    pub name_type_index: u16,
}

impl CpInfo for CpMethodRefinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: u32) -> u32 {
        // self.var = 0.0;
        0
    }
}
