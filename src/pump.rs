struct Pump {
  producer: &str,
  name: &str,
  date: &str,
  rpm: i32,
  min: f32,
  nom: f32,
  max: f32,
  flows: [f32; 7],
  lifts: [f32; 7],
  powers: [f32; 7]
}