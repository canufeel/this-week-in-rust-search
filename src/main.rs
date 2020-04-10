mod links;
mod fetch;
mod temp_adapter;
mod fs_adapter;
mod logger;

use comrak::{parse_document, Arena, ComrakOptions};
use comrak::nodes::AstNode;
use log::{info, warn};
use temp_adapter::StorageItem;

fn parse_path_date (path: &String) -> String {
  path[0..10].to_owned()
}

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

  logger::init().map_err(|e| e.to_string())?;
  info!("Logger setup success");
  let temp_adapter = Box::new(fs_adapter::FsAdapter::new());
  let fetcher = fetch::Fetcher::with_env(temp_adapter)?;
  let all_contents = fetcher.get_all_file_contents()?;

  let arena = Arena::new();
  let roots = all_contents
    .into_iter()
    .map(|StorageItem { url, path, content }| (
      url,
      path,
      parse_document(
        &arena,
        &content,
        &ComrakOptions::default(),
      )
    )
    ).collect::<Vec<(String, String, &AstNode)>>();

  let mut links_container = links::LinksContainer::new();

  for (url, path, root) in roots {
    if !links_container.extend_from_root(root, parse_path_date(&path)) {
      warn!("Not found for url: {} and path: {}", &url, &path);
    }
  }

  for ref link in &links_container.links {
    info!("{}", *link);
  }
  info!("Total links found: {}", links_container.links.len());
  Ok(())
}
