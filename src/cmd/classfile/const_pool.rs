use super::clz_reader;
use super::cp_class_info::CpClassinfo;
use super::cp_info::CpInfo;
use super::cp_invoke_dyn_info::{CpConstMethodTypeinfo, CpInvokeDyninfo, CpMethodHandleinfo};
use super::cp_name_type_info::CpNameTypeinfo;
use super::cp_number_info::{CpDoubleinfo, CpFloatinfo, CpIntinfo, CpLonginfo};
use super::cp_ref_info::{CpFieldRefinfo, CpIfaceMethodinfo, CpMethodRefinfo};
use super::cp_string_info::CpStringinfo;
use super::cp_utf8_info::CpUTF8info;
// use std::ops::Deref;

//自定义常量指针
// struct CpBox<T>(T);
// impl<T> CpBox<T> {
//     fn new(x: T) -> CpBox<T> {
//         CpBox(x)
//     }
// }
//定义Deref 解引用
// impl<T> Deref for CpBox<T> {
//     type Target = T;
//     fn deref(&self) -> &T {
//         &self.0
//     }
// }

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

/**
 * 读取常量池大小
 * 0 为无效索引，常量池常量最大个数 小于等于 n-1
 * double long类型常量占两个位置
 */
fn read_const_count(data: &Vec<u8>, count: u16, index: u32) -> (u16, u32) {
    let (count, index) = clz_reader::read_u16(data, count, index);
    println!("{}:{}", data[8], data[9]);
    println!("constant pool count : {}, index : {}", count, index);
    (count, index)
}

fn read_const_info(data: &Vec<u8>, pool: &mut Constantpool, index: u32) {
    for i in (1..pool.count) {
        let (tag, index) = read_info_tag(data, index);
        match tag {
            3 => {
                let mut cpbox = Box::new(CpIntinfo { var: 0 });
                pool.constants.push(cpbox);
            }
            4 => {
                let mut cpbox = Box::new(CpFloatinfo { var: 0.0 });
                pool.constants.push(cpbox);
            }
            5 => {
                let mut cpbox = Box::new(CpLonginfo { var: 0 });
                pool.constants.push(cpbox);
            }
            6 => {
                let mut cpbox = Box::new(CpDoubleinfo { var: 0.0 });
                pool.constants.push(cpbox);
            }
            1 => {
                let mut cpbox = Box::new(CpUTF8info {
                    var: String::from(""),
                });
                pool.constants.push(cpbox);
            }
            8 => {
                let mut cpbox = Box::new(CpStringinfo { index: 0 });
                pool.constants.push(cpbox);
            }
            7 => {
                let mut cpbox = Box::new(CpClassinfo { index: 0 });
                pool.constants.push(cpbox);
            }
            9 => {
                let mut cpbox = Box::new(CpFieldRefinfo {
                    class_index: 0,
                    name_type_index: 0,
                });
                pool.constants.push(cpbox);
            }
            10 => {
                let mut cpbox = Box::new(CpMethodRefinfo {
                    class_index: 0,
                    name_type_index: 0,
                });
                pool.constants.push(cpbox);
            }
            11 => {
                let mut cpbox = Box::new(CpIfaceMethodinfo {
                    class_index: 0,
                    name_type_index: 0,
                });
                pool.constants.push(cpbox);
            }
            12 => {
                let mut cpbox = Box::new(CpNameTypeinfo {
                    name_index: 0,
                    desc_index: 0,
                });
                pool.constants.push(cpbox);
            }
            16 => {
                let mut cpbox = Box::new(CpConstMethodTypeinfo { desc_index: 0 });
                pool.constants.push(cpbox);
            }
            15 => {
                let mut cpbox = Box::new(CpMethodHandleinfo {
                    ref_kind: 0,
                    ref_index: 0,
                });
                pool.constants.push(cpbox);
            }
            18 => {
                let mut cpbox = Box::new(CpInvokeDyninfo {
                    boot_attr_index: 0,
                    name_type_index: 0,
                });
                pool.constants.push(cpbox);
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
    read_const_info(data, &mut constpool, index);
    0
}
