use std::ffi::CString;

use wrapper::*;

use tree::{ParseOption, ParseResult, Encoding, ParseStatus, FormatOption};

pub enum CDocument {}

#[derive(Debug)]
pub struct Document {
    doc_ptr: *mut CDocument,
}

impl Drop for Document {
    fn drop(&mut self) {
        unsafe {
            pugi_delete_document(self.doc_ptr);
        }
    }
}

impl Document {
    pub fn new() -> Result<Self, ()> {
        unsafe {
            let document = pugi_new_document();
            if document.is_null() {
                Err(())
            } else {
                Ok(Document { doc_ptr: document })
            }
        }
    }

    pub fn load_file(self, path: &str, parse_options: Vec<ParseOption>, encoding: Encoding)
        -> Result<Self, ParseResult> {
        let combined_parse_options = parse_options.iter()
            .fold(ParseOption::RawValue(0), |acc, x| acc | *x);
        let c_path = CString::new(path).unwrap();

        unsafe {
            match ParseResult::new(pugi_load_file(self.doc_ptr,
                                                  c_path.as_ptr(),
                                                  combined_parse_options.value(),
                                                  encoding)) {
                Ok(result) => {
                    match result.status {
                        ParseStatus::Ok => Ok(self),
                        _ => Err(result),
                    }
                }
                _ => Err(ParseResult::default()),
            }
        }
    }

    pub fn load_buffer(self, contents: &str, parse_options: Vec<ParseOption>, encoding: Encoding)
        -> Result<Self, ParseResult> {
        let combined_parse_options = parse_options.iter()
            .fold(ParseOption::RawValue(0), |acc, x| acc | *x);
        let c_contents = CString::new(contents).unwrap();

        unsafe {
            match ParseResult::new(pugi_load_buffer(self.doc_ptr,
                                                    c_contents.as_ptr(),
                                                    contents.len(),
                                                    combined_parse_options.value(),
                                                    encoding)) {
                Ok(result) => {
                    match result.status {
                        ParseStatus::Ok => Ok(self),
                        _ => Err(result),
                    }
                }
                _ => Err(ParseResult::default()),
            }
        }
    }

    pub fn save_file(self, path: &str, indent_string: &str, format_options: Vec<FormatOption>,
                     encoding: Encoding)
        -> Result<Self, ()> {
        let combined_format_options = format_options.iter()
            .fold(FormatOption::RawValue(0), |acc, x| acc | *x);
        let c_path = CString::new(path).unwrap();
        let c_indent_string = CString::new(indent_string).unwrap();

        unsafe {
            match pugi_save_file(self.doc_ptr,
                                 c_path.as_ptr(),
                                 c_indent_string.as_ptr(),
                                 combined_format_options.value(),
                                 encoding) {
                1 => Ok(self),
                _ => Err(()),
            }
        }
    }
}
