use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use comrak::{parse_document, Arena, ComrakOptions};
use comrak::nodes::{AstNode, NodeValue, NodeLink};
use std::str;
use std::fmt::{self, Display};

struct Link {
    pub url: String,
    pub text: String
}

impl Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {}, Link: {}", self.text, self.url)
    }
}

fn find_nodes <'a>(root: &'a AstNode<'a>) -> Vec<Link> {
    let children = root.children();
    let mut node_section_lookup = false;
    let mut link_list: Vec<Link> = Vec::new();
      for node in children {
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
                        for paragraph_markdown in item.children() {
                            let paragraph_markdown_ast = paragraph_markdown.data.borrow();
                            let paragraph_text = str::from_utf8(&paragraph_markdown_ast.content).unwrap();

                            let mut url = String::new();
                            let mut text = String::new();
                            for child in paragraph_markdown.children() {
                                match &child.data.borrow().value {
                                    NodeValue::Link(NodeLink { url: link_url, title }) => {
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
                            link_list.push(
                                Link {
                                    url,
                                    text
                                }
                            );
                        }
                    }
                    break;
                },
                _ => {}
            }
        }
    }
    link_list
}

fn main() {
    // Create a path to the desired file
    let path = Path::new("2020-04-07-this-week-in-rust.md");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.description()),
        Ok(file) => file,
    };

    let arena = Arena::new();
    let mut s = String::new();
    file.read_to_string(& mut s).unwrap();
    let root = parse_document(
        &arena,
        &s,
        &ComrakOptions::default()
    );
    let links = find_nodes(root);
    for ref link in links {
        println!("{}", link);
    }
}
