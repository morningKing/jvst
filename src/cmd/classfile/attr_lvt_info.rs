use super::attr_info::AttrInfo;
use super::clz_reader;

pub struct AttrLocalVarTabInfo {
    pub local_var_table: Vec<LocalVarTabEntry>,
}

pub struct LocalVarTabEntry {
    pub start_pc: u16,
    pub length: u16,
    pub nm_index: u16,
    pub desc_index: u16,
    pub index: u16,
}

impl AttrInfo for AttrLocalVarTabInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {
        let lvt_len = clz_reader::read_u16(data, index);
        for i in 0..lvt_len {
            let mut lvt = LocalVarTabEntry {
                start_pc: 0,
                length: 0,
                nm_index: 0,
                desc_index: 0,
                index: 0,
            };
            lvt.start_pc = clz_reader::read_u16(data, index);
            lvt.length = clz_reader::read_u16(data, index);
            lvt.nm_index = clz_reader::read_u16(data, index);
            lvt.desc_index = clz_reader::read_u16(data, index);
            lvt.index = clz_reader::read_u16(data, index);
            self.local_var_table.push(lvt);
        }
    }
}
