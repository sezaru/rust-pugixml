use libc::c_void;

use std::ffi::CString;

use tree::{ParseOption, ParseResult, Encoding, ParseStatus, FormatOption};

use tree::NodeBase;

use wrapper::*;

pub enum CDocument {}

pub struct Document {
    ptr: *mut CDocument,
}

impl NodeBase for Document {
    fn get_ptr(&self) -> *mut c_void {
        self.ptr as *mut c_void
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        unsafe {
            pugi_delete_document(self.ptr);
        }
    }
}

impl Document {
    fn new() -> Result<Self, ParseResult> {
        let document = unsafe { pugi_new_document() };
        if document.is_null() {
            Err(ParseResult::default())
        } else {
            Ok(Document { ptr: document })
        }
    }
}


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
