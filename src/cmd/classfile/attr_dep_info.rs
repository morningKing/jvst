use super::attr_info::Attrinfo;

pub struct AttrDepInfo {}

impl Attrinfo for AttrDepInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {
        println!("data len is {}", data.len());
        println!("DeprecatedAttribute at {}", index);
    }
}
