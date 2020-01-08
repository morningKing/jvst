use super::attr_info;
use super::attr_info::Attrinfo;
use super::clz_reader;
use super::const_pool::Constantpool;
pub struct Attrcode<'a> {
    pub cp: &'a Constantpool,
    pub max_stack: u16,
    pub max_local: u16,
    pub code: Vec<u8>,
    pub exception_table: Vec<Exception>,
    pub attrs: Vec<Box<dyn Attrinfo + 'a>>,
}

pub struct Exception {
    start: u16,
    end: u16,
    handler: u16,
    catch: u16,
}

//带生命周期的结构体实现trait
impl<'a> Attrinfo for Attrcode<'a> {
    fn read_inf(&mut self, data: &Vec<u8>, index: &mut u32) {
        self.max_stack = clz_reader::read_u16(data, self.max_stack, index);
        self.max_local = clz_reader::read_u16(data, self.max_local, index);

        let mut code_len: u32 = 0;
        code_len = clz_reader::read_u32(data, code_len, index);
        clz_reader::read_u8s(data, &mut self.code, index, code_len);

        read_exp(data, index, &mut self.exception_table);
        attr_info::read_attrs(data, index, self.cp, &mut self.attrs);
    }
}

fn read_exp(data: &Vec<u8>, index: &mut u32, exptab: &mut Vec<Exception>) {
    let mut exp_len = 0;
    exp_len = clz_reader::read_u16(data, exp_len, index);
    for i in 0..exp_len {
        let mut exp = Exception {
            start: 0,
            end: 0,
            handler: 0,
            catch: 0,
        };
        exp.start = clz_reader::read_u16(data, exp.start, index);
        exp.end = clz_reader::read_u16(data, exp.end, index);
        exp.handler = clz_reader::read_u16(data, exp.handler, index);
        exp.catch = clz_reader::read_u16(data, exp.catch, index);
        exptab.push(exp);
    }
}
