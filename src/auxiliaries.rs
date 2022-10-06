/// соединение массива значений в строку
/// с указанным разделителем
pub fn join_to_string<T: std::string::ToString>(array: &[T], sep: &str) -> String {
  array.into_iter()
    .map(|x| x.to_string())
    .collect::<Vec<String>>()
    .join(sep)
}

/// округление значения с плавающей запятой
/// до указанного кол-ва знаков после запятой
pub fn round(value: f32, decimals: u32) -> f32 {
  let coef = 10u32.pow(decimals) as f32;
  (coef * value).round() / coef
}