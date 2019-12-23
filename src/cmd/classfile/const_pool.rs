use super::clz_reader;
use super::cp_info::CpInfo;
use super::cp_number_info::CpFloatinfo;
use super::cp_number_info::CpIntinfo;

pub struct CpLonginfo {
    pub var: i64,
}

pub struct CpDoubleinfo {
    pub var: f64,
}

pub struct CpUTF8info {
    pub var: String,
}

pub struct CpStringinfo {
    pub index: u16, // 只存索引值
}

pub struct CpClassinfo {
    pub index: u16, //只存索引值
}

pub struct CpMemberRefinfo {
    pub class_index: u16,
    pub name_type_index: u16,
}

pub struct CpFieldFefinfo {
    pub class_index: u16,
    pub name_type_index: u16,
}

pub struct CpIfaceMethodinfo {
    pub class_index: u16,
    pub name_type_index: u16,
}

pub struct CpNameTypeinfo {
    pub name_index: u16, //名称索引
    pub desc_index: u16, //修饰词索引
}

pub struct CpConstMethodTypeinfo {
    pub desc_index: u16,
}

pub struct CpMethodHandleinfo {
    pub ref_kind: u8,
    pub ref_index: u16,
}

pub struct CpInvokeDyninfo {
    pub boot_attr_index: u16,
    pub name_type_index: u16,
}

//常量池结构体
pub struct Constantpool {
    pub count: u16,
    pub constants: Vec<Box<dyn CpInfo>>,
}

//读取常量池常量tag
fn read_info_tag(data: &Vec<u8>, index: u32) -> (u8, u32) {
    let tag = 0;
    let (tag, index) = clz_reader::read_u8(data, tag, index);
    (tag, index)
}

fn read_const_count(data: &Vec<u8>, count: u16, index: u32) -> (u16, u32) {
    let (count, index) = clz_reader::read_u16(data, count, index);
    println!("{}:{}", data[8], data[9]);
    println!("constant pool count : {}, index : {}", count, index);
    (count, index)
}

fn read_const_info(data: &Vec<u8>, mut pool: Constantpool, index: u32) {
    for i in (1..pool.count) {
        let (tag, index) = read_info_tag(data, index);
        match tag {
            3 => {
                pool.constants.push(Box::new(CpIntinfo { var: 0 }));
            }
            4 => {
                pool.constants.push(Box::new(CpFloatinfo { var: 0.0 }));
            }
            _ => {}
        }
    }
}

pub fn read_constant_pool(data: &Vec<u8>) -> u32 {
    let count = 0;
    let index = 8; // 常量池在class文件中的第8个字节开始
    let (count, index) = read_const_count(data, count, index);

    let mut constants: Vec<Box<dyn CpInfo>> = Vec::new();
    let mut constpool = Constantpool {
        count: count,
        constants: constants,
    };
    read_const_info(data, constpool, index);

    let mut cii = CpIntinfo { var: 0 };
    cii.read_inf();
    0
}
