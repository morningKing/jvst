use super::attr_info::Attrinfo;
use super::clz_reader;

pub struct AttrLineNmInfo {
    pub ln: Vec<LineNmEntry>,
}

pub struct LineNmEntry {
    pub start_pc: u16,
    pub line_nm: u16,
}

impl Attrinfo for AttrLineNmInfo {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {
        let mut ln_len = 0;
        ln_len = clz_reader::read_u16(data, ln_len, index);
        for i in 0..ln_len {
            let mut ln = LineNmEntry {
                start_pc: 0,
                line_nm: 0,
            };
            ln.start_pc = clz_reader::read_u16(data, ln.start_pc, index);
            ln.line_nm = clz_reader::read_u16(data, ln.line_nm, index);
            self.ln.push(ln);
        }
    }
}
