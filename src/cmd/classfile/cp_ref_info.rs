use super::clz_reader;
use super::cp_info::CpInfo;
use std::any::Any;

pub struct CpFieldRefInfo {
    pub class_index: u16,
    pub name_type_index: u16,
}

impl CpInfo for CpFieldRefInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let class_index = 0;
        let class_index = clz_reader::read_u16(data, class_index, index);
        self.class_index = class_index;
        let name_type_index = 0;
        let name_type_index = clz_reader::read_u16(data, name_type_index, index);
        self.name_type_index = name_type_index;
        println!(
            "fieldRefinfo : {},{}",
            self.class_index, self.name_type_index
        );
        *index
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CpIfaceMethodinfo {
    pub class_index: u16,
    pub name_type_index: u16,
}

impl CpInfo for CpIfaceMethodinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let class_index = 0;
        let class_index = clz_reader::read_u16(data, class_index, index);
        self.class_index = class_index;
        let name_type_index = 0;
        let name_type_index = clz_reader::read_u16(data, name_type_index, index);
        self.name_type_index = name_type_index;
        println!(
            "ifaceMethodinfo : {},{}",
            self.class_index, self.name_type_index
        );
        *index
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub struct CpMethodRefinfo {
    pub class_index: u16,
    pub name_type_index: u16,
}

impl CpInfo for CpMethodRefinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32 {
        let class_index = 0;
        let class_index = clz_reader::read_u16(data, class_index, index);
        self.class_index = class_index;
        let name_type_index = 0;
        let name_type_index = clz_reader::read_u16(data, name_type_index, index);
        self.name_type_index = name_type_index;
        println!(
            "methodRefinfo : {},{}",
            self.class_index, self.name_type_index
        );
        *index
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}
