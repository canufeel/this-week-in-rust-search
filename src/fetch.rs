use tokio::runtime::{Runtime};
use tokio::time::delay_for;
use futures::{stream, StreamExt, future};
use serde::Deserialize;
use std::str;
use base64::decode;
use std::error::Error;
use std::time::Duration;
use reqwest::{Client, header, Response};

static INITIAL_TREE_URL: &str = "https://api.github.com/repos/emberian/this-week-in-rust/git/trees/master";
static APP_USER_AGENT: &str = "This-Week-In-Rust-Search";
const PARALLEL_REQUESTS: usize = 4;

#[derive(Deserialize)]
struct TreeItem {
  path: String,
  #[serde(rename(deserialize = "type"))]
  tree_type: String,
  sha: String,
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

async fn https_get (url: &str) -> Result<Response, String> {
  let mut headers = header::HeaderMap::new();
  headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/vnd.github.v3+json"));
  Client::builder()
    .user_agent(APP_USER_AGENT)
    .default_headers(headers)
    .build()
    .map_err(|e| e.to_string())?
    .get(url)
    .send()
    .await
    .map_err(|e| e.to_string())
}

async fn get_tree () -> Result<Vec<String>, String> {
  let tree_response: TreeResponse = https_get(INITIAL_TREE_URL)
    .await?
    .json::<TreeResponse>()
    .await
    .map_err(|e| e.to_string())?;
  match tree_response.tree
    .iter()
    .find(|tree_item| &tree_item.path == "content") {
    Some(item) => {
      let content_tree_response: TreeResponse = https_get(&item.url)
        .await?
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
  https_get(url)
    .await?
    .json::<DocPayload>()
    .await
    .map(|r| str::replace(&r.content, "\n", "").to_owned())
    .map_err(|e| e.to_string())
}

fn get_all_base64_blobs () -> Result<Vec<(String, String)>, String> {
  let mut rt = Runtime::new()
    .map_err(|e| e.to_string())?;

  rt.block_on(async {
    let urls = get_tree().await?;

    stream::iter(urls)
      .map(|url| async move {
        delay_for(Duration::from_secs(1)).await;
        let res =  get_blob(&url).await?;
        Ok((url, res))
      })
      .buffer_unordered(PARALLEL_REQUESTS)
      .collect::<Vec<Result<(String, String), String>>>()
      .await
      .into_iter()
      .collect::<Result<Vec<(String, String)>, String>>()
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

pub fn get_all_file_contents () -> Result<Vec<(String, String)>, String> {
  let blobs = get_all_base64_blobs()?;
  let (urls, string_blobs): (Vec<String>, Vec<String>) = blobs
    .into_iter()
    .unzip();
  let parsed_blobs = parse_base64_blobs(string_blobs)?;
  Ok(
    urls
      .into_iter()
      .zip(parsed_blobs)
      .collect()
  )
}