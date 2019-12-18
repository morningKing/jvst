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

fn read_u8(data: &[u8], res: u8) -> u8 {
    0
}

fn read_u16(data: &[u8], res: u16) -> u16 {
    0
}

fn read_u32(data: &[u8], mut res: u32, index: u8) -> u32 {
    // let arr: [u8; 4] = [data[index],data[index],data[index],data[index]];
    // for number in (0..3).rev() {
    //     let index: u8 = index + 1;
    //     arr[number] = data[number];
    // }
    // unsafe {
    //     res = mem::transmute::<[u8; 4], u32>(arr);
    // }
    0
}

fn readAnChkMagic(data: &Vec<u8>) {}

impl Classfile {
    pub fn parseClzFile(data: &Vec<u8>) {}
}
