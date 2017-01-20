use helpers::from_c_char_to_string;
use std::ffi::CString;

use std::hash::{Hash, Hasher};

use wrapper::*;

pub enum CAttribute {}

#[derive(Debug)]
pub struct Attribute {
    attribute_ptr: *mut CAttribute,
}

impl Drop for Attribute {
    fn drop(&mut self) {
        unsafe { pugi_delete_attr(self.attribute_ptr) }
    }
}

/// Check for Eq

impl Hash for Attribute {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.attribute_ptr.hash(state)
    }
}

impl Attribute {
    pub fn new() -> Result<Self, ()> {
        let attribute = unsafe { pugi_new_attribute() };
        if attribute.is_null() {
            Err(())
        } else {
            Ok(Attribute { attribute_ptr: attribute })
        }
    }

    pub fn from_ptr(attribute_ptr: *mut CAttribute) -> Result<Self, ()> {
        if attribute_ptr.is_null() {
            Err(())
        } else {
            Ok(Attribute { attribute_ptr: attribute_ptr })
        }
    }

    pub fn name(&self) -> String {
        unsafe { from_c_char_to_string(pugi_attr_name(self.attribute_ptr)) }
    }

    pub fn value(&self) -> String {
        unsafe { from_c_char_to_string(pugi_attr_value(self.attribute_ptr)) }
    }

    pub fn set_name(&mut self, name: &str) -> bool {
        let c_name = CString::new(name).unwrap();
        match unsafe { pugi_attr_set_name(self.attribute_ptr, c_name.as_ptr()) } {
            1 => true,
            _ => false,
        }
    }

    pub fn set_value(&mut self, value: &str) -> bool {
        let c_value = CString::new(value).unwrap();
        match unsafe { pugi_attr_set_value(self.attribute_ptr, c_value.as_ptr()) } {
            1 => true,
            _ => false,
        }
    }
}
