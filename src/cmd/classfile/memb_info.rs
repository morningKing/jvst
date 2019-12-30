use super::attr_info::Attrinfo;
use super::const_pool::Constantpool;
pub struct MemberInfo {
    pub cp: Constantpool,
    pub access_flag: u16,
    pub name_index: u16,
    pub desc_index: u16,
    pub attrs: Vec<Box<dyn Attrinfo>>,
}
