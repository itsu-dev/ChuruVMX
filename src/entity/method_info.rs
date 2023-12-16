use crate::entity::attribute_info::AttributeKind;
use crate::entity::constant_pool::*;
use crate::entity::attribute_info::*;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeKind>,
}

pub fn load_methods(count: u16, buffer: &mut &[u8], constant_pool: &Vec<ConstantKind>) -> Vec<MethodInfo> {
    let mut methods = Vec::new();

    for _ in 0..count {
        let attributes_count;
        methods.push(MethodInfo {
            access_flags: buffer.read_u16::<BigEndian>().unwrap(),
            name_index: buffer.read_u16::<BigEndian>().unwrap(),
            descriptor_index: buffer.read_u16::<BigEndian>().unwrap(),
            attributes_count: {
                attributes_count = buffer.read_u16::<BigEndian>().unwrap();
                attributes_count
            },
            attributes: load_attributes(attributes_count, buffer, constant_pool),
        });
    }

    methods
}
