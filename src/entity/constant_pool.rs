use byteorder::{BigEndian, ReadBytesExt};
use std::string::String;

#[derive(Debug, Clone)]
pub struct ConstantInfoBase {
    pub tag: u8,
}

#[derive(Debug, Clone)]
pub struct ConstantUtf8Info {
    pub base: ConstantInfoBase,  // 1
    pub length: u16,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct ConstantIntegerInfo {
    pub base: ConstantInfoBase,  // 3
    pub bytes: u32,
}

#[derive(Debug, Clone)]
pub struct ConstantFloatInfo {
    pub base: ConstantInfoBase,  // 4
    pub bytes: u32,
}

#[derive(Debug, Clone)]
pub struct ConstantLongInfo {
    pub base: ConstantInfoBase,  // 5
    pub high_bytes: u32,
    pub low_bytes: u32,
}

#[derive(Debug, Clone)]
pub struct ConstantDoubleInfo {
    pub base: ConstantInfoBase,  // 6
    pub high_bytes: u32,
    pub low_bytes: u32,
}

#[derive(Debug, Clone)]
pub struct ConstantClassInfo {
    pub base: ConstantInfoBase,  // 7
    pub name_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantStringInfo {
    pub base: ConstantInfoBase,  // 8
    pub string_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantFieldrefInfo {
    pub base: ConstantInfoBase,  // 9
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantMethodrefInfo {
    pub base: ConstantInfoBase,  // 10
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantInterfaceMethodrefInfo {
    pub base: ConstantInfoBase,  // 11
    pub class_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantNameAndTypeInfo {
    pub base: ConstantInfoBase,  // 12
    pub name_index: u16,
    pub descriptor_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantMethodHandleInfo {
    pub base: ConstantInfoBase,  // 15
    pub reference_kind: u8,
    pub reference_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantMethodTypeInfo {
    pub base: ConstantInfoBase,  // 16
    pub descriptor_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantInvokeDynamicInfo {
    pub base: ConstantInfoBase,  // 18
    pub bootstrap_method_attr_index: u16,
    pub name_and_type_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantModuleInfo {
    pub base: ConstantInfoBase,  // 19
    pub name_index: u16,
}

#[derive(Debug, Clone)]
pub struct ConstantPackageInfo {
    pub base: ConstantInfoBase,  // 20
    pub name_index: u16,
}

#[derive(Debug, Clone)]
pub enum ConstantKind {
    Empty(ConstantInfoBase),
    Utf8(ConstantUtf8Info),
    Integer(ConstantIntegerInfo),
    Float(ConstantFloatInfo),
    Long(ConstantLongInfo),
    Double(ConstantDoubleInfo),
    Class(ConstantClassInfo),
    String(ConstantStringInfo),
    Fieldref(ConstantFieldrefInfo),
    Methodref(ConstantMethodrefInfo),
    InterfaceMethodref(ConstantInterfaceMethodrefInfo),
    NameAndType(ConstantNameAndTypeInfo),
    MethodHandle(ConstantMethodHandleInfo),
    MethodType(ConstantMethodTypeInfo),
    InvokeDynamic(ConstantInvokeDynamicInfo),
    Module(ConstantModuleInfo),
    Package(ConstantPackageInfo),
}

pub fn load_constant_pool(count: u16, buffer: &mut &[u8]) -> Vec<ConstantKind> {
    let mut constant_pool = Vec::new();

    // This constant is for accessing constant pool by index.
    constant_pool.push(ConstantKind::Empty(ConstantInfoBase {
         tag: 0,
    }));

    for i in 0..count - 1 {
        let tag = buffer.read_u8().unwrap();

        match tag {
            1 => {
                let mut bytes: Vec<u8> = Vec::new();
                let length = buffer.read_u16::<BigEndian>().unwrap();
                for _ in 0..length {
                    bytes.push(buffer.read_u8().unwrap());
                }

                constant_pool.push(ConstantKind::Utf8(ConstantUtf8Info {
                    base: ConstantInfoBase { tag },
                    length,
                    text: String::from_utf8(bytes).unwrap(),
                }));
            }

            3 => {
                constant_pool.push(ConstantKind::Integer(ConstantIntegerInfo {
                    base: ConstantInfoBase { tag },
                    bytes: buffer.read_u32::<BigEndian>().unwrap(),
                }));
            }

            4 => {
                constant_pool.push(ConstantKind::Float(ConstantFloatInfo {
                    base: ConstantInfoBase { tag },
                    bytes: buffer.read_u32::<BigEndian>().unwrap(),
                }));
            }


            5 => {
                constant_pool.push(ConstantKind::Long(ConstantLongInfo {
                    base: ConstantInfoBase { tag },
                    high_bytes: buffer.read_u32::<BigEndian>().unwrap(),
                    low_bytes: buffer.read_u32::<BigEndian>().unwrap(),
                }));
            }

            6 => {
                constant_pool.push(ConstantKind::Double(ConstantDoubleInfo {
                    base: ConstantInfoBase { tag },
                    high_bytes: buffer.read_u32::<BigEndian>().unwrap(),
                    low_bytes: buffer.read_u32::<BigEndian>().unwrap(),
                }));
            }

            7 => {
                constant_pool.push(ConstantKind::Class(ConstantClassInfo {
                    base: ConstantInfoBase { tag },
                    name_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            8 => {
                constant_pool.push(ConstantKind::String(ConstantStringInfo {
                    base: ConstantInfoBase { tag },
                    string_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            9 => {
                constant_pool.push(ConstantKind::Fieldref(ConstantFieldrefInfo {
                    base: ConstantInfoBase { tag },
                    class_index: buffer.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            10 => {
                constant_pool.push(ConstantKind::Methodref(ConstantMethodrefInfo {
                    base: ConstantInfoBase { tag },
                    class_index: buffer.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            11 => {
                constant_pool.push(ConstantKind::InterfaceMethodref(ConstantInterfaceMethodrefInfo {
                    base: ConstantInfoBase { tag },
                    class_index: buffer.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            12 => {
                constant_pool.push(ConstantKind::NameAndType(ConstantNameAndTypeInfo {
                    base: ConstantInfoBase { tag },
                    name_index: buffer.read_u16::<BigEndian>().unwrap(),
                    descriptor_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            15 => {
                constant_pool.push(ConstantKind::MethodHandle(ConstantMethodHandleInfo {
                    base: ConstantInfoBase { tag },
                    reference_kind: buffer.read_u8().unwrap(),
                    reference_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            16 => {
                constant_pool.push(ConstantKind::MethodType(ConstantMethodTypeInfo {
                    base: ConstantInfoBase { tag },
                    descriptor_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            18 => {
                constant_pool.push(ConstantKind::InvokeDynamic(ConstantInvokeDynamicInfo {
                    base: ConstantInfoBase { tag },
                    bootstrap_method_attr_index: buffer.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            19 => {
                constant_pool.push(ConstantKind::Module(ConstantModuleInfo {
                    base: ConstantInfoBase { tag },
                    name_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            20 => {
                constant_pool.push(ConstantKind::Package(ConstantPackageInfo {
                    base: ConstantInfoBase { tag },
                    name_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            _ => {
                panic!("Invalid constant pool tag.");
            }
        }

    }

    constant_pool
}