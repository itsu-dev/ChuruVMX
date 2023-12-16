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