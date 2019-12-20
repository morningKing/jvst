extern crate hex;
use std::convert::TryInto;
use std::mem;
pub struct contant_pool {}

pub struct field {}

pub struct method {}

pub struct iface {}

pub struct Classfile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub const_pool_count: u16,
    pub const_pool: Vec<contant_pool>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub iface_count: u16,
    pub ifaces: Vec<iface>,
    pub fields_count: u16,
    pub fields: Vec<field>,
    pub methods_count: u16,
    pub methods: Vec<method>,
}

// fn read_u8(data: &[u8], mut res: String, mut index: u32) -> (String, u32) {

// res = hex::encode(data.get(index));
// 0
// }

fn read_u16(data: &[u8], mut res: String, mut index: u32) -> (String, u32) {
    let mut tmparr: [u8; 2] = [0; 2];
    let end = index + 2;
    for (i, n) in data.iter().enumerate() {
        // 使用try_into().unwrap() 将u8 转换成usize
        if i >= index.try_into().unwrap() && i < end.try_into().unwrap() {
            // println!("{},{}", i, n);
            tmparr[i] = *n;
        } else if i == end.try_into().unwrap() {
            break;
        }
    }
    index = index + 2;
    res = hex::encode(tmparr);
    // unsafe {
    //     res = mem::transmute::<[u8; 2], u16>(tmparr);
    // }
    // 使用元祖类型进行多值返回
    (res, index)
}

fn read_u32(data: &[u8], mut res: String, mut index: u32) -> (String, u32) {
    let mut tmparr: [u8; 4] = [0; 4];
    let end = index + 4;
    for (i, n) in data.iter().enumerate() {
        if i >= index.try_into().unwrap() && i < end.try_into().unwrap() {
            tmparr[i] = *n;
        } else if i == end.try_into().unwrap() {
            break;
        }
    }
    index = index + 4;
    res = hex::encode(tmparr);
    (res, index)
}

pub fn read_chk_magic(data: &Vec<u8>) -> u32 {
    let mut res = String::from("");
    let index = 0;
    println!("magic content : {} {}", data[0], data[1]);
    let (res, index) = read_u16(data, res, index);
    println!("magic : {}, index : {}", res, index);
    index
}

pub fn read_chk_minor_version(data: &Vec<u8>) {}

impl Classfile {
    pub fn parseClzFile(data: &Vec<u8>) {}
}
