pub trait CpInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32;
}
