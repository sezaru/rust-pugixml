use libc::c_void;

use std::cmp::Eq;

use std::hash::{Hash, Hasher};

use tree::NodeBase;

use wrapper::{pugi_delete_node, pugi_node_equal};

pub enum CNode {}

#[derive(Debug)]
pub struct Node {
    ptr: *mut CNode,
}

impl NodeBase for Node {
    fn get_ptr(&self) -> *mut c_void {
        self.ptr as *mut c_void
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe { pugi_delete_node(self.ptr) }
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        unsafe { pugi_node_equal(self.ptr, other.ptr) == 1 }
    }
}

impl Hash for Node {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.ptr.hash(state)
    }
}

impl Node {
    pub fn from_ptr(ptr: *mut CNode) -> Result<Self, ()> {
        if ptr.is_null() {
            Err(())
        } else {
            Ok(Node { ptr: ptr })
        }
    }
}
