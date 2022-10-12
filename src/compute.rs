use crate::date;
use chrono::NaiveDate;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
  #[serde(with = "date")]
  start: NaiveDate,
  #[serde(with = "date")]
  end: NaiveDate,
  total: u32,
  vat: f64,
  #[serde(with = "date")]
  payment_received_on: NaiveDate,
}

pub struct Results {
  pub income: u32,
  pub net_income: f64,
  pub vat_cashed: f64,
}

pub fn compute(
  file_path: &String,
  start_date: &NaiveDate,
  end_date: &NaiveDate,
  charges_rate: &f64,
) -> Result<Results, Box<dyn Error>> {
  let mut rdr = csv::ReaderBuilder::new()
    .delimiter(b';')
    .from_path(file_path)?;
  let mut income = 0;
  let mut vat_cashed = 0.0;
  for result in rdr.deserialize() {
    let record: Record = result?;
    if start_date <= &record.start && &record.end <= end_date {
      income = income + record.total;
    }
    if start_date <= &record.payment_received_on && &record.payment_received_on <= end_date {
      vat_cashed = vat_cashed + record.vat;
    }
  }
  let results = Results {
    income,
    net_income: income as f64 * (1.0 - charges_rate),
    vat_cashed,
  };
  Ok(results)
}
