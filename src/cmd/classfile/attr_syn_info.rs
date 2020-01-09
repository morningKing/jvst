use super::attr_info::AttrInfo;

pub struct AttrSynInfo {}

impl AttrInfo for AttrSynInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {
        println!("Synthetic data len is {} index is {}", data.len(), index);
    }
}
