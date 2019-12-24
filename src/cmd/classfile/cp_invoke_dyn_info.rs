use super::cp_info::CpInfo;

pub struct CpInvokeDyninfo {
    pub boot_attr_index: u16,
    pub name_type_index: u16,
}

impl CpInfo for CpInvokeDyninfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: u32) -> u32 {
        // self.var = 0.0;
        0
    }
}

pub struct CpMethodHandleinfo {
    pub ref_kind: u8,
    pub ref_index: u16,
}

impl CpInfo for CpMethodHandleinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: u32) -> u32 {
        // self.var = 0.0;
        0
    }
}

pub struct CpConstMethodTypeinfo {
    pub desc_index: u16,
}

impl CpInfo for CpConstMethodTypeinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: u32) -> u32 {
        // self.var = 0.0;
        0
    }
}
