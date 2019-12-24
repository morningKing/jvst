pub trait CpInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: u32) -> u32;
}
