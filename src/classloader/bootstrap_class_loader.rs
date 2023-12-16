use crate::entity::java_class_file::*;
use crate::entity::constant_pool::*;
use crate::entity::field_info::*;
use crate::entity::attribute_info::*;
use crate::entity::method_info::*;
use byteorder::{BigEndian, ReadBytesExt};
use std::string::String;

pub fn define_class(bytes: &[u8]) {
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

    let class_file = JavaClassFileFormat {
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
    };
    dbg!(&class_file);

    class_file;
}

fn load_constant_pool(count: u16, buffer: &mut &[u8]) -> Vec<ConstantKind> {
    let mut constantPool = Vec::new();

    // This constant is for accessing constant pool by index.
    constantPool.push(ConstantKind::Empty(ConstantInfoBase {
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

                constantPool.push(ConstantKind::Utf8(ConstantUtf8Info {
                    base: ConstantInfoBase { tag },
                    length,
                    text: String::from_utf8(bytes).unwrap(),
                }));
            }

            3 => {
                constantPool.push(ConstantKind::Integer(ConstantIntegerInfo {
                    base: ConstantInfoBase { tag },
                    bytes: buffer.read_u32::<BigEndian>().unwrap(),
                }));
            }

            4 => {
                constantPool.push(ConstantKind::Float(ConstantFloatInfo {
                    base: ConstantInfoBase { tag },
                    bytes: buffer.read_u32::<BigEndian>().unwrap(),
                }));
            }


            5 => {
                constantPool.push(ConstantKind::Long(ConstantLongInfo {
                    base: ConstantInfoBase { tag },
                    high_bytes: buffer.read_u32::<BigEndian>().unwrap(),
                    low_bytes: buffer.read_u32::<BigEndian>().unwrap(),
                }));
            }

            6 => {
                constantPool.push(ConstantKind::Double(ConstantDoubleInfo {
                    base: ConstantInfoBase { tag },
                    high_bytes: buffer.read_u32::<BigEndian>().unwrap(),
                    low_bytes: buffer.read_u32::<BigEndian>().unwrap(),
                }));
            }

            7 => {
                constantPool.push(ConstantKind::Class(ConstantClassInfo {
                    base: ConstantInfoBase { tag },
                    name_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            8 => {
                constantPool.push(ConstantKind::String(ConstantStringInfo {
                    base: ConstantInfoBase { tag },
                    string_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            9 => {
                constantPool.push(ConstantKind::Fieldref(ConstantFieldrefInfo {
                    base: ConstantInfoBase { tag },
                    class_index: buffer.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            10 => {
                constantPool.push(ConstantKind::Methodref(ConstantMethodrefInfo {
                    base: ConstantInfoBase { tag },
                    class_index: buffer.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            11 => {
                constantPool.push(ConstantKind::InterfaceMethodref(ConstantInterfaceMethodrefInfo {
                    base: ConstantInfoBase { tag },
                    class_index: buffer.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            12 => {
                constantPool.push(ConstantKind::NameAndType(ConstantNameAndTypeInfo {
                    base: ConstantInfoBase { tag },
                    name_index: buffer.read_u16::<BigEndian>().unwrap(),
                    descriptor_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            15 => {
                constantPool.push(ConstantKind::MethodHandle(ConstantMethodHandleInfo {
                    base: ConstantInfoBase { tag },
                    reference_kind: buffer.read_u8().unwrap(),
                    reference_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            16 => {
                constantPool.push(ConstantKind::MethodType(ConstantMethodTypeInfo {
                    base: ConstantInfoBase { tag },
                    descriptor_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            18 => {
                constantPool.push(ConstantKind::InvokeDynamic(ConstantInvokeDynamicInfo {
                    base: ConstantInfoBase { tag },
                    bootstrap_method_attr_index: buffer.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            19 => {
                constantPool.push(ConstantKind::Module(ConstantModuleInfo {
                    base: ConstantInfoBase { tag },
                    name_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            20 => {
                constantPool.push(ConstantKind::Package(ConstantPackageInfo {
                    base: ConstantInfoBase { tag },
                    name_index: buffer.read_u16::<BigEndian>().unwrap(),
                }));
            }

            _ => {
                panic!("Invalid constant pool tag.");
            }
        }

    }

    constantPool
}

fn load_interfaces(count: u16, buffer: &mut &[u8]) -> Vec<u16> {
    let mut interfaces = Vec::new();

    for _ in 0..count {
        interfaces.push(buffer.read_u16::<BigEndian>().unwrap());
    }

    interfaces
}

fn load_fields(count: u16, buffer: &mut &[u8], constant_pool: &Vec<ConstantKind>) -> Vec<FieldInfo> {
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

fn load_methods(count: u16, buffer: &mut &[u8], constant_pool: &Vec<ConstantKind>) -> Vec<MethodInfo> {
    let mut methods = Vec::new();

    for _ in 0..count {
        let mut attributes_count = 0;
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

fn load_attributes(count: u16, buffer: &mut &[u8], constant_pool: &Vec<ConstantKind>) -> Vec<AttributeKind> {
    let mut attributes: Vec<AttributeKind> = Vec::new();

    for i in 0..count {
        let attribute_name_index = buffer.read_u16::<BigEndian>().unwrap();
        let attribute_length = buffer.read_u32::<BigEndian>().unwrap();
        let name = match &constant_pool[attribute_name_index as usize] {
            ConstantKind::Utf8(utf8_info) => utf8_info.text.clone(),
            _ => panic!("Invalid constant pool entry: {:?}", attribute_name_index),
        };

        match &name[..] {
            "Code" => {
                let mut code_length = 0;
                let mut exception_table_length = 0;
                let mut attributes_count = 0;

                attributes.push(AttributeKind::Code(CodeAttribute {
                    base: AttributeInfoBase{
                        attribute_name_index, attribute_length,
                    },
                    max_stack: buffer.read_u16::<BigEndian>().unwrap(),
                    max_locals: buffer.read_u16::<BigEndian>().unwrap(),
                    code_length: {
                        code_length = buffer.read_u32::<BigEndian>().unwrap();
                        code_length
                    },
                    code: {
                        let mut code = Vec::new();
                        for _ in 0..code_length {
                            code.push(buffer.read_u8().unwrap());
                        }
                        code
                    },
                    exception_table_length: {
                        exception_table_length = buffer.read_u16::<BigEndian>().unwrap();
                        exception_table_length
                    },
                    exception_table: {
                        let mut exception_table = Vec::new();
                        for _ in 0..exception_table_length {
                            exception_table.push(ExceptionTableEntry {
                                start_pc: buffer.read_u16::<BigEndian>().unwrap(),
                                end_pc: buffer.read_u16::<BigEndian>().unwrap(),
                                handler_pc: buffer.read_u16::<BigEndian>().unwrap(),
                                catch_type: buffer.read_u16::<BigEndian>().unwrap(),
                            });
                        }
                        exception_table
                    },
                    attributes_count: {
                        attributes_count = buffer.read_u16::<BigEndian>().unwrap();
                        attributes_count
                    },
                    attributes: load_attributes(attributes_count, buffer, constant_pool),
                }));
            }

            // Unimplemented attributes
            _ => {
                for _ in 0..attribute_length {
                    buffer.read_u8();
                }
            }
        }
    }

    attributes
}
