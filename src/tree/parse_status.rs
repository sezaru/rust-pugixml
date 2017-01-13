#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum ParseStatus {
    Ok,
    FileNotFound,
    IOError,
    OutOfMemory,
    InternalError,
    UnrecognizedTag,
    BadPI,
    BadComment,
    BadCData,
    BadDocType,
    BadPCData,
    BadStartElement,
    BadAttribute,
    BadEndElement,
    EndElementMismatch,
    AppendInvalidRoot,
    NoDocumentElement,
}