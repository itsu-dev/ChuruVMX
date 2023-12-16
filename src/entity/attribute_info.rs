use crate::entity::constant_pool::*;
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug, Clone)]
pub struct AttributeInfoBase {
    pub attribute_name_index: u16,
    pub attribute_length: u32,
}

#[derive(Debug, Clone)]
pub struct ConstantValueAttribute {
    pub base: AttributeInfoBase,
    pub constant_value_index: u16,
}

#[derive(Debug, Clone)]
pub struct CodeAttribute {
    pub base: AttributeInfoBase,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code_length: u32,
    pub code: Vec<u8>,
    pub exception_table_length: u16,
    pub exception_table: Vec<ExceptionTableEntry>,
    pub attributes_count: u16,
    pub attributes: Vec<AttributeKind>,
}

#[derive(Debug, Clone)]
pub struct ExceptionTableEntry {
    pub start_pc: u16,
    pub end_pc: u16,
    pub handler_pc: u16,
    pub catch_type: u16,
}

#[derive(Debug, Clone)]
pub struct StackMapTableAttribute {
    pub base: AttributeInfoBase,
    pub number_of_entries: u16,
    pub entries: Vec<StackMapFrame>,
}

#[derive(Debug, Clone)]
pub struct StackMapFrame {
    pub frame_type: u8,
    pub same_frame: SameFrame,
    pub same_locals_1_stack_item_frame: SameLocals1StackItemFrame,
    pub same_locals_1_stack_item_frame_extended: SameLocals1StackItemFrameExtended,
    pub chop_frame: ChopFrame,
    pub same_frame_extended: SameFrameExtended,
    pub append_frame: AppendFrame,
    pub full_frame: FullFrame,
}

#[derive(Debug, Clone)]
pub struct SameFrame {
    pub frame_type: u8,
}

#[derive(Debug, Clone)]
pub struct VerificationTypeInfo {
    pub tag: u8,
    pub value: u16,
}

#[derive(Debug, Clone)]
pub struct SameLocals1StackItemFrame {
    pub frame_type: u8,
    pub stack: VerificationTypeInfo,
}

#[derive(Debug, Clone)]
pub struct SameLocals1StackItemFrameExtended {
    pub frame_type: u8,
    pub offset_delta: u16,
    pub stack: VerificationTypeInfo,
}

#[derive(Debug, Clone)]
pub struct ChopFrame {
    pub frame_type: u8,
    pub offset_delta: u16,
}

#[derive(Debug, Clone)]
pub struct SameFrameExtended {
    pub frame_type: u8,
    pub offset_delta: u16,
}

#[derive(Debug, Clone)]
pub struct AppendFrame {
    pub frame_type: u8,
    pub offset_delta: u16,
    pub locals: Vec<VerificationTypeInfo>,
}

#[derive(Debug, Clone)]
pub struct FullFrame {
    pub frame_type: u8,
    pub offset_delta: u16,
    pub number_of_locals: u16,
    pub locals: Vec<VerificationTypeInfo>,
    pub number_of_stack_items: u16,
    pub stack: Vec<VerificationTypeInfo>,
}

#[derive(Debug, Clone)]
pub struct BootstrapMethodsAttribute {
    pub base: AttributeInfoBase,
    pub num_bootstrap_methods: u16,
    pub bootstrap_methods: Vec<BootstrapMethod>,
}

#[derive(Debug, Clone)]
pub struct BootstrapMethod {
    pub bootstrap_method_ref: u16,
    pub num_bootstrap_arguments: u16,
    pub bootstrap_arguments: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct NestHostAttribute {
    pub base: AttributeInfoBase,
    pub host_class_index: u16,
}

#[derive(Debug, Clone)]
pub struct NestMembersAttribute {
    pub base: AttributeInfoBase,
    pub number_of_classes: u16,
    pub classes: Vec<u16>,
}

#[derive(Debug, Clone)]
pub struct PermittedSubclassesAttribute {
    pub base: AttributeInfoBase,
    pub number_of_classes: u16,
    pub classes: Vec<u16>,
}

// TODO: use unused attributes.
#[derive(Debug, Clone)]
#[allow(dead_code)]  // TODO remove
pub enum AttributeKind {
    ConstantValue(ConstantValueAttribute),
    Code(CodeAttribute),
    StackMapTable(StackMapTableAttribute),
    BootstrapMethods(BootstrapMethodsAttribute),
    NestHost(NestHostAttribute),
    NestMembers(NestMembersAttribute),
    PermittedSubclasses(PermittedSubclassesAttribute),
}

pub fn load_attributes(count: u16, buffer: &mut &[u8], constant_pool: &Vec<ConstantKind>) -> Vec<AttributeKind> {
    let mut attributes: Vec<AttributeKind> = Vec::new();

    for _ in 0..count {
        let attribute_name_index = buffer.read_u16::<BigEndian>().unwrap();
        let attribute_length = buffer.read_u32::<BigEndian>().unwrap();
        let name = match &constant_pool[attribute_name_index as usize] {
            ConstantKind::Utf8(utf8_info) => utf8_info.text.clone(),
            _ => panic!("Invalid constant pool entry: {:?}", attribute_name_index),
        };

        match &name[..] {
            "Code" => {
                let code_length;
                let exception_table_length;
                let attributes_count;

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
                    let _ = buffer.read_u8();
                }
            }
        }
    }

    attributes
}
