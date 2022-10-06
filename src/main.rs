use std::fs::read_dir;

pub mod auxiliaries;
pub mod pump;
pub mod dta_reader;
pub mod db_writer;
use pump::Pump;
use db_writer::write_all_pumps;

fn main() {
  // получение полных путей
  let path_db = get_full_path("assets/pump.sqlite");
  let path_curves = get_full_path("assets/curves");

  // получение списка DTA файлов с описанием типоразмеров
  let files = get_files(path_curves.as_str());
  if !files.is_empty() {
    // получение списка типоразмеров
    let mut pumps = Vec::new();
    files.iter().for_each(
      |f| {
        if let Some(pump) = Pump::load(f.as_str()) {
          pumps.push(pump);
        }
      }
    );

    println!("Readed {}", pumps.len());
    println!("Last {:#?}", pumps.last());

    // запись типоразмеров в БД
    let pumps_written = write_all_pumps(path_db.as_str(), pumps);
    if pumps_written > 0 {
      println!("{pumps_written} pumps written successfully");
    } else {
      println!("Failed to write pumps");
    }
  }
}

/// получение полного пути к папке с DTA файлами
fn get_full_path(subpath: &str) -> String {
  let path_buf = std::env::current_dir()
    .expect("Error getting current working directory")
    .as_path()
    .join(std::path::Path::new(subpath));
  match path_buf.as_path().to_str() {
    Some(result) => String::from(result),
    _ => String::new()
  }
}

/// получение списка полных путей к DTA файлам
fn get_files(path: &str) -> Vec<String> {
  let files = read_dir(path);
  if files.is_ok() {
    files.unwrap()
    .map(
      |file| file.unwrap()
      .path()
      .display()
      .to_string())
    .filter(|file| file.ends_with(".DTA"))
    .collect::<Vec<String>>()
  } else {
    println!("{path}\n{:#?}", files.unwrap_err());
    Vec::new()
  }
}