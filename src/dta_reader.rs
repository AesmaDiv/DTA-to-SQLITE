use std::fs::{read_to_string, File};
use std::io::Read;

use encoding_rs::WINDOWS_1251;
use encoding_rs_io::DecodeReaderBytesBuilder;

pub struct DTAReader;
impl DTAReader {
  /// чтение текстового DTA файла и получение строк,
  /// готовых к парсингу
  pub fn read_dta(path: &str) -> Vec<String> {
    let mut lines = DTAReader::read_file(path);
    DTAReader::fix_lines(&mut lines);

    lines
  }

  /// очистка строк от лишних символов и пробелов:
  /// остаются только значения готовые к парсингу
  fn fix_lines(lines: &mut Vec<String>) {
    if lines.len() >= 37 {
      lines.iter_mut().for_each(|line: &mut String| {
        let i = line.find("_").unwrap_or(0);
        line.truncate(i);
        *line = line.trim().to_string();
      })
    }
  }

  /// получение строк из текстового DTA файла
  fn read_file(path: &str) -> Vec<String> {
    let mut file_content = read_to_string(path);
    if file_content.is_err() {
      file_content = DTAReader::read_file_alt(path);
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

  /// альтернативная функция получения строк из DTA файла
  /// для WIN1251 кодировки
  fn read_file_alt(path: &str) -> Result<String, std::io::Error> {
    let mut result = String::new();
    let file = File::open(path).expect("Error reading file");
    _ = DecodeReaderBytesBuilder::new()
      .encoding(Some(WINDOWS_1251))
      .build(file)
      .read_to_string(&mut result);
    Ok(result)
  }
}

