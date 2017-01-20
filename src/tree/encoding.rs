#[repr(C)]
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
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
