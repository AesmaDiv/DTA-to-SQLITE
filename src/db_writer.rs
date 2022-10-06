use rusqlite::Connection;
use crate::pump::Pump;
use crate::auxiliaries::{round, join_to_string};

/// подключение и запись в БД информации о типоразмерах
pub fn write_all_pumps(path: &str, pumps: Vec<Pump>) -> usize {
  let mut result = 0;
  match Connection::open(path) {
    Ok(conn) => {
      pumps.into_iter().for_each(
        |pump| {
          result += write_single_pump(&conn, &pump);
      });
      _ = conn.close();
    },
    Err(err) => {
      println!("Error connecting to DB\n{path}\n{}", err);
    }
  }
  result
}

/// запись в БД информации о типоразмере
fn write_single_pump(conn: &Connection, pump: &Pump) -> usize {
  let sql = "
    INSERT INTO TypesRust
      (ProducerName, Name, Date, Rpm, Min, Nom, Max, Flows, Lifts, Powers)
    Values
      (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
  ";
  let params = (
    &pump.producer,
    &pump.name,
    &pump.date,
    &pump.rpm,
    &pump.min,
    &pump.nom,
    &pump.max,
    &join_to_string(&pump.flows, ","),
    &join_to_string(&pump.lifts, ","),
    &join_to_string(&pump.powers, ",")
  );
  match conn.execute(sql, params) {
    Ok(count) => count,
    Err(err) => {
      println!("Failed to write pump {:#?}\n{:#?}", pump, err);
      0
    }
  }
}

