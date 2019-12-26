use super::clz_reader;
use super::cp_class_info::CpClassinfo;
use super::cp_info::CpInfo;
use super::cp_invoke_dyn_info::{CpConstMethodTypeinfo, CpInvokeDyninfo, CpMethodHandleinfo};
use super::cp_name_type_info::CpNameTypeinfo;
use super::cp_number_info::{CpDoubleinfo, CpFloatinfo, CpIntinfo, CpLonginfo};
use super::cp_ref_info::{CpFieldRefinfo, CpIfaceMethodinfo, CpMethodRefinfo};
use super::cp_string_info::CpStringinfo;
use super::cp_utf8_info::CpUTF8info;
use std::collections::HashMap;
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
    // pub constants: Vec<Box<dyn CpInfo>>,
    pub constants: HashMap<u16, Box<dyn CpInfo>>,
}

fn read_const_info(data: &Vec<u8>, pool: &mut Constantpool, index: &mut u32) {
    let mut counter: u16 = 1;

    loop {
        if counter == pool.count {
            break;
        }
        let mut tag = 0;
        tag = clz_reader::read_u8(data, tag, index);
        let mut cpbox: Box<dyn CpInfo> = match tag {
            3 => Box::new(CpIntinfo { var: 0 }),

            4 => Box::new(CpFloatinfo { var: 0.0 }),

            5 => Box::new(CpLonginfo { var: 0 }),

            6 => Box::new(CpDoubleinfo { var: 0.0 }),

            1 => Box::new(CpUTF8info {
                var: String::from(""),
            }),

            8 => Box::new(CpStringinfo { string_index: 0 }),

            7 => Box::new(CpClassinfo { class_index: 0 }),

            9 => Box::new(CpFieldRefinfo {
                class_index: 0,
                name_type_index: 0,
            }),

            10 => Box::new(CpMethodRefinfo {
                class_index: 0,
                name_type_index: 0,
            }),

            11 => Box::new(CpIfaceMethodinfo {
                class_index: 0,
                name_type_index: 0,
            }),

            12 => Box::new(CpNameTypeinfo {
                name_index: 0,
                desc_index: 0,
            }),

            16 => Box::new(CpConstMethodTypeinfo { desc_index: 0 }),

            15 => Box::new(CpMethodHandleinfo {
                ref_kind: 0,
                ref_index: 0,
            }),

            18 => Box::new(CpInvokeDyninfo {
                boot_attr_index: 0,
                name_type_index: 0,
            }),
            _ => Box::new(CpIntinfo { var: 0 }),
        };
        if tag == 5 || tag == 6 {
            counter = counter + 2;
        } else {
            counter = counter + 1;
        }
        cpbox.read_inf(data, index);
        pool.constants.insert(counter, cpbox);
    }
}

pub fn read_constant_pool(data: &Vec<u8>) -> u32 {
    let count = 0;
    let mut index = 8; // 常量池在class文件中的第8个字节开始
    let count = clz_reader::read_u16(data, count, &mut index);

    // let mut constants: Vec<Box<dyn CpInfo>> = Vec::new();
    let mut constants: HashMap<u16, Box<dyn CpInfo>> = HashMap::new();
    let mut constpool = Constantpool {
        count: count,
        // constants: constants,
        constants: constants,
    };
    read_const_info(data, &mut constpool, &mut index);
    0
}
