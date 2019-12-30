use std::any::Any;
pub trait CpInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) -> u32;
    fn as_any(&self) -> &dyn Any; //将box指针拆箱为实际的结构体类型
}
