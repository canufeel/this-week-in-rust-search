use crate::temp_adapter::{TempAdapter, StorageItem};
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashSet;

static FS_PATH: &str = "temp.b";

pub struct FsAdapter <'a> {
  path: &'a Path
}



impl <'a> FsAdapter <'a> {
  pub fn new() -> Self {
    FsAdapter { path: Path::new(FS_PATH) }
  }
}

impl <'a> TempAdapter for FsAdapter <'a> {
  fn load(&self) -> Result<Option<Vec<StorageItem>>, String> {
    let mut file = match File::open(self.path) {
      Ok(f) => f,
      Err(_) => {
        return Ok(None)
      }
    };

    let mut unparsed = Vec::new();
    file
      .read_to_end(&mut unparsed)
      .map_err(|e| e.to_string())?;
    let parsed = bincode::deserialize::<Vec<StorageItem>>(&unparsed)
      .map_err(|e| e.to_string())?;
    Ok(
      Some(
        parsed
      )
    )
  }

  fn load_ids(&self) -> Result<Option<HashSet<String>>, String> {
    let loaded = self.load()?;
    Ok(match loaded {
      Some(items) => {
        let mut set = HashSet::new();
        items.into_iter().for_each(|StorageItem { url, .. }| { set.insert(url); });
        Some(set)
      },
      None => None
    })
  }

  fn update(&self, write_item: StorageItem) -> Result<bool, String> {
    let load_result = match self.load()? {
      Some(mut l) => {
        // if item with the same path exists (usually this means the same name) then we exclude it
        l = l.into_iter().filter(|i| i.path != write_item.path).collect();
        l.push(write_item);
        l
      },
      None => {
        vec![write_item]
      }
    };
    let mut file = File::create(self.path)
      .map_err(|e| e.to_string())?;
    let serialized = bincode::serialize(&load_result)
      .map_err(|e| e.to_string())?;
    file.write(&serialized).map_err(|e| e.to_string())?;
    Ok(true)
  }
}

