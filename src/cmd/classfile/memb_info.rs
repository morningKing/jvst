use super::attr_info;
use super::attr_info::AttrInfo;
use super::clz_reader;
use super::const_pool::Constantpool;

pub struct MemberInfo<'a> {
    pub cp: &'a Constantpool<'a>,
    pub access_flag: u16,
    pub name_index: u16,
    pub desc_index: u16,
    pub attrs: Vec<Box<dyn AttrInfo + 'a>>,
}

pub fn read_mems_info<'a>(
    data: &Vec<u8>,
    cp: &'a Constantpool,
    index: &mut u32,
) -> Vec<MemberInfo<'a>> {
    let mut mems = Vec::new();
    let mems_num = clz_reader::read_u16(data, index);
    for i in 0..mems_num {
        let mut mem = MemberInfo {
            cp: cp,
            access_flag: 0,
            name_index: 0,
            desc_index: 0,
            attrs: Vec::new(),
        };
        mem.access_flag = clz_reader::read_u16(data, index);
        mem.name_index = clz_reader::read_u16(data, index);
        mem.desc_index = clz_reader::read_u16(data, index);
        attr_info::read_attrs(data, index, cp, &mut mem.attrs);
        mems.push(mem);
    }
    mems
}
