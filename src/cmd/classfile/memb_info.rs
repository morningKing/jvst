use super::attr_info;
use super::attr_info::Attrinfo;
use super::clz_reader;
use super::const_pool::Constantpool;

pub struct MemberInfo<'a> {
    pub cp: &'a Constantpool,
    pub access_flag: u16,
    pub name_index: u16,
    pub desc_index: u16,
    pub attrs: Vec<Box<dyn Attrinfo + 'a>>,
}

fn read_mems_info(data: &Vec<u8>, cp: &Constantpool, index: &mut u32, mems: &mut Vec<MemberInfo>) {
    let mut mems_num = 0;
    mems_num = clz_reader::read_u16(data, mems_num, index);
    for i in 0..mems_num {
        let mut mem = MemberInfo {
            cp: cp,
            access_flag: 0,
            name_index: 0,
            desc_index: 0,
            attrs: Vec::new(),
        };
        mem.access_flag = clz_reader::read_u16(data, mem.access_flag, index);
        mem.name_index = clz_reader::read_u16(data, mem.name_index, index);
        mem.desc_index = clz_reader::read_u16(data, mem.desc_index, index);
        attr_info::read_attrs(data, index, cp, &mut mem.attrs);
        mems.push(mem);
    }
}
