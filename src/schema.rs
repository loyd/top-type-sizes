#[derive(Debug, PartialEq, Eq)]
pub struct Type {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub kind: TypeKind,
    pub end_padding: Option<usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeKind {
    Struct(StructType),
    Enum(EnumType),
}

#[derive(Debug, PartialEq, Eq)]
pub struct StructType {
    pub items: Vec<FieldOrPadding>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumType {
    pub discriminant_size: Option<usize>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumVariant {
    pub name: String,
    pub size: usize,
    pub items: Vec<FieldOrPadding>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FieldOrPadding {
    Field(Field),
    Padding(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub size: usize,
    pub align: Option<usize>,
    // TODO: what is it?
    pub offset: Option<usize>,
}
