

use helpers::from_c_char_to_string;
use libc::c_void;

use std::ffi::CString;

use tree::{Node, NodeKind, Attribute};

use tree::node::CNode;

use wrapper::*;

pub trait NodeBase {
    fn get_ptr(&self) -> *mut c_void;

    fn kind(&self) -> NodeKind {
        unsafe { pugi_node_type(self.get_ptr() as *mut CNode) }
    }

    fn name(&self) -> String {
        unsafe { from_c_char_to_string(pugi_node_name(self.get_ptr() as *mut CNode)) }
    }

    fn value(&self) -> String {
        unsafe { from_c_char_to_string(pugi_node_value(self.get_ptr() as *mut CNode)) }
    }

    fn text(&self) -> String {
        unsafe { from_c_char_to_string(pugi_node_text(self.get_ptr() as *mut CNode)) }
    }

    fn path(&self, delimiter: char) -> String {
        use libc::c_char;

        assert!((delimiter as u32) < 127,
                "Invalid C character value as delimiter");

        unsafe {
            from_c_char_to_string(pugi_node_path(self.get_ptr() as *mut CNode, delimiter as c_char))
        }
    }

    fn set_name(&mut self, name: &str) -> bool {
        let c_name = CString::new(name).unwrap();
        match unsafe { pugi_node_set_name(self.get_ptr() as *mut CNode, c_name.as_ptr()) } {
            1 => true,
            _ => false,
        }
    }

    fn set_value(&mut self, value: &str) -> bool {
        let c_value = CString::new(value).unwrap();
        match unsafe { pugi_node_set_value(self.get_ptr() as *mut CNode, c_value.as_ptr()) } {
            1 => true,
            _ => false,
        }
    }
}

pub fn parent(node: &NodeBase) -> Result<Node, ()> {
    Node::from_ptr(unsafe { pugi_node_parent(node.get_ptr() as *mut CNode) })
}

pub fn first_child(node: &NodeBase) -> Result<Node, ()> {
    Node::from_ptr(unsafe { pugi_node_first_child(node.get_ptr() as *mut CNode) })
}

pub fn last_child(node: &NodeBase) -> Result<Node, ()> {
    Node::from_ptr(unsafe { pugi_node_last_child(node.get_ptr() as *mut CNode) })
}

pub fn next_sibling(node: &NodeBase) -> Result<Node, ()> {
    Node::from_ptr(unsafe { pugi_node_next_sibling(node.get_ptr() as *mut CNode) })
}

pub fn previous_sibling(node: &NodeBase) -> Result<Node, ()> {
    Node::from_ptr(unsafe { pugi_node_previous_sibling(node.get_ptr() as *mut CNode) })
}

pub fn child_by_name(node: &NodeBase, name: &str) -> Result<Node, ()> {
    let c_name = CString::new(name).unwrap();

    Node::from_ptr(unsafe {
        pugi_node_child_by_name(node.get_ptr() as *mut CNode, c_name.as_ptr())
    })
}

pub fn next_sibling_by_name(node: &NodeBase, name: &str) -> Result<Node, ()> {
    let c_name = CString::new(name).unwrap();

    Node::from_ptr(unsafe {
        pugi_node_next_sibling_by_name(node.get_ptr() as *mut CNode, c_name.as_ptr())
    })
}

pub fn previous_sibling_by_name(node: &NodeBase, name: &str) -> Result<Node, ()> {
    let c_name = CString::new(name).unwrap();

    Node::from_ptr(unsafe {
        pugi_node_previous_sibling_by_name(node.get_ptr() as *mut CNode, c_name.as_ptr())
    })
}

pub fn find_child_by_attribute(node: &NodeBase, attribute: &Attribute) -> Result<Node, ()> {
    let c_attribute_name = CString::new(attribute.name()).unwrap();
    let c_attribute_value = CString::new(attribute.value()).unwrap();

    Node::from_ptr(unsafe {
        pugi_node_find_child_by_attribute(node.get_ptr() as *mut CNode,
                                          c_attribute_name.as_ptr(),
                                          c_attribute_value.as_ptr())
    })
}

pub fn find_child_by_name_and_attribute(node: &NodeBase, name: &str, attribute: &Attribute)
    -> Result<Node, ()> {
    let c_name = CString::new(name).unwrap();
    let c_attribute_name = CString::new(attribute.name()).unwrap();
    let c_attribute_value = CString::new(attribute.value()).unwrap();

    Node::from_ptr(unsafe {
        pugi_node_find_child_by_name_and_attribute(node.get_ptr() as *mut CNode,
                                                   c_name.as_ptr(),
                                                   c_attribute_name.as_ptr(),
                                                   c_attribute_value.as_ptr())
    })
}

pub fn first_element_by_path(node: &NodeBase, path: &str, delimiter: char) -> Result<Node, ()> {
    use libc::c_char;

    let c_path = CString::new(path).unwrap();

    assert!((delimiter as u32) < 127,
            "Invalid C character value as delimiter");

    Node::from_ptr(unsafe {
        pugi_node_first_element_by_path(node.get_ptr() as *mut CNode,
                                        c_path.as_ptr(),
                                        delimiter as c_char)
    })
}

pub fn root(node: &NodeBase) -> Result<Node, ()> {
    Node::from_ptr(unsafe { pugi_node_root(node.get_ptr() as *mut CNode) })
}

pub fn attribute(node: &NodeBase, attribute_name: &str) -> Result<Attribute, ()> {
    let c_attribute_name = CString::new(attribute_name).unwrap();

    Attribute::from_ptr(unsafe {
        pugi_node_attribute(node.get_ptr() as *mut CNode, c_attribute_name.as_ptr())
    })
}

pub fn append_attribute(node: &mut NodeBase, attribute: &Attribute) -> bool {
    let c_attribute_name = CString::new(attribute.name()).unwrap();
    let c_attribute_value = CString::new(attribute.value()).unwrap();

    match unsafe {
        pugi_node_append_attribute(node.get_ptr() as *mut CNode,
                                   c_attribute_name.as_ptr(),
                                   c_attribute_value.as_ptr())
    } {
        1 => true,
        _ => false,
    }
}

pub fn prepend_attribute(node: &mut NodeBase, attribute: &Attribute) -> bool {
    let c_attribute_name = CString::new(attribute.name()).unwrap();
    let c_attribute_value = CString::new(attribute.value()).unwrap();

    match unsafe {
        pugi_node_prepend_attribute(node.get_ptr() as *mut CNode,
                                    c_attribute_name.as_ptr(),
                                    c_attribute_value.as_ptr())
    } {
        1 => true,
        _ => false,
    }
}

pub fn append_child(node: &mut NodeBase, node_type: NodeKind) -> Result<Node, ()> {
    Node::from_ptr(unsafe { pugi_node_append_child(node.get_ptr() as *mut CNode, node_type) })
}

pub fn prepend_child(node: &mut NodeBase, node_type: NodeKind) -> Result<Node, ()> {
    Node::from_ptr(unsafe { pugi_node_prepend_child(node.get_ptr() as *mut CNode, node_type) })
}

pub fn append_copy(node: &mut NodeBase, proto: &NodeBase) -> Result<Node, ()> {
    Node::from_ptr(unsafe {
        pugi_node_append_copy(node.get_ptr() as *mut CNode, proto.get_ptr() as *mut CNode)
    })
}

pub fn prepend_copy(node: &mut NodeBase, proto: &NodeBase) -> Result<Node, ()> {
    Node::from_ptr(unsafe {
        pugi_node_prepend_copy(node.get_ptr() as *mut CNode, proto.get_ptr() as *mut CNode)
    })
}

pub fn remove_attribute(node: &mut NodeBase, attribute_name: &str) -> bool {
    let c_attribute_name = CString::new(attribute_name).unwrap();

    match unsafe {
        pugi_node_remove_attribute(node.get_ptr() as *mut CNode, c_attribute_name.as_ptr())
    } {
        1 => true,
        _ => false,
    }
}

pub fn remove_child(node: &mut NodeBase, to_be_removed_child: &NodeBase) -> bool {

    match unsafe {
        pugi_node_remove_child(node.get_ptr() as *mut CNode,
                               to_be_removed_child.get_ptr() as *mut CNode)
    } {
        1 => true,
        _ => false,
    }
}
