use super::clz_reader;
use super::cp_class_info::CpClassInfo;
use super::cp_info::CpInfo;
use super::cp_invoke_dyn_info::{CpConstMethodTypeinfo, CpInvokeDynInfo, CpMethodHandleinfo};
use super::cp_name_type_info::CpNameTypeInfo;
use super::cp_number_info::{CpDoubleinfo, CpFloatInfo, CpIntinfo, CpLonginfo};
use super::cp_ref_info::{CpFieldRefInfo, CpIfaceMethodinfo, CpMethodRefinfo};
use super::cp_string_info::CpStringInfo;
use super::cp_utf8_info::CpUTF8Info;
use std::collections::HashMap;
// use std::ops::Deref;

//常量池结构体
pub struct Constantpool {
    pub count: u16,
    pub constants: HashMap<u16, Box<dyn CpInfo>>,
}

impl Constantpool {
    pub fn get_utf8(&self, index: u16, res: &mut String) {
        let bt = self.constants.get(&index).unwrap();
        //https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object
        let utf8: &CpUTF8Info = match bt.as_any().downcast_ref::<CpUTF8Info>() {
            Some(utf8) => utf8,
            None => panic!("invalid utf8 string"),
        };
        *res = utf8.var.clone();
    }
}

pub fn read_constant_pool(data: &Vec<u8>, constpool: &mut Constantpool) -> u32 {
    let count = 0;
    let mut index = 8; // 常量池在class文件中的第8个字节开始
    let count = clz_reader::read_u16(data, count, &mut index);
    constpool.count = count;
    read_const_info(data, constpool, &mut index);
    index
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

            4 => Box::new(CpFloatInfo { var: 0.0 }),

            5 => Box::new(CpLonginfo { var: 0 }),

            6 => Box::new(CpDoubleinfo { var: 0.0 }),

            1 => Box::new(CpUTF8Info {
                var: String::from(""),
            }),

            8 => Box::new(CpStringInfo { string_index: 0 }),

            7 => Box::new(CpClassInfo { class_index: 0 }),

            9 => Box::new(CpFieldRefInfo {
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

            12 => Box::new(CpNameTypeInfo {
                name_index: 0,
                desc_index: 0,
            }),

            16 => Box::new(CpConstMethodTypeinfo { desc_index: 0 }),

            15 => Box::new(CpMethodHandleinfo {
                ref_kind: 0,
                ref_index: 0,
            }),

            18 => Box::new(CpInvokeDynInfo {
                boot_attr_index: 0,
                name_type_index: 0,
            }),
            _ => Box::new(CpIntinfo { var: 0 }),
        };
        //long和double占两个槽位
        if tag == 5 || tag == 6 {
            counter = counter + 2;
        } else {
            counter = counter + 1;
        }
        cpbox.read_inf(data, index);
        pool.constants.insert(counter, cpbox);
    }
}
