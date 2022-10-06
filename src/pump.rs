use crate::dta_reader::DTAReader;
use crate::auxiliaries::round;

#[derive(Debug)]
pub struct Pump {
  pub producer: String,
  pub name: String,
  pub date: String,
  pub rpm: i32,
  pub min: f32,
  pub nom: f32,
  pub max: f32,
  pub flows: [f32; 7],
  pub lifts: [f32; 7],
  pub powers: [f32; 7]
}

impl Pump {
  /// конструктор, посредством чтения из DTA файла 
  pub fn load(path: &str) -> Option<Self> {
    let lines = DTAReader::read_dta(path);
    Pump::parse(lines)
  }

  /// конструктор, посредством парсинга строк 
  fn parse(dta_lines: Vec<String>) -> Option<Self> {
    Some(Pump {
      producer: dta_lines[1].clone(),
      name: dta_lines[2].clone(),
      date: dta_lines[3].clone(),
      rpm: dta_lines[4].parse().unwrap_or(0),
      min: dta_lines[13].parse().unwrap_or(0.0),
      nom: dta_lines[14].parse().unwrap_or(0.0),
      max: dta_lines[15].parse().unwrap_or(0.0),
      flows: Pump::get_flows(dta_lines[16].as_str()),
      lifts: Pump::get_values(&dta_lines, [18, 24], 1.0),
      powers: Pump::get_values(&dta_lines, [29, 35], 1.0),
    })
  }

  /// получение массива значений расхода
  /// из значения макс.расхода разделенного на 7 частей
  fn get_flows(line: &str) -> [f32; 7] {
    let flow = line.parse().unwrap_or(0) as f32;
    let flows: Result<[f32; 7], _> = (0..7)
      .map(
        |x|
          round(x as f32 * flow / 6.0, 2))
      .collect::<Vec<f32>>()
      .try_into();
    match flows {
        Ok(vals) => vals,
        _ => [0.0; 7]
    }
  }

  /// получение массива значений из массива строк
  /// с указанием индексов первой и последней строки массива
  /// которые будут парситься в числовые значения
  /// с применением коэфициента
  fn get_values(lines: &Vec<String>, range: [usize; 2], coef: f32) -> [f32; 7] {
    let vals: Result<[f32; 7], _> = (range[0]..range[1]+1)
      .map(
        |x| 
          round(lines[x]
            .parse()
            .unwrap_or(0.0) * coef, 3))
      .collect::<Vec<f32>>()
      .try_into();
    match vals {
        Ok(result) => result,
        _ => [0.0; 7]
    }
  }
}