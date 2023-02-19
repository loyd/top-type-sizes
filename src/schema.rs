pub type Size = u32;

#[derive(Debug, PartialEq, Eq)]
pub struct Type {
    pub name: String,
    pub size: Size,
    pub align: Size,
    pub kind: TypeKind,
    pub end_padding: Option<Size>,
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
    pub discriminant_size: Option<Size>,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EnumVariant {
    pub name: String,
    pub size: Size,
    pub items: Vec<FieldOrPadding>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FieldOrPadding {
    Field(Field),
    Padding(Size),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Field {
    pub name: String,
    pub size: Size,
    pub align: Option<Size>,
    // TODO: what is it?
    pub offset: Option<Size>,
}
