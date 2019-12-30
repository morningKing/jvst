pub mod attr_code;
pub mod attr_info;
pub mod clz_reader;
pub mod const_pool;
pub mod cp_class_info;
pub mod cp_info;
pub mod cp_invoke_dyn_info;
pub mod cp_name_type_info;
pub mod cp_number_info;
pub mod cp_ref_info;
pub mod cp_string_info;
pub mod cp_utf8_info;
pub mod memb_info;
pub struct constant_info {}

pub struct field {}

pub struct method {}

pub struct iface {}

pub struct Classfile {
    pub magic: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub const_pool: const_pool::Constantpool,
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
    let mut index = 0;
    read_chk_magic(data);
    read_chk_minor_version(data);
    read_chk_major_version(data);
    index = const_pool::read_constant_pool(data);
    let mut access_flag = 0;
    access_flag = clz_reader::read_u16(data, access_flag, &mut index);
    let mut this_class = 0;
    this_class = clz_reader::read_u16(data, this_class, &mut index);
    let mut super_class = 0;
    super_class = clz_reader::read_u16(data, super_class, &mut index);
    let mut slice: Vec<u16> = Vec::new();
    clz_reader::read_u16s(data, &mut slice, &mut index);
}

//检查4字节魔数 cafababe
pub fn read_chk_magic(data: &Vec<u8>) -> u32 {
    let res = String::from("");
    let mut index = 0;
    let res = clz_reader::read_u32_string(data, res, &mut index);
    println!("magic : {}, index : {}", res, index);
    index
}
//检查主版本号
pub fn read_chk_major_version(data: &Vec<u8>) -> u32 {
    let res = String::from("");
    let mut index = 6;
    let res = clz_reader::read_u16_string(data, res, &mut index);
    println!("major version : {}, index : {}", res, index);
    index
}
//检查次版本号
pub fn read_chk_minor_version(data: &Vec<u8>) -> u32 {
    let res = String::from("");
    let mut index = 4;
    let res = clz_reader::read_u16_string(data, res, &mut index);
    println!("minor version : {}, index : {}", res, index);
    index
}
