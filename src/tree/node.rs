use wrapper::pugi_delete_node;

pub enum CNode {}

#[derive(Debug)]
pub struct Node {
    node_ptr: *mut CNode,
}

impl Drop for Node {
    fn drop(&mut self) {
        unsafe {
            pugi_delete_node(self.node_ptr);
        }
    }
}
