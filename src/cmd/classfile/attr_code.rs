use super::attr_info::Attrinfo;
use super::const_pool::Constantpool;

pub struct Attrcode {
    cp: Constantpool,
    max_stack: u16,
    max_local: u16,
    code: Vec<u8>,
    exception_table: Vec<Exception>,
    attrs: Vec<Box<dyn Attrinfo>>,
}

pub struct Exception {
    start: u16,
    end: u16,
    handler: u16,
    catch: u16,
}

impl Attrinfo for Attrcode {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {}
}
