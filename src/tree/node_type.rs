#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum NodeType {
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
