use super::attr_info::AttrInfo;
use super::clz_reader;

pub struct AttrLineNmInfo {
    pub ln: Vec<LineNmEntry>,
}

pub struct LineNmEntry {
    pub start_pc: u16,
    pub line_nm: u16,
}

impl AttrInfo for AttrLineNmInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {
        let ln_len = clz_reader::read_u16(data, index);
        for i in 0..ln_len {
            let mut ln = LineNmEntry {
                start_pc: 0,
                line_nm: 0,
            };
            ln.start_pc = clz_reader::read_u16(data, index);
            ln.line_nm = clz_reader::read_u16(data, index);
            self.ln.push(ln);
        }
    }
}
