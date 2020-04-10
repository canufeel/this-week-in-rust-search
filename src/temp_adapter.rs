
pub trait TempAdapter {
  // path, url, content
  fn load(&self) -> Result<Option<(String, String, String)>, String>;
  fn update(&self) -> Result<bool, String>;
}

