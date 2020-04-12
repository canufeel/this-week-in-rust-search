use std::process::Command;

fn main() {
  Command::new("yarn").status().unwrap();
  Command::new("yarn").args(&["build"]).status().unwrap();
}