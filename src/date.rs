use chrono::format::ParseError;
use chrono::NaiveDate;
use serde::{self, Deserialize, Deserializer};

pub const FORMAT: &'static str = "%Y-%m-%d";

// pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     let s = format!("{}", date.format(FORMAT));
//     serializer.serialize_str(&s)
// }

fn parse_date(s: &String) -> Result<NaiveDate, ParseError> {
  NaiveDate::parse_from_str(&s, FORMAT)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  parse_date(&s).map_err(serde::de::Error::custom)
}
