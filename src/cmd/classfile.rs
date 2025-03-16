pub mod attr_code;           // 代码属性模块
pub mod attr_const_info;     // 常量属性信息模块
pub mod attr_dep_info;       // 依赖属性信息模块
pub mod attr_exp_info;       // 导出属性信息模块
pub mod attr_info;           // 属性信息基础模块
pub mod attr_ln_info;        // 行号属性信息模块
pub mod attr_lvt_info;       // 局部变量表属性信息模块
pub mod attr_src_info;       // 源码属性信息模块
pub mod attr_syn_info;       // 综合属性信息模块
pub mod attr_tbc;            // 待定属性模块
pub mod clz_reader;          // 类文件读取器模块
pub mod const_pool;          // 常量池模块
pub mod cp_class_info;       // 类信息常量模块
pub mod cp_info;             // 常量信息基础模块
pub mod cp_invoke_dyn_info;  // 动态调用信息常量模块
pub mod cp_name_type_info;   // 名称类型信息常量模块
pub mod cp_number_info;      // 数字信息常量模块
pub mod cp_ref_info;         // 引用信息常量模块
pub mod cp_string_info;      // 字符串信息常量模块
pub mod cp_utf8_info;        // UTF8字符串信息常量模块
pub mod memb_info;           // 成员信息模块
use clz_reader::{read_u16, read_u16s};
use const_pool::Constantpool;
use std::convert::TryInto;

pub struct Classfile<'a> {
    pub magic: String,                           // 魔数，标识字节码文件的格式
    pub minor_v: String,                         // 次版本号
    pub major_v: String,                         // 主版本号
    pub const_pool: &'a Constantpool<'a>,        // 常量池引用
    pub access_flag: u16,                        // 访问标志，表示类或接口的访问权限和属性
    pub this_class: u16,                         // 当前类索引
    pub super_class: u16,                        // 父类索引
    pub iface_count: u16,                        // 接口数量
    pub ifaces: Vec<u16>,                        // 接口索引数组
    pub fields_count: u16,                       // 字段数量
    pub fields: Vec<memb_info::MemberInfo<'a>>,  // 字段信息数组
    pub methods_count: u16,                      // 方法数量
    pub methods: Vec<memb_info::MemberInfo<'a>>, // 方法信息数组
}

pub fn readclz<'a>(data: &'a Vec<u8>) {
    let mut index = 8; // 常量池从第8位开始
    let mut class = Classfile {
        magic: read_chk_magic(data),               // 读取魔数
        minor_v: read_chk_minor_version(data),     // 读取次版本号
        major_v: read_chk_major_version(data),     // 读取主版本号
        const_pool: &const_pool::read_constant_pool(data, &mut index), // 读取常量池
        access_flag: read_u16(data, &mut index),   // 读取访问标志
        this_class: read_u16(data, &mut index),    // 读取当前类索引
        super_class: read_u16(data, &mut index),   // 读取父类索引
        iface_count: 0,                            // 初始化接口数量为0
        ifaces: read_u16s(data, &mut index),       // 读取接口索引数组
        fields_count: 0,                           // 初始化字段数量为0
        fields: Vec::new(),                        // 初始化字段数组
        methods_count: 0,                          // 初始化方法数量为0
        methods: Vec::new(),                       // 初始化方法数组
    };
    // 设置接口数量
    class.iface_count = class.ifaces.len().try_into().unwrap();
    // 读取字段信息
    class.fields = memb_info::read_mems_info(data, class.const_pool, &mut index);
    class.fields_count = class.fields.len().try_into().unwrap();
    // 读取方法信息
    class.methods = memb_info::read_mems_info(data, class.const_pool, &mut index);
    class.methods_count = class.methods.len().try_into().unwrap();
    // 读取属性表
    let mut attrs: Vec<Box<dyn attr_info::AttrInfo>> = Vec::new();
    attr_info::read_attrs(data, &mut index, class.const_pool, &mut attrs);
}

// 检查4字节魔数 cafababe，验证是否为有效的Java类文件
pub fn read_chk_magic(data: &Vec<u8>) -> String {
    let res = String::from("");
    let mut index = 0;
    // 从字节数组中读取4字节的魔数（u32）并转换为字符串
    let res = clz_reader::read_u32_string(data, res, &mut index);
    println!("magic : {}, index : {}", res, index);
    res
}

// 检查主版本号，确定类文件的JDK版本兼容性
pub fn read_chk_major_version(data: &Vec<u8>) -> String {
    let res = String::from("");
    let mut index = 6;
    // 从字节数组的第6个位置读取2字节的主版本号并转换为字符串
    let res = clz_reader::read_u16_string(data, res, &mut index);
    println!("major version : {}, index : {}", res, index);
    res
}

// 检查次版本号，通常为0，偶尔用于标识小更新
pub fn read_chk_minor_version(data: &Vec<u8>) -> String {
    let res = String::from("");
    let mut index = 4;
    // 从字节数组的第4个位置读取2字节的次版本号并转换为字符串
    let res = clz_reader::read_u16_string(data, res, &mut index);
    println!("minor version : {}, index : {}", res, index);
    res
}
