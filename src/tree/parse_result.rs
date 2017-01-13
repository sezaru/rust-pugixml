use wrapper::*;

use libc::ptrdiff_t;

use std::default::Default;

use std::ptr::null;

use helpers::from_c_char_to_string;

use tree::{ParseStatus, Encoding};

pub enum CParseResult {}

#[derive(Debug)]
pub struct ParseResult {
    parse_result_ptr: *const CParseResult,

    pub status: ParseStatus,
    pub offset: ptrdiff_t,
    pub encoding: Encoding,
    pub description: String,
}

impl Default for ParseResult {
    fn default() -> Self {
        ParseResult {
            parse_result_ptr: null(),
            status: ParseStatus::InternalError,
            offset: 0,
            encoding: Encoding::Auto,
            description: String::new(),
        }
    }
}

impl Drop for ParseResult {
    fn drop(&mut self) {
        unsafe {
            pugi_delete_parse_result(self.parse_result_ptr);
        }
    }
}

impl ParseResult {
    pub fn new(parse_result: *const CParseResult) -> Result<Self, ()> {
        if parse_result.is_null() {
            Err(())
        } else {
            unsafe {
                Ok(ParseResult {
                    parse_result_ptr: parse_result,
                    status: pugi_parse_result_status(parse_result),
                    offset: pugi_parse_result_offset(parse_result),
                    encoding: pugi_parse_result_encoding(parse_result),
                    description: from_c_char_to_string(pugi_parse_result_description(parse_result)),
                })
            }
        }
    }
}
