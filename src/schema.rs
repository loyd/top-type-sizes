use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Type {
    pub name: String,
    pub size: usize,
    pub align: usize,
    pub kind: TypeKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_padding: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum TypeKind {
    Struct(StructType),
    Enum(EnumType),
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct StructType {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<FieldOrPadding>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct EnumType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discriminant_size: Option<usize>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct EnumVariant {
    pub name: String,
    pub size: usize,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<FieldOrPadding>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum FieldOrPadding {
    Field(Field),
    Padding(usize),
}

impl FieldOrPadding {
    pub fn size(&self) -> usize {
        match self {
            Self::Field(f) => f.size,
            Self::Padding(p) => *p,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Field {
    pub kind: FieldKind,
    pub name: String,
    pub size: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_type: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub enum FieldKind {
    AdtField,
    Upvar,
    GeneratorLocal,
}
