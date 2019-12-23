pub mod clz_reader;
pub mod const_pool;
pub mod cp_info;
pub mod cp_number_info;
pub struct contant_pool {}

pub struct constant_info {}

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

pub fn readclz(data: &Vec<u8>) {
    read_chk_magic(data);
    read_chk_minor_version(data);
    read_chk_major_version(data);
    const_pool::read_constant_pool(data);
}

//检查4字节魔数 cafababe
pub fn read_chk_magic(data: &Vec<u8>) -> u32 {
    let res = String::from("");
    let index = 0;
    let (res, index) = clz_reader::read_u32_string(data, res, index);
    println!("magic : {}, index : {}", res, index);
    index
}
//检查主版本号
pub fn read_chk_major_version(data: &Vec<u8>) -> u32 {
    let res = String::from("");
    let index = 6;
    let (res, index) = clz_reader::read_u16_string(data, res, index);
    println!("major version : {}, index : {}", res, index);
    index
}
//检查次版本号
pub fn read_chk_minor_version(data: &Vec<u8>) -> u32 {
    let res = String::from("");
    let index = 4;
    let (res, index) = clz_reader::read_u16_string(data, res, index);
    println!("minor version : {}, index : {}", res, index);
    index
}