use crate::date;
use chrono::NaiveDate;
use serde::Serialize;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Row<'a> {
    #[serde(with = "date")]
    start: &'a NaiveDate,
    #[serde(with = "date")]
    end: &'a NaiveDate,
    total: &'a u32,
    vat: &'a f64,
    #[serde(with = "date")]
    payment_received_on: &'a NaiveDate,
}

pub fn add_invoice(
    file_path: &String,
    start_date: &NaiveDate,
    end_date: &NaiveDate,
    total: &u32,
    vat: &f64,
    payment_received_on: &NaiveDate,
) -> Result<(), Box<dyn Error>> {
    let file_exists = Path::new(file_path).is_file();
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .has_headers(!file_exists)
        .from_writer(file);
    wtr.serialize(Row {
        start: start_date,
        end: end_date,
        total,
        vat,
        payment_received_on,
    })?;
    wtr.flush()?;
    Ok(())
}
