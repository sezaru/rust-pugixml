use wrapper::*;

use std::ops::BitOr;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum ParseOption {
    CData,
    Comments,
    Declaration,
    Default,
    DocType,
    Eol,
    Escapes,
    Fragment,
    Full,
    Minimal,
    PI,
    TrimPCData,
    WSPCData,
    WSPCDataSingle,
    WConvAttribute,
    WNormAttribute,
    RawValue(u32),
}

impl ParseOption {
    pub fn value(self) -> u32 {
        match self {
            ParseOption::CData => unsafe { pugi_parse_flag_cdata() },
            ParseOption::Comments => unsafe { pugi_parse_flag_comments() },
            ParseOption::Declaration => unsafe { pugi_parse_flag_declaration() },
            ParseOption::Default => unsafe { pugi_parse_flag_default() },
            ParseOption::DocType => unsafe { pugi_parse_flag_doctype() },
            ParseOption::Eol => unsafe { pugi_parse_flag_eol() },
            ParseOption::Escapes => unsafe { pugi_parse_flag_escapes() },
            ParseOption::Fragment => unsafe { pugi_parse_flag_fragment() },
            ParseOption::Full => unsafe { pugi_parse_flag_full() },
            ParseOption::Minimal => unsafe { pugi_parse_flag_minimal() },
            ParseOption::PI => unsafe { pugi_parse_flag_pi() },
            ParseOption::TrimPCData => unsafe { pugi_parse_flag_trim_pcdata() },
            ParseOption::WSPCData => unsafe { pugi_parse_flag_ws_pcdata() },
            ParseOption::WSPCDataSingle => unsafe { pugi_parse_flag_ws_pcdata_single() },
            ParseOption::WConvAttribute => unsafe { pugi_parse_flag_wconv_attribute() },
            ParseOption::WNormAttribute => unsafe { pugi_parse_flag_wnorm_attribute() },
            ParseOption::RawValue(n) => n,
        }
    }
}

impl BitOr for ParseOption {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        ParseOption::RawValue(self.value() | rhs.value())
    }
}
