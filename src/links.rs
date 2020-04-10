use comrak::{
  nodes::{AstNode, NodeValue, NodeLink, Ast},
  arena_tree::Node
};
use std::{
  str,
  fmt::{self, Display},
  cell::RefCell
};

pub struct Link {
  pub url: String,
  pub text: String
}

pub struct LinksContainer {
  pub links: Vec<Link>
}

impl LinksContainer {
  fn extend_from_root<'a>(&mut self, root: &'a AstNode<'a>) {
    let mut node_section_lookup = false;
    for node in root.children() {
      let ast = node.data.borrow();
      if !node_section_lookup {
        match ast.value {
          NodeValue::Heading(_) => {
            if "News & Blog Posts" == str::from_utf8(&ast.content).unwrap() {
              node_section_lookup = true;
            }
          },
          _ => {}
        }
      } else {
        match ast.value {
          NodeValue::List(_) => {
            for item in node.children() {
              match Link::try_from_list_node(item) {
                Some(l) => self.links.push(l),
                None => {},
              }
            }
            break;
          },
          _ => {}
        }
      }
    }
  }
}

impl <'a> From<&'a AstNode<'a>> for LinksContainer {
  fn from(root: &'a AstNode<'a>) -> Self {
    let mut links_container = LinksContainer {
      links: Vec::new()
    };
    links_container.extend_from_root(root);
    links_container
  }
}

impl Link {
  fn try_from_list_node<'a>(item: &'a Node<'a, RefCell<Ast>>) -> Option<Self> {
    for paragraph_markdown in item.children() {
      let paragraph_markdown_ast = paragraph_markdown.data.borrow();
      let _paragraph_text = str::from_utf8(&paragraph_markdown_ast.content).unwrap();

      let mut url = String::new();
      let mut text = String::new();
      for child in paragraph_markdown.children() {
        match &child.data.borrow().value {
          NodeValue::Link(NodeLink { url: link_url, .. }) => {
            url.push_str(str::from_utf8(&link_url).unwrap());
            for link_child in child.children() {
              match &link_child.data.borrow().value {
                NodeValue::Text(contents) => {
                  text.push_str(str::from_utf8(&contents).unwrap());
                  break;
                },
                _ => {}
              }
            }
          },
          NodeValue::Text(contents) => {
            text.push_str(str::from_utf8(&contents).unwrap());
          }
          _ => {},
        }
      }
      return Some(Link {
        url,
        text
      });
    }
    None
  }
}

impl Display for Link {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Title: {}, Link: {}", self.text, self.url)
  }
}
