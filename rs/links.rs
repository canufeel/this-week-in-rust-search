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
use std::collections::HashMap;
use guid_create::GUID;

static EQ_DICT: [&str; 6] = [
  "News & Blog Posts",
  "blog posts",
  "Announcements",
  "Blog Posts",
  "Blogosphere",
  "Notable Links"
];

type Slug = String;

#[derive(Clone, Serialize)]
pub struct Link {
  #[serde(skip_serializing)]
  pub url: String,
  pub text: String,
  pub slug: Slug,
  hits: u32,
  user_rating: i32,
}

#[derive(Clone, Serialize)]
pub struct LinksContainer {
  pub urls_to_links: HashMap<String, Link>,
  pub slugs_to_urls: HashMap<Slug, String>
}

impl LinksContainer {
  pub fn new() -> Self {
    LinksContainer {
      urls_to_links: HashMap::new(),
      slugs_to_urls: HashMap::new()
    }
  }

  pub fn filter_query(&self, query: &str) -> Vec<Link> {
    let lowercase_query = query.to_lowercase();
    let mut filtered_links = Vec::new();
    for link in self.urls_to_links.values() {
      if link.text.to_lowercase().contains(&lowercase_query) {
        filtered_links.push(link.clone());
      }
    }
    filtered_links
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
                  if !self.urls_to_links.contains_key(&l.url) {
                    self.slugs_to_urls.insert(l.slug.clone(), l.url.clone());
                    self.urls_to_links.insert(l.url.clone(), l);

                    found_num += 1;
                  }
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
        text,
        slug: GUID::rand().to_string(),
        hits: 0,
        user_rating: 0
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
