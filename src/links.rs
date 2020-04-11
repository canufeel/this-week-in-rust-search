use comrak::{
  nodes::{AstNode, NodeValue, NodeLink, Ast},
  arena_tree::Node
};
use serde::Serialize;
use std::{
  str,
  fmt::{self, Display},
  cell::RefCell
};
use log::{warn, debug};

use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde_json;

static EQ_DICT: [&str; 6] = [
  "News & Blog Posts",
  "blog posts",
  "Announcements",
  "Blog Posts",
  "Blogosphere",
  "Notable Links"
];

#[derive(Clone, Serialize)]
pub struct Link {
  pub url: String,
  pub text: String
}

#[derive(Clone, Serialize)]
pub struct LinksContainer {
  pub links: Vec<Link>
}

impl LinksContainer {
  pub fn new() -> Self {
    LinksContainer {
      links: Vec::new()
    }
  }

  pub fn filter_query(&self, query: &str) -> Self {
    LinksContainer {
      links: self.links
        .iter()
        .filter(|Link { text, .. }| text.contains(query))
        .cloned()
        .collect::<Vec<Link>>()
    }
  }

  pub fn extend_from_root<'a>(
    &mut self,
    root: &'a AstNode<'a>,
    container_id: String
  ) -> bool {
    let mut node_section_lookup = false;
    for node in root.children() {
      let ast = node.data.borrow();
      if !node_section_lookup {
        match ast.value {
          NodeValue::Heading(_) => {
            let heading_content = str::from_utf8(&ast.content).unwrap();
            for comp in EQ_DICT.iter() {
              if heading_content.contains(*comp)  {
                node_section_lookup = true;
                break;
              }
            }
          },
          _ => {}
        }
      } else {
        match ast.value {
          NodeValue::List(_) => {
            let mut found_num = 0;
            for item in node.children() {
              match Link::try_from_list_node(item) {
                Some(l) => {
                  self.links.push(l);
                  found_num += 1;
                },
                None => {},
              }
            }
            if found_num > 0 {
              debug!("List parsing for id: {} found {} links", container_id, found_num);
              return true;
            }
            warn!("List parsing failed for id: {}", container_id);
            break;
          },
          _ => {}
        }
      }
    }
    warn!("Heading not found for id: {}", container_id);
    false
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

// Responder
impl Responder for LinksContainer {
  type Error = Error;
  type Future = Ready<Result<HttpResponse, Error>>;

  fn respond_to(self, _req: &HttpRequest) -> Self::Future {
    let body = serde_json::to_string(&self).unwrap();

    // Create response and set content type
    ready(Ok(HttpResponse::Ok()
      .content_type("application/json")
      .body(body)))
  }
}
