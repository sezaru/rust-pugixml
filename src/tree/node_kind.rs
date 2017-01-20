#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum NodeKind {
    Null,
    Document,
    Element,
    PCData,
    CData,
    Comment,
    PI,
    Declaration,
    DocType,
}
