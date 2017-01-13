use wrapper::*;

use std::ops::BitOr;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum FormatOption {
    Default,
    Indent,
    IndentAttributes,
    NoEmptyElementTags,
    NoDeclaration,
    NoEscapes,
    Raw,
    SaveFileText,
    WriteBom,
    RawValue(u32),
}

impl FormatOption {
    pub fn value(self) -> u32 {
        match self {
            FormatOption::Default => unsafe { pugi_format_flag_default() },
            FormatOption::Indent => unsafe { pugi_format_flag_indent() },
            FormatOption::IndentAttributes => unsafe { pugi_format_flag_indent_attributes() },
            FormatOption::NoEmptyElementTags => unsafe { pugi_format_flag_no_empty_element_tags() },
            FormatOption::NoDeclaration => unsafe { pugi_format_flag_no_declaration() },
            FormatOption::NoEscapes => unsafe { pugi_format_flag_no_escapes() },
            FormatOption::Raw => unsafe { pugi_format_flag_raw() },
            FormatOption::SaveFileText => unsafe { pugi_format_flag_save_file_text() },
            FormatOption::WriteBom => unsafe { pugi_format_flag_write_bom() },
            FormatOption::RawValue(n) => n,
        }
    }
}

impl BitOr for FormatOption {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        FormatOption::RawValue(self.value() | rhs.value())
    }
}
