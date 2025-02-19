use serde;
use serde::Deserialize;

pub fn vec_deserializer<'de, D: serde::Deserializer<'de>, T: serde::Deserialize<'de>>(deserializer: D) -> Result<Vec<T>, D::Error> {
    let opt = Option::<Vec<T>>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

pub trait StreamObject<T> {
    fn print_from_vec(objects: Vec<T>);
}
