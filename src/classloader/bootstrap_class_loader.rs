use std::collections::HashMap;

use crate::entity::java_class_file::*;
use crate::entity::constant_pool::load_constant_pool;
use crate::entity::field_info::load_fields;
use crate::entity::attribute_info::load_attributes;
use crate::entity::method_info::load_methods;
use byteorder::{BigEndian, ReadBytesExt};

// TODO need to rename: remove the prefix `_`
pub fn _find_class(name: &str, _class_path: &Vec<String>, class_files: &mut HashMap<String, JavaClassFileFormat>) -> JavaClassFileFormat {
    if let Some(existing_class) = class_files.get(name) {
        return existing_class.clone();
    }

    // TODO find class from class_path
    define_class(&(name.to_owned() + ".class"), &[][..], class_files)
}

pub fn define_class(path: &str, bytes: &[u8], class_files: &mut HashMap<String, JavaClassFileFormat>) -> JavaClassFileFormat {
    let defined_class = _define_class(bytes);
    class_files.entry(path.to_string().split("/").last().unwrap().replace(".class", ""))
        .or_insert(defined_class)
        .to_owned()
}

fn _define_class(bytes: &[u8]) -> JavaClassFileFormat {
    let mut buffer = &bytes[..];

    let magic = buffer.read_u32::<BigEndian>().unwrap();
    if magic != 0xCAFEBABE {
        panic!("Invalid binary type.");
    }

    let minor_version = buffer.read_u16::<BigEndian>().unwrap();
    let major_version = buffer.read_u16::<BigEndian>().unwrap();
    if major_version < 45 || 61 < major_version {
        panic!("Invalid binary version.");
    }

    let constant_pool_count = buffer.read_u16::<BigEndian>().unwrap();
    let constant_pool = load_constant_pool(constant_pool_count, &mut buffer);
    let access_flags = buffer.read_u16::<BigEndian>().unwrap();
    let this_class = buffer.read_u16::<BigEndian>().unwrap();
    let super_class = buffer.read_u16::<BigEndian>().unwrap();
    let interfaces_count = buffer.read_u16::<BigEndian>().unwrap();
    let interfaces = load_interfaces(interfaces_count, &mut buffer);
    let fields_count = buffer.read_u16::<BigEndian>().unwrap();
    let fields = load_fields(fields_count, &mut buffer, &constant_pool);
    let methods_count = buffer.read_u16::<BigEndian>().unwrap();
    let methods = load_methods(methods_count, &mut buffer, &constant_pool);
    let attributes_count = buffer.read_u16::<BigEndian>().unwrap();
    let attributes = load_attributes(attributes_count, &mut buffer, &constant_pool);

    JavaClassFileFormat {
        minor_version,
        major_version,
        constant_pool_count,
        constant_pool,
        access_flags,
        this_class,
        super_class,
        interfaces_count,
        interfaces,
        fields_count,
        fields,
        methods_count,
        methods,
        attributes_count,
        attributes,
    }
}

fn load_interfaces(count: u16, buffer: &mut &[u8]) -> Vec<u16> {
    let mut interfaces = Vec::new();

    for _ in 0..count {
        interfaces.push(buffer.read_u16::<BigEndian>().unwrap());
    }

    interfaces
}
