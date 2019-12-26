use super::clz_reader;
use super::cp_info::CpInfo;
pub struct CpInvokeDyninfo {
    pub boot_attr_index: u16,
    pub name_type_index: u16,
}

impl CpInfo for CpInvokeDyninfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let boot_attr_index = 0;
        let boot_attr_index = clz_reader::read_u16(data, boot_attr_index, index);
        self.boot_attr_index = boot_attr_index;
        let name_type_index = 0;
        let name_type_index = clz_reader::read_u16(data, name_type_index, index);
        self.name_type_index = name_type_index;
        println!(
            "invokeDyninfo : {},{}",
            self.boot_attr_index, self.name_type_index
        );
        *index
    }
}

pub struct CpMethodHandleinfo {
    pub ref_kind: u8,
    pub ref_index: u16,
}

impl CpInfo for CpMethodHandleinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let ref_kind = 0;
        let ref_kind = clz_reader::read_u8(data, ref_kind, index);
        self.ref_kind = ref_kind;
        let ref_index = 0;
        let ref_index = clz_reader::read_u16(data, ref_index, index);
        self.ref_index = ref_index;
        println!("methodHandleinfo : {},{}", ref_kind, ref_index);
        *index
    }
}

pub struct CpConstMethodTypeinfo {
    pub desc_index: u16,
}

impl CpInfo for CpConstMethodTypeinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let desc_index = 0;
        let desc_index = clz_reader::read_u16(data, desc_index, index);
        self.desc_index = desc_index;
        println!("methodTypeinfo : {}", self.desc_index);
        *index
    }
}
