use tokio::runtime::{Runtime};
use futures::{stream, StreamExt, future};
use serde::Deserialize;
use std::str;
use base64::decode;
use std::error::Error;

static INITIAL_TREE_URL: &str = "https://api.github.com/repos/emberian/this-week-in-rust/git/trees/master";
const PARALLEL_REQUESTS: usize = 4;

#[derive(Deserialize)]
struct TreeItem {
  path: String,
  mode: String,
  tree_type: String,
  sha: String,
  size: u32,
  url: String
}

#[derive(Deserialize)]
struct TreeResponse {
  sha: String,
  url: String,
  tree: Vec<TreeItem>,
  truncated: bool,
}

#[derive(Deserialize)]
struct DocPayload {
  content: String
}

async fn get_tree () -> Result<Vec<String>, String> {
  let tree_response: TreeResponse = reqwest::get(INITIAL_TREE_URL)
    .await
    .map_err(|e| e.to_string())?
    .json::<TreeResponse>()
    .await
    .map_err(|e| e.to_string())?;
  match tree_response.tree.iter().find(|tree_item| &tree_item.tree_type == "content") {
    Some(item) => {
      let content_tree_response: TreeResponse = reqwest::get(&item.url)
        .await
        .map_err(|e| e.to_string())?
        .json::<TreeResponse>()
        .await
        .map_err(|e| e.to_string())?;
      let filtered_vec = content_tree_response.tree
        .into_iter()
        .filter(|item| item.path.ends_with(".md"))
        .map(|i| i.url)
        .collect::<Vec<String>>();
      Ok(filtered_vec)
    },
    None => Err(String::from("Content not found")),
  }
}

async fn get_blob (url: &str) -> Result<String, String> {
  reqwest::get(url)
    .await
    .map_err(|e| e.to_string())?
    .json::<DocPayload>()
    .await
    .map(|r| str::replace(&r.content, "\n", "").to_owned())
    .map_err(|e| e.to_string())
}

fn get_all_base64_blobs () -> Result<Vec<String>, String> {
  let mut rt = Runtime::new()
    .map_err(|e| e.to_string())?;

  rt.block_on(async {
    let urls = get_tree().await?;

    stream::iter(urls)
      .map(|url| async move {
        get_blob(&url).await
      })
      .buffer_unordered(PARALLEL_REQUESTS)
      .collect::<Vec<Result<String, String>>>()
      .await
      .into_iter()
      .collect::<Result<Vec<String>, String>>()
  })
}

fn parse_base64_blobs (blobs: Vec<String>) -> Result<Vec<String>, String> {
  let parser = |b: String| -> Result<String, String> {
    let bytes = decode(b)
      .map_err(|e| e.to_string())?;
    let str = str::from_utf8(&bytes)
      .map_err(|e| e.to_string())?;
    Ok(str.to_owned())
  };
  blobs
    .into_iter()
    .map(parser)
    .collect::<Result<Vec<String>, String>>()
}