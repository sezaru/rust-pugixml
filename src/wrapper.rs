use libc::{c_int, c_char, c_uint, size_t, ptrdiff_t, c_double};

use tree::{ParseStatus, Encoding, NodeType};

use tree::node::CNode;
use tree::document::CDocument;
use tree::parse_result::CParseResult;

extern "C" {
    pub fn pugi_version() -> c_int;

    pub fn pugi_parse_flag_cdata() -> c_uint;
    pub fn pugi_parse_flag_comments() -> c_uint;
    pub fn pugi_parse_flag_declaration() -> c_uint;
    pub fn pugi_parse_flag_default() -> c_uint;
    pub fn pugi_parse_flag_doctype() -> c_uint;
    pub fn pugi_parse_flag_eol() -> c_uint;
    pub fn pugi_parse_flag_escapes() -> c_uint;
    pub fn pugi_parse_flag_fragment() -> c_uint;
    pub fn pugi_parse_flag_full() -> c_uint;
    pub fn pugi_parse_flag_minimal() -> c_uint;
    pub fn pugi_parse_flag_pi() -> c_uint;
    pub fn pugi_parse_flag_trim_pcdata() -> c_uint;
    pub fn pugi_parse_flag_ws_pcdata() -> c_uint;
    pub fn pugi_parse_flag_ws_pcdata_single() -> c_uint;
    pub fn pugi_parse_flag_wconv_attribute() -> c_uint;
    pub fn pugi_parse_flag_wnorm_attribute() -> c_uint;

    pub fn pugi_format_flag_default() -> c_uint;
    pub fn pugi_format_flag_indent() -> c_uint;
    pub fn pugi_format_flag_indent_attributes() -> c_uint;
    pub fn pugi_format_flag_no_empty_element_tags() -> c_uint;
    pub fn pugi_format_flag_no_declaration() -> c_uint;
    pub fn pugi_format_flag_no_escapes() -> c_uint;
    pub fn pugi_format_flag_raw() -> c_uint;
    pub fn pugi_format_flag_save_file_text() -> c_uint;
    pub fn pugi_format_flag_write_bom() -> c_uint;

    pub fn pugi_parse_result_status(parse_result: *const CParseResult) -> ParseStatus;
    pub fn pugi_parse_result_offset(parse_result: *const CParseResult) -> ptrdiff_t;
    pub fn pugi_parse_result_encoding(parse_result: *const CParseResult) -> Encoding;
    pub fn pugi_parse_result_description(parse_result: *const CParseResult) -> *const c_char;

    pub fn pugi_delete_parse_result(parse_result: *const CParseResult);

    pub fn pugi_new_document() -> *mut CDocument;
    pub fn pugi_delete_document(document: *mut CDocument);

    pub fn pugi_reset_document_with(document: *mut CDocument, proto: *mut CDocument);

    pub fn pugi_load_file(document: *mut CDocument, path: *const c_char, options: c_uint,
                      encoding: Encoding)
        -> *const CParseResult;

    pub fn pugi_load_buffer(document: *mut CDocument, contents: *const c_char, size: size_t,
                        options: c_uint, encoding: Encoding)
        -> *const CParseResult;

    pub fn pugi_save_file(document: *mut CDocument, path: *const c_char, indent: *const c_char,
                      options: c_uint, encoding: Encoding)
        -> c_int;

    pub fn pugi_delete_node(node: *mut CNode);

    pub fn pugi_node_equal(lhs: *mut CNode, rhs: *mut CNode) -> c_int;

    pub fn pugi_node_hash_value(node: *mut CNode) -> size_t;

    pub fn pugi_node_type(node: *mut CNode) -> NodeType;
    pub fn pugi_node_name(node: *mut CNode) -> *const c_char;
    pub fn pugi_node_value(node: *mut CNode) -> *const c_char;

    pub fn pugi_node_parent(node: *mut CNode) -> *mut CNode;
    pub fn pugi_node_child(node: *mut CNode, name: *const c_char) -> *mut CNode;

    pub fn pugi_node_first_child(node: *mut CNode) -> *mut CNode;
    pub fn pugi_node_last_child(node: *mut CNode) -> *mut CNode;
    pub fn pugi_node_next_sibling(node: *mut CNode) -> *mut CNode;
    pub fn pugi_node_previous_sibling(node: *mut CNode) -> *mut CNode;
    pub fn pugi_node_next_sibling_by_name(node: *mut CNode, name: *const c_char) -> *mut CNode;
    pub fn pugi_node_previous_sibling_by_name(node: *mut CNode, name: *const c_char) -> *mut CNode;

    pub fn pugi_node_find_child_by_name_and_attribute(node: *mut CNode, name: *const c_char,
                                                  attribute_name: *const c_char,
                                                  attribute_value: *const c_char)
        -> *mut CNode;
    pub fn pugi_node_find_child_by_attribute(node: *mut CNode, attribute_name: *const c_char,
                                         attribute_value: *const c_char)
        -> *mut CNode;

    pub fn pugi_node_child_value(node: *mut CNode) -> *const c_char;
    pub fn pugi_node_child_value_by_name(node: *mut CNode, name: *const c_char) -> *mut CNode;

    pub fn pugi_node_text(node: *mut CNode) -> *const c_char;

    // pub fn pugi_node_attribute(node: *mut CNode, name: *const c_char) -> *mut CAttribute;

    pub fn pugi_node_path(node: *mut CNode) -> *const c_char;

    pub fn pugi_node_first_element_by_path(node: *mut CNode, path: *const c_char, del: c_char)
        -> *mut CNode;

    pub fn pugi_node_root(node: *mut CNode) -> *mut CNode;

    pub fn pugi_node_set_name(node: *mut CNode, name: *const c_char) -> c_int;
    pub fn pugi_node_set_value(node: *mut CNode, name: *const c_char) -> c_int;

    pub fn pugi_node_append_attribute(node: *mut CNode, name: *const c_char, value: *const c_char)
        -> c_int;
    pub fn pugi_node_prepend_attribute(node: *mut CNode, name: *const c_char, value: *const c_char)
        -> c_int;

    pub fn pugi_node_append_child(node: *mut CNode, node_type: NodeType) -> *mut CNode;
    pub fn pugi_node_prepend_child(node: *mut CNode, node_type: NodeType) -> *mut CNode;

    pub fn pugi_node_append_copy(node: *mut CNode, proto: *mut CNode) -> *mut CNode;
    pub fn pugi_node_prepend_copy(node: *mut CNode, proto: *mut CNode) -> *mut CNode;

    pub fn pugi_node_remove_attribute(node: *mut CNode, name: *const c_char) -> c_int;
    pub fn pugi_node_remove_child(node: *mut CNode, to_be_removed_child: *mut CNode) -> c_int;

    // pub fn pugi_node_print(node: *mut CNode,

    // pub fn pugi_node_find_attribute_by_predicate
    // pub fn pugi_node_find_child_by_predicate
    // pub fn pugi_node_find_node_by_predicate

    // pub fn pugi_node_map_sibling
    // pub fn pugi_node_map_attributes

    // pub fn pugi_xpath_delete_xpath_node(node: *mut CXPathNode);

    // pub fn pugi_xpath_node_node(node: *mut CXPathNode) -> *mut CNode;
    // pub fn pugi_xpath_node_attribute(node: *mut CXPathNode) -> *mut CAttribute;

    // pub fn pugi_xpath_delete_xpath_node_set(node_set: *const CNodeSet);

    // pub fn pugi_xpath_node_set_size(node_set: *const CNodeSet) -> size_t;
    // pub fn pugi_xpath_node_set_is_empty(node_set: *const CNodeSet) -> c_int;

    // pub fn pugi_xpath_node_set_index(node_set: *mut CNodeSet, index: size_t) -> *mut CXPathNode;

    // pub fn pugi_xpath_node_set_map

    // pub fn pugi_xpath_delete_xpath_query(xpath: *const CXPath);

    // pub fn pugi_xpath_query_evaluate_boolean(xpath: *const CXPath, node: *mut CNode) -> c_int;
    // pub fn pugi_xpath_query_evaluate_number(xpath: *const CXPath, node: *mut CNode) -> c_double;
    // pub fn pugi_xpath_query_evaluate_string(xpath: *const CXPath, node: *mut CNode) -> *const c_char;
    // pub fn pugi_xpath_query_evaluate_node_set(xpath: *const CXPath, node: *mut CNode)
    //     -> *const CNodeSet;

    // pub fn pugi_xpath_query_return_type(xpath: *const CXPath) -> XPathValueType;

    // pub fn pugi_xpath_query_parse_is_success(xpath: *const CXPath) -> c_int;

    // pub fn pugi_xpath_select_single_node(node: *mut CNode, xpath: *const CXPath) -> *mut CXPathNode;
    // pub fn pugi_xpath_select_nodes(node: *mut CNode, xpath: *const CXPath) -> *const CNodeSet;
}
