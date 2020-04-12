use std::process::{Command};

fn main() {
  if !Command::new("npm")
    .args(&["view", "yarn", "version", "--global"])
    .status()
    .unwrap()
    .success() {
    Command::new("npm").args(&["install", "-g", "yarn"]).status().unwrap();
  }
  Command::new("yarn").status().unwrap();
  Command::new("yarn").args(&["build", "--production"]).status().unwrap();
}