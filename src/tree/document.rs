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

#[derive(PartialEq)]
enum DataType {
    File,
    String,
    None,
}

pub struct DocumentBuilder {
    parse_options: Vec<ParseOption>,
    encoding: Encoding,
    data_type: DataType,
    data: String,
}

impl DocumentBuilder {
    pub fn new() -> DocumentBuilder {
        DocumentBuilder {
            parse_options: vec![],
            encoding: Encoding::Auto,
            data_type: DataType::None,
            data: String::new(),
        }
    }

    pub fn from_file(mut self, path: &str) -> DocumentBuilder {
        self.data_type = DataType::File;
        self.data = path.to_string();
        self
    }

    pub fn from_string(mut self, string: &str) -> DocumentBuilder {
        self.data_type = DataType::String;
        self.data = string.to_string();
        self
    }

    pub fn encoded_in(mut self, encoding: Encoding) -> DocumentBuilder {
        self.encoding = encoding;
        self
    }

    pub fn with_parse_option(mut self, parse_option: ParseOption) -> DocumentBuilder {
        self.parse_options.push(parse_option);
        self
    }

    pub fn finish(&self) -> Result<Document, ParseResult> {
        let combined_parse_options = if self.parse_options.is_empty() {
            ParseOption::Default
        } else {
            self.parse_options
                .iter()
                .fold(ParseOption::RawValue(0), |acc, x| acc | *x)
        };
        let c_data = CString::new(&self.data as &str).unwrap();

        let document = try!(Document::new());

        match self.data_type {
            DataType::File => {
                match ParseResult::new(unsafe {
                    pugi_load_file(document.ptr,
                                   c_data.as_ptr(),
                                   combined_parse_options.value(),
                                   self.encoding)
                }) {
                    Ok(result) => {
                        match result.status {
                            ParseStatus::Ok => Ok(document),
                            _ => Err(result),
                        }
                    }
                    _ => Err(ParseResult::default()),
                }
            }
            DataType::String => {
                match ParseResult::new(unsafe {
                    pugi_load_buffer(document.ptr,
                                     c_data.as_ptr(),
                                     self.data.len(),
                                     combined_parse_options.value(),
                                     self.encoding)
                }) {
                    Ok(result) => {
                        match result.status {
                            ParseStatus::Ok => Ok(document),
                            _ => Err(result),
                        }
                    }
                    _ => Err(ParseResult::default()),
                }
            }
            DataType::None => Document::new(),
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
