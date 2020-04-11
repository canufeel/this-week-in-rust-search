use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize)]
pub struct StorageItem {
  pub url: String,
  pub path: String,
  pub content: String
}

impl StorageItem {
  pub fn new(url: String, path: String, content: String) -> Self {
    StorageItem {
      url,
      path,
      content
    }
  }
}

pub trait TempAdapter {
  // path, url, content
  fn load(&self) -> Result<Option<Vec<StorageItem>>, String>;
  fn load_ids(&self) -> Result<Option<HashSet<String>>, String>;
  fn update(&self, write_item: StorageItem) -> Result<bool, String>;
}

