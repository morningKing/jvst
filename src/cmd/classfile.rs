pub mod attr_code;
pub mod attr_const_info;
pub mod attr_dep_info;
pub mod attr_exp_info;
pub mod attr_info;
pub mod attr_ln_info;
pub mod attr_lvt_info;
pub mod attr_src_info;
pub mod attr_syn_info;
pub mod attr_tbc;
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
use clz_reader::{read_u16, read_u16s};
use const_pool::Constantpool;
use std::convert::TryInto;

pub struct Classfile<'a> {
    pub magic: String,
    pub minor_v: String,
    pub major_v: String,
    pub const_pool: &'a Constantpool<'a>,
    pub access_flag: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub iface_count: u16,
    pub ifaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<memb_info::MemberInfo<'a>>,
    pub methods_count: u16,
    pub methods: Vec<memb_info::MemberInfo<'a>>,
}

pub fn readclz<'a>(data: &'a Vec<u8>) {
    let mut index = 8; //常量池从第8位开始
    let mut class = Classfile {
        magic: read_chk_magic(data),
        minor_v: read_chk_minor_version(data),
        major_v: read_chk_major_version(data),
        const_pool: &const_pool::read_constant_pool(data, &mut index),
        access_flag: read_u16(data, &mut index),
        this_class: read_u16(data, &mut index),
        super_class: read_u16(data, &mut index),
        iface_count: 0,
        ifaces: read_u16s(data, &mut index),
        fields_count: 0,
        fields: Vec::new(),
        methods_count: 0,
        methods: Vec::new(),
    };
    //结构数量
    class.iface_count = class.ifaces.len().try_into().unwrap();
    //成员
    class.fields = memb_info::read_mems_info(data, class.const_pool, &mut index);
    class.fields_count = class.fields.len().try_into().unwrap();
    //成员方法
    class.methods = memb_info::read_mems_info(data, class.const_pool, &mut index);
    class.methods_count = class.methods.len().try_into().unwrap();
    //属性表
    let mut attrs: Vec<Box<dyn attr_info::AttrInfo>> = Vec::new();
    attr_info::read_attrs(data, &mut index, class.const_pool, &mut attrs);
}

//检查4字节魔数 cafababe
pub fn read_chk_magic(data: &Vec<u8>) -> String {
    let res = String::from("");
    let mut index = 0;
    let res = clz_reader::read_u32_string(data, res, &mut index);
    println!("magic : {}, index : {}", res, index);
    res
}
//检查主版本号
pub fn read_chk_major_version(data: &Vec<u8>) -> String {
    let res = String::from("");
    let mut index = 6;
    let res = clz_reader::read_u16_string(data, res, &mut index);
    println!("major version : {}, index : {}", res, index);
    res
}
//检查次版本号
pub fn read_chk_minor_version(data: &Vec<u8>) -> String {
    let res = String::from("");
    let mut index = 4;
    let res = clz_reader::read_u16_string(data, res, &mut index);
    println!("minor version : {}, index : {}", res, index);
    res
}
