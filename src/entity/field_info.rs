use crate::entity::attribute_info::AttributeKind;
use crate::entity::constant_pool::*;
use crate::entity::attribute_info::*;
use byteorder::{BigEndian, ReadBytesExt};


#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeKind>,
}

pub fn load_fields(count: u16, buffer: &mut &[u8], constant_pool: &Vec<ConstantKind>) -> Vec<FieldInfo> {
    let mut fields = Vec::new();

    for _ in 0..count {
        fields.push(FieldInfo {
            access_flags: buffer.read_u16::<BigEndian>().unwrap(),
            name_index: buffer.read_u16::<BigEndian>().unwrap(),
            descriptor_index: buffer.read_u16::<BigEndian>().unwrap(),
            attributes_count: buffer.read_u16::<BigEndian>().unwrap(),
            attributes: load_attributes(count, buffer, constant_pool),
        });
    }

    fields
}
