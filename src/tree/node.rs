use wrapper::pugi_delete_node;

use tree::NodeBase;


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


impl Node {
    pub fn from_ptr(ptr: *mut CNode) -> Result<Self, ()> {
        if ptr.is_null() {
            Err(())
        } else {
            Ok(Node { ptr: ptr })
        }
    }
}
