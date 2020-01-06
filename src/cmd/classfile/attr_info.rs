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
use super::const_pool;
use super::const_pool::Constantpool;

pub trait Attrinfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32);
}
fn read_attrs(
    data: &Vec<u8>,
    index: &mut u32,
    pool: &Constantpool,
    attrs: &Vec<Box<dyn Attrinfo>>,
) {
    let mut count = 0;
    count = clz_reader::read_u16(data, count, index);
    for i in (0..count) {}
}

fn read_attr(data: &Vec<u8>, index: &mut u32, cp: &Constantpool, attrs: &Vec<Box<dyn Attrinfo>>) {
    let mut attr_nm_index = 0;
    attr_nm_index = clz_reader::read_u16(data, attr_nm_index, index);
    let mut attr_nm_string = String::from("");
    const_pool::get_utf8(cp, attr_nm_index, &mut attr_nm_string);
    let mut attr_len = 0;
    attr_len = clz_reader::read_u32(data, attr_len, index);
}

fn new_attr<'a>(
    data: &Vec<u8>,
    index: &mut u32,
    nm: String,
    cp: &Constantpool,
) -> Box<dyn Attrinfo> {
    //先解引用，在调用&转成&str
    let dnm = &(*nm);
    let attr: Box<dyn Attrinfo> = match dnm {
        "code" => Box::new(Attrcode {
            cp: *cp,
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
            cp: *cp,
            src_index: 0,
        }),
        "Synthetic" => Box::new(AttrSynInfo {}),
        _ => Box::new(AttrTbcInfo {
            name: nm,
            length: 0,
            info: Vec::new(),
        }),
    };
    attr
}
