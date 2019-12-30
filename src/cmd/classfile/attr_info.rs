use super::clz_reader;
use super::const_pool;
use super::const_pool::Constantpool;

pub trait Attrinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32);
}
fn read_attrs(
    data: &Vec<u8>,
    index: &mut u32,
    pool: &Constantpool,
    attrs: &Vec<Box<dyn Attrinfo>>,
) {
    let mut count = 0;
    count = clz_reader::read_u16(data, count, index);
    for i in (0..count) {}
}

fn read_attr<'a>(data: &Vec<u8>, index: &mut u32, cp: &Constantpool) {
    let mut attr_nm_index = 0;
    attr_nm_index = clz_reader::read_u16(data, attr_nm_index, index);
    let mut attr_nm_string = String::from("");
    const_pool::get_utf8(cp, attr_nm_index, &mut attr_nm_string);
    let mut attr_len = 0;
    // attr_len = clz_reader::read_u32(data, attr_len, index);
}
