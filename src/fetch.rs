use tokio::runtime::{Runtime};
use futures::{stream, StreamExt};
use serde::Deserialize;
use std::str;
use base64::{decode, encode};
use reqwest::{Client, header, Response};
use log::{info};

use dotenv::dotenv;
use std::env;
use crate::temp_adapter::{TempAdapter, StorageItem};

static INITIAL_TREE_URL: &str = "https://api.github.com/repos/emberian/this-week-in-rust/git/trees/master";
static APP_USER_AGENT: &str = "This-Week-In-Rust-Search";
static GITHUB_CLIENT_KEY: &str = "GITHUB_CLIENT_KEY";
static GITHUB_CLIENT_SECRET: &str = "GITHUB_CLIENT_SECRET";

const PARALLEL_REQUESTS: usize = 10;

#[derive(Deserialize)]
struct TreeItem {
  path: String,
  url: String
}

#[derive(Deserialize)]
struct TreeResponse {
  tree: Vec<TreeItem>,
}

#[derive(Deserialize)]
struct DocPayload {
  content: String
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

pub struct Fetcher {
  client_key: String,
  client_secret: String,
  temp_adapter: Box<dyn TempAdapter>,
}

impl Fetcher {
  pub fn with_env (temp_adapter: Box<dyn TempAdapter>) -> Result<Self, String> {
    dotenv().ok();
    let client_key = env::var(GITHUB_CLIENT_KEY)
      .map_err(|e| e.to_string())?;
    let client_secret = env::var(GITHUB_CLIENT_SECRET)
      .map_err(|e| e.to_string())?;
    Ok(Fetcher {
      client_key,
      client_secret,
      temp_adapter
    })
  }

  async fn https_get (&self, url: &str) -> Result<Response, String> {
    let auth_header = format!("Basic {}", encode(format!("{}:{}", self.client_key, self.client_secret)));
    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/vnd.github.v3+json"));
    headers.insert(
      header::AUTHORIZATION,
      header::HeaderValue::from_str(&auth_header)
      .map_err(|e| e.to_string())?
    );
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

  // Vec<(url, path)>
  async fn get_tree (&self) -> Result<Vec<(String, String)>, String> {
    let tree_response: TreeResponse = self.https_get(INITIAL_TREE_URL)
      .await?
      .json::<TreeResponse>()
      .await
      .map_err(|e| e.to_string())?;
    match tree_response.tree
      .iter()
      .find(|tree_item| &tree_item.path == "content") {
      Some(item) => {
        let content_tree_response: TreeResponse = self.https_get(&item.url)
          .await?
          .json::<TreeResponse>()
          .await
          .map_err(|e| e.to_string())?;
        let md_only_vec = content_tree_response.tree
          .into_iter()
          .filter(|item| item.path.ends_with(".md"))
          .map(|i| (i.url, i.path))
          .collect::<Vec<(String, String)>>();
        let filtered_vec = if let Some(loaded) = self.temp_adapter.load_ids()? {
          info!("Loaded {} items from tmp", loaded.len());
          md_only_vec
            .into_iter()
            .filter(|(url, _)| !loaded.contains(url))
            .collect::<Vec<(String, String)>>()
        } else {
          info!("Loaded 0 items from tmp");
          md_only_vec
        };
        info!("Fetched details about {} items", filtered_vec.len());
        Ok(filtered_vec)
      },
      None => Err(String::from("Content not found")),
    }
  }

  async fn get_blob (&self, url: &str) -> Result<String, String> {
    self.https_get(url)
      .await?
      .json::<DocPayload>()
      .await
      .map(|r| str::replace(&r.content, "\n", "").to_owned())
      .map_err(|e| e.to_string())
  }


  fn fetch_save_all_base64_blobs (&self) -> Result<Vec<()>, String> {
    let mut rt = Runtime::new()
      .map_err(|e| e.to_string())?;

    rt.block_on(async {
      let urls = self.get_tree().await?;

      stream::iter(urls.into_iter().enumerate())
        .map(|(idx, (url, path))| async move {
          info!("About to get blob for item with idx: {}", idx);
          let res = self.get_blob(&url).await?;
          let item = StorageItem::new(url, path, res);
          info!("Storing item url: {}, path: {}", item.url, item.path);
          self.temp_adapter.update(item)?;
          Ok(())
        })
        .buffer_unordered(PARALLEL_REQUESTS)
        .collect::<Vec<Result<(), String>>>()
        .await
        .into_iter()
        .collect::<Result<Vec<()>, String>>()
    })
  }

  pub fn get_all_file_contents (&self) -> Result<Vec<StorageItem>, String> {
    self.fetch_save_all_base64_blobs()?;
    if let Some(blobs) = self.temp_adapter.load()? {
      let (urls_and_paths, string_blobs): (Vec<(String, String)>, Vec<String>) = blobs
        .into_iter()
        .map(| StorageItem { url, content, path }| ((url, path), content))
        .unzip();
      let parsed_blobs = parse_base64_blobs(string_blobs)?;
      Ok(
        urls_and_paths
          .into_iter()
          .zip(parsed_blobs).map(|((url, path), content)| StorageItem::new(url, path, content))
          .collect()
      )
    } else {
      Err("no records found".to_owned())
    }
  }
}