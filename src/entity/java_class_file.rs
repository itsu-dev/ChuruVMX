use crate::entity::constant_pool::ConstantKind;

use super::{field_info::FieldInfo, method_info::MethodInfo, attribute_info::AttributeKind};

#[derive(Debug, Clone)]
pub struct JavaClassFileFormat {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Vec<ConstantKind>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Vec<u16>,
    pub fields_count: u16,
    pub fields: Vec<FieldInfo>,
    pub methods_count: u16,
    pub methods: Vec<MethodInfo>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeKind>,
}

pub trait JavaClassFile {
    
}


impl JavaClassFile for JavaClassFileFormat {
    
}