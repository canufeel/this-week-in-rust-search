mod links;
mod fetch;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use comrak::{parse_document, Arena, ComrakOptions};

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
    let links = links::LinksContainer::from(root);
    for ref link in links.links {
        println!("{}", link);
    }
}
