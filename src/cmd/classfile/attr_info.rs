use super::attr_code::Attrcode;
use super::attr_const_info::AttrConst;
use super::attr_dep_info::AttrDepInfo;
use super::attr_exp_info::AttrExpInfo;
use super::attr_ln_info::AttrLineNmInfo;
use super::attr_lvt_info::AttrLocalVarTabInfo;
use super::attr_src_info::AttrSrcInfo;
use super::attr_syn_info::AttrSynInfo;
use super::attr_tbc::AttrTbcInfo;
use super::clz_reader;
use super::const_pool::Constantpool;

pub trait AttrInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32);
}
pub fn read_attrs<'a>(
    data: &Vec<u8>,
    index: &mut u32,
    pool: &'a Constantpool,
    attrs: &mut Vec<Box<dyn AttrInfo + 'a>>,
) {
    let count = clz_reader::read_u16(data, index);
    for i in 0..count {
        read_attr(data, index, pool, attrs);
    }
}

fn read_attr<'a>(
    data: &Vec<u8>,
    index: &mut u32,
    cp: &'a Constantpool,
    attrs: &mut Vec<Box<dyn AttrInfo + 'a>>,
) {
    let attr_nm_index = clz_reader::read_u16(data, index);
    let mut attr_nm_string = String::from("");
    cp.get_utf8(attr_nm_index, &mut attr_nm_string);
    let mut attr_len = 0;
    attr_len = clz_reader::read_u32(data, attr_len, index);
    attrs.push(new_attr(data, index, attr_nm_string, attr_len, cp));
}

fn new_attr<'a>(
    data: &Vec<u8>,
    index: &mut u32,
    nm: String,
    length: u32,
    cp: &'a Constantpool,
) -> Box<dyn AttrInfo + 'a> {
    //带生命周期的返回
    //先解引用，在调用&转成&str
    let dnm = &(*nm);
    let mut attr: Box<dyn AttrInfo + 'a> = match dnm {
        "code" => Box::new(Attrcode {
            cp: cp,
            max_stack: 0,
            max_local: 0,
            code: Vec::new(),
            exception_table: Vec::new(),
            attrs: Vec::new(),
        }),
        "ConstantValue" => Box::new(AttrConst { const_index: 0 }), // _ => (),
        "Deprecated" => Box::new(AttrDepInfo {}),
        "Exceptions" => Box::new(AttrExpInfo {
            exp_index: Vec::new(),
        }),
        "LineNumberTable" => Box::new(AttrLineNmInfo { ln: Vec::new() }),
        "LocalVariableTable" => Box::new(AttrLocalVarTabInfo {
            local_var_table: Vec::new(),
        }),
        "SourceFile" => Box::new(AttrSrcInfo {
            cp: cp,
            src_index: 0,
        }),
        "Synthetic" => Box::new(AttrSynInfo {}),
        _ => Box::new(AttrTbcInfo {
            name: nm,
            length: length,
            info: Vec::new(),
        }),
    };
    attr.read_inf(data, index);
    attr
}
