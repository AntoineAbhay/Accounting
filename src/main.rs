use chrono::NaiveDate;
use clap::Parser;
use serde::Deserialize;
use std::error::Error;
use std::process;

mod date;

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
    payement_received_on: NaiveDate,
}

struct Results {
    income: u32,
    net_income: f64,
    vat_cashed: f64,
}

fn run(
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
        if start_date <= &record.payement_received_on && &record.payement_received_on <= end_date {
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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path of the file you want to use
    #[arg(short, long)]
    file: String,
    /// Start of the filter period
    #[arg(short, long, default_value_t = NaiveDate::from_ymd(2000, 1,1))]
    start_date: NaiveDate,
    /// End of the filter period
    #[arg(short, long, default_value_t = NaiveDate::from_ymd(2100, 1,1))]
    end_date: NaiveDate,
    /// Global rate of charges on income
    #[arg(short, long, default_value_t = 0.222)]
    charges_rate: f64
}

fn main() {
    let args = Args::parse();

    let file_path = args.file;
    let start_date = args.start_date;
    let end_date = args.end_date;
    let charges_rate = args.charges_rate;

    println!("Filter dates: {:?}/{:?}", start_date, end_date);

    match run(&file_path, &start_date, &end_date, &charges_rate) {
        Ok(results) => {
            println!("Total income: {}", results.income);
            println!("Net total income: {}", results.net_income);
            println!("Total VAT cashed: {}", results.vat_cashed);
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }
}
