#[repr(C)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum Encoding {
    Auto,
    Utf8,
    Utf16Le,
    Utf16Be,
    Utf16,
    Utf32Le,
    Utf32Be,
    Utf32,
    Wchar,
    Latin1,
}
