use crate::schema::Size;

#[derive(Debug, Clone)]
pub struct Options {
    pub limit: usize,
    pub reverse: bool,
    pub hide_less: Size,
}
