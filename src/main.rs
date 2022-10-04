use std::fs::{read_dir, read_to_string, File};
use std::io::Read;

use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;

fn main() {
  let path = std::env::current_dir()
    .expect("Error getting current working directory")
    .as_path()
    .join(std::path::Path::new("assets/curves"));
  let files = get_files(path.as_path().to_str().unwrap());
  // files.iter().for_each(|f| println!("{}", f));

  let file_content = read_file(files[2].as_str());
  file_content.iter().for_each(|line| println!("{line}"));
}

fn get_files(path: &str) -> Vec<String> {
  let files = read_dir(path);
  if files.is_ok() {
    files.unwrap()
      .map(
        |file| file.unwrap()
          .path()
          .display()
          .to_string())
      .collect::<Vec<String>>()
    } else {
      Vec::new()
    }
}

fn read_file(path: &str) -> Vec<String> {
  let mut file_content = read_to_string(path);
  if file_content.is_err() {
    file_content = read_file_alt(path);
  }
  if file_content.is_ok() {
    file_content
      .unwrap()
      .lines()
      .map(|line| line.to_string())
      .collect::<Vec<String>>()
  } else {
    println!("Error reading file {}:\n{}", path, file_content.unwrap_err());
    Vec::new()
  }
}

fn read_file_alt(path: &str) -> Result<String, std::io::Error> {
  let mut result = String::new();
  let file = File::open(path).expect("Error reading file");
  _ = DecodeReaderBytesBuilder::new()
    .encoding(Some(WINDOWS_1251))
    .build(file)
    .read_to_string(&mut result);
  Ok(result)
}