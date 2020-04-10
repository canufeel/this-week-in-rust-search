mod links;
mod fetch;
mod temp_adapter;
mod fs_adapter;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use comrak::{parse_document, Arena, ComrakOptions};
use comrak::nodes::AstNode;

fn main() -> Result<(), String> {
  /*// Create a path to the desired file
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


  let mut s = String::new();
  file.read_to_string(& mut s).unwrap();*/

  let temp_adapter = Box::new(fs_adapter::FsAdapter::new());
  let fetcher = fetch::Fetcher::with_env(temp_adapter)?;
  let all_contents = fetcher.get_all_file_contents()?;

  let arena = Arena::new();
  let roots = all_contents
    .into_iter()
    .map(|(url, s)| (
      url,
      parse_document(
        &arena,
        &s,
        &ComrakOptions::default(),
      )
    )
    ).collect::<Vec<(String, &AstNode)>>();

  let mut links_container = links::LinksContainer::new();

  for (url, root) in roots {
    if !links_container.extend_from_root(root) {
      println!("Not found for url: {}", &url);
    }
  }

  for ref link in links_container.links {
    println!("{}", link);
  }
  Ok(())
}
