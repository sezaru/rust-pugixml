#include <cstring>
#include <pugixml.hpp>
#include <sstream>
#include <stdlib.h>
#include <string>

extern "C" {
typedef pugi::xml_document Document;
typedef pugi::xml_node Node;
typedef pugi::xml_attribute Attr;
typedef pugi::xml_parse_result ParseResult;
typedef pugi::xpath_query XPath;
typedef pugi::xpath_node XPathNode;
typedef pugi::xpath_node_set NodeSet;
}

namespace detail
{
  inline Node * check_new_node(const Node & n)
  {
    if (n)
    {
      return new Node(n);
    }
    else
    {
      return nullptr;
    }
  }

  inline Attr * check_new_attribute(const Attr & a)
  {
    if (a)
    {
      return new Attr(a);
    }
    else
    {
      return nullptr;
    }
  }

  inline XPathNode * check_new_xpath_node(const XPathNode & n)
  {
    if (n)
    {
      return new XPathNode(n);
    }
    else
    {
      return nullptr;
    }
  }

  struct wrap_writer : pugi::xml_writer
  {
    void (*func)(const char * data, size_t size);

    virtual void write(const void * data, size_t size)
    {
      func(static_cast<const char *>(data), size);
    }
  };

  typedef struct predicate
  {
    bool (*attr_pred)(const Attr *);
    bool (*node_pred)(const Node *);

    bool operator()(pugi::xml_attribute attr) const { return attr_pred(&attr); }

    bool operator()(pugi::xml_node node) const { return node_pred(&node); }
  } predicate_t;
}

extern "C" {

int pugi_version() { return PUGIXML_VERSION; }

// formatting option bit flags
unsigned int pugi_formatflag_default() { return pugi::format_default; }
unsigned int pugi_formatflag_indent() { return pugi::format_indent; }
unsigned int pugi_formatflag_indent_attributes() { return pugi::format_indent_attributes; }
unsigned int pugi_formatflag_no_empty_element_tags() { return pugi::format_no_empty_element_tags; }
unsigned int pugi_formatflag_no_declaration() { return pugi::format_no_declaration; }
unsigned int pugi_formatflag_no_escapes() { return pugi::format_no_escapes; }
unsigned int pugi_formatflag_raw() { return pugi::format_raw; }
unsigned int pugi_formatflag_save_file_text() { return pugi::format_save_file_text; }
unsigned int pugi_formatflag_write_bom() { return pugi::format_write_bom; }

// parsing option bit flags
unsigned int pugi_parse_flag_cdata() { return pugi::parse_cdata; }
unsigned int pugi_parse_flag_comments() { return pugi::parse_comments; }
unsigned int pugi_parse_flag_declaration() { return pugi::parse_declaration; }
unsigned int pugi_parse_flag_default() { return pugi::parse_default; }
unsigned int pugi_parse_flag_doctype() { return pugi::parse_doctype; }
unsigned int pugi_parse_flag_eol() { return pugi::parse_eol; }
unsigned int pugi_parse_flag_escapes() { return pugi::parse_escapes; }
unsigned int pugi_parse_flag_fragment() { return pugi::parse_fragment; }
unsigned int pugi_parse_flag_full() { return pugi::parse_full; }
unsigned int pugi_parse_flag_minimal() { return pugi::parse_minimal; }
unsigned int pugi_parse_flag_pi() { return pugi::parse_pi; }
unsigned int pugi_parse_flag_trim_pcdata() { return pugi::parse_trim_pcdata; }
unsigned int pugi_parse_flag_ws_pcdata() { return pugi::parse_ws_pcdata; }
unsigned int pugi_parse_flag_ws_pcdata_single() { return pugi::parse_ws_pcdata_single; }
unsigned int pugi_parse_flag_wconv_attribute() { return pugi::parse_wconv_attribute; }
unsigned int pugi_parse_flag_wnorm_attribute() { return pugi::parse_wnorm_attribute; }

// methods of document
Document * pugi_new_document() { return new pugi::xml_document(); }
void pugi_delete_document(const Document * doc) { delete doc; }

void pugi_reset_document_with(Document * doc, const Document * proto) { doc->reset(*proto); }

ParseResult * pugi_load_buffer(Document * doc, const void * str, size_t size, unsigned int options,
                               pugi::xml_encoding encoding)
{
  return new pugi::xml_parse_result(doc->load_buffer(str, size, options, encoding));
}

ParseResult * pugi_load_file(Document * doc, const char * path, unsigned int options,
                             pugi::xml_encoding encoding)
{
  return new pugi::xml_parse_result(doc->load_file(path, options, encoding));
}

int pugi_save_file(const Document * doc, const char * path, const char * indent, unsigned int flags,
                   pugi::xml_encoding encoding)
{
  return doc->save_file(path, indent, flags, encoding);
}

void pugi_save_string(const Document * doc, void (*func)(const char *, size_t), const char * indent,
                      unsigned int flags, pugi::xml_encoding encoding)
{
  detail::wrap_writer wtr;
  wtr.func = func;
  doc->save(wtr, indent, flags, encoding);
}

Node * pugi_document_element(const Document * doc)
{
  return detail::check_new_node(doc->document_element());
}

////// methods of xml_parse_result
void pugi_delete_parse_result(const ParseResult * r) { delete r; }
int pugi_parse_is_success(const ParseResult * r) { return *r ? true : false; }
pugi::xml_parse_status pugi_parse_result_status(const ParseResult * r) { return r->status; }
ptrdiff_t pugi_parse_result_offset(const ParseResult * r) { return r->offset; }
pugi::xml_encoding pugi_parse_result_encoding(const ParseResult * r) { return r->encoding; }
const char * pugi_parse_result_description(const ParseResult * r) { return r->description(); }

////// methods of xml_attribute
void pugi_delete_attr(const Attr * a) { delete a; }
size_t pugi_attr_hash_value(const Attr * a) { return a->hash_value(); }

const char * pugi_attr_name(const Attr * a) { return a->name(); }
const char * pugi_attr_value(const Attr * a) { return a->value(); }

int pugi_attr_set_value(Attr * a, const char * v) { return a->set_value(v); }

////// methods of node
void pugi_delete_node(const Node * n) { delete n; }

int pugi_node_equal(const Node * a, const Node * b) { return *a == *b; }

size_t pugi_node_hash_value(const Node * n) { return n->hash_value(); }

pugi::xml_node_type pugi_node_type(const Node * n) { return n->type(); }

const char * pugi_node_name(const Node * n) { return n->name(); }
const char * pugi_node_value(const Node * n) { return n->value(); }

Node * pugi_node_parent(const Node * n) { return detail::check_new_node(n->parent()); }
Node * pugi_node_first_child(const Node * n) { return detail::check_new_node(n->first_child()); }
Node * pugi_node_last_child(const Node * n) { return detail::check_new_node(n->last_child()); }
Node * pugi_node_next_sibling(const Node * n) { return detail::check_new_node(n->next_sibling()); }
Node * pugi_node_previous_sibling(const Node * n)
{
  return detail::check_new_node(n->previous_sibling());
}

Node * pugi_node_child(const Node * n, const char * name)
{
  return detail::check_new_node(n->child(name));
}
Attr * pugi_node_attribute(const Node * n, const char * name)
{
  return detail::check_new_attribute(n->attribute(name));
}
Node * pugi_node_next_sibling_by_name(const Node * n, const char * name)
{
  return detail::check_new_node(n->next_sibling(name));
}
Node * pugi_node_previous_sibling_by_name(const Node * n, const char * name)
{
  return detail::check_new_node(n->previous_sibling(name));
}
Node * pugi_node_find_child_by_name_and_attribute(const Node * n, const char * name,
                                                  const char * attr_name, const char * attr_value)
{
  return detail::check_new_node(n->find_child_by_attribute(name, attr_name, attr_value));
}
Node * pugi_node_find_child_by_attribute(const Node * n, const char * attr_name,
                                         const char * attr_value)
{
  return detail::check_new_node(n->find_child_by_attribute(attr_name, attr_value));
}

const char * pugi_node_child_value(const Node * n) { return n->child_value(); }
const char * pugi_node_child_value_by_name(const Node * n, const char * name)
{
  return n->child_value(name);
}
const char * pugi_node_text(const Node * n) { return n->text().get(); }

bool pugi_default_attr_pred(const Attr *) { return true; }
bool pugi_default_node_pred(const Node *) { return true; }

Attr * pugi_node_find_attribute_by_predicate(const Node * node, bool (*attr_pred)(const Attr *))
{
  detail::predicate_t pred;
  pred.attr_pred = attr_pred;
  pred.node_pred = &pugi_default_node_pred;
  return detail::check_new_attribute(node->find_attribute(pred));
}

Node * pugi_node_find_child_by_predicate(const Node * node, bool(node_pred)(const Node *))
{
  detail::predicate_t pred;
  pred.attr_pred = pugi_default_attr_pred;
  pred.node_pred = node_pred;
  return detail::check_new_node(node->find_child(pred));
}

Node * pugi_node_find_node_by_predicate(const Node * node, bool(node_pred)(const Node *))
{
  detail::predicate_t pred;
  pred.attr_pred = pugi_default_attr_pred;
  pred.node_pred = node_pred;
  return detail::check_new_node(node->find_node(pred));
}

void pugi_node_map_sibling(const Node * node, void (*fun)(Node *))
{
  for (pugi::xml_node_iterator it = node->begin(); it != node->end(); ++it)
  {
    fun(&*it);
  }
}
void pugi_node_map_attributes(const Node * node, void (*fun)(Attr *))
{
  for (pugi::xml_attribute_iterator it = node->attributes_begin(); it != node->attributes_end();
       ++it)
  {
    fun(&*it);
  }
}

char * pugi_node_path(const Node * node, const char del)
{
  std::string s = node->path(del);
  char * ret = (char *)malloc(s.length() + 1);
  std::strcpy(ret, s.c_str());
  return ret;
}

Node * pugi_node_first_element_by_path(const Node * node, const char * path, char del)
{
  return detail::check_new_node(node->first_element_by_path(path, del));
}

Node * pugi_node_root(const Node * n) { return detail::check_new_node(n->root()); }

int pugi_node_set_name(Node * n, const char * name) { return n->set_name(name); }
int pugi_node_set_value(Node * n, const char * name) { return n->set_value(name); }

int pugi_node_append_attribute(Node * n, const char * name, const char * val)
{
  return n->append_attribute(name).set_value(val);
}
int pugi_node_prepend_attribute(Node * n, const char * name, const char * val)
{
  return n->append_attribute(name).set_value(val);
}

Node * pugi_node_append_child(Node * n, pugi::xml_node_type typ)
{
  return detail::check_new_node(n->append_child(typ));
}
Node * pugi_node_prepend_child(Node * n, pugi::xml_node_type typ)
{
  return detail::check_new_node(n->prepend_child(typ));
}

Node * pugi_node_append_copy(Node * n, const Node * proto)
{
  return detail::check_new_node(n->append_copy(*proto));
}
Node * pugi_node_prepend_copy(Node * n, const Node * proto)
{
  return detail::check_new_node(n->prepend_copy(*proto));
}

int pugi_node_remove_attribute(Node * n, const char * name) { return n->remove_attribute(name); }
int pugi_node_remove_child(Node * n, const Node * cld) { return n->remove_child(*cld); }

ParseResult * pugi_node_append_buffer(Node * node, const void * cont, size_t size,
                                      unsigned int options, pugi::xml_encoding enc)
{
  return new pugi::xml_parse_result(node->append_buffer(cont, size, options, enc));
}

void pugi_node_print(const Node * node, void (*func)(const char *, size_t), const char * indent,
                     unsigned int flags, pugi::xml_encoding encoding, unsigned int depth)
{
  detail::wrap_writer wtr;
  wtr.func = func;
  node->print(wtr, indent, flags, encoding, depth);
}

XPathNode * pugi_xpath_select_single_node(const Node * node, const XPath * x)
{
  return detail::check_new_xpath_node(node->select_single_node(*x));
}
NodeSet * pugi_xpath_select_nodes(const Node * node, const XPath * x)
{
  return new NodeSet(node->select_nodes(*x));
}

////// methods of xpath_node
void pugi_xpath_delete_xpath_node(const XPathNode * p) { delete p; }
Node * pugi_xpath_node_node(const XPathNode * p) { return detail::check_new_node(p->node()); }
Attr * pugi_xpath_node_attribute(const XPathNode * p)
{
  return detail::check_new_attribute(p->attribute());
}

////// methods of xpath_node_set
void pugi_xpath_delete_xpath_node_set(const NodeSet * p) { delete p; }
size_t pugi_xpath_node_set_size(const NodeSet * p) { return p->size(); }
int pugi_xpath_node_set_is_empty(const NodeSet * p) { return p->empty(); }
XPathNode * pugi_xpath_node_set_index(const NodeSet * p, size_t i)
{
  return detail::check_new_xpath_node((*p)[i]);
}
void pugi_xpath_node_set_map(const NodeSet * p, void (*func)(const XPathNode *))
{
  for (pugi::xpath_node_set::const_iterator it = p->begin(); it != p->end(); ++it)
  {
    func(&*it);
  }
}

////// methods of xpath_query
void pugi_xpath_delete_xpath_query(const XPath * p) { delete p; }
XPath * pugi_new_xpath_query_no_variable(const char * query)
{
  return new pugi::xpath_query(query);
}

int pugi_xpath_query_evaluate_boolean(const XPath * p, const Node * n)
{
  XPathNode xn(*n);
  return p->evaluate_boolean(xn);
}
double pugi_xpath_query_evaluate_number(const XPath * p, const Node * n)
{
  XPathNode xn(*n);
  return p->evaluate_number(xn);
}

char * pugi_xpath_query_evaluate_string(const XPath * p, const Node * n)
{
  XPathNode xn(*n);
  std::string s = p->evaluate_string(xn);
  char * ret = (char *)malloc(s.length() + 1);
  std::strcpy(ret, s.c_str());
  return ret;
}

NodeSet * pugi_xpath_query_evaluate_node_set(const XPath * p, const Node * n)
{
  XPathNode xn(*n);
  return new NodeSet(p->evaluate_node_set(xn));
}

pugi::xpath_value_type pugi_xpath_query_return_type(const XPath * p) { return p->return_type(); }
int pugi_xpath_query_parse_is_success(const XPath * p) { return static_cast<bool>(*p); }
}
