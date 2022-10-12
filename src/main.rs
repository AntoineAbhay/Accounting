use chrono::NaiveDate;
use clap::Parser;
use std::process;

mod compute;
mod date;
mod invoices;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand, Debug)]
enum Action {
    Compute {
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
        charges_rate: f64,
    },
    Add {
        /// The path of the file you want to use
        #[arg(short, long)]
        file: String,
        /// Start of the period
        #[arg(short, long)]
        start_date: NaiveDate,
        /// End of the period
        #[arg(short, long)]
        end_date: NaiveDate,
        /// Total income
        #[arg(short, long)]
        total: u32,
        /// Total vat
        #[arg(short, long)]
        vat: f64,
        /// Date of the payment
        #[arg(short, long)]
        payment_received_on: NaiveDate,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.action {
        Action::Compute {
            file,
            start_date,
            end_date,
            charges_rate,
        } => {
            println!("Filter dates: {:?}/{:?}", start_date, end_date);

            match compute::compute(&file, &start_date, &end_date, &charges_rate) {
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
        Action::Add {
            file,
            start_date,
            end_date,
            total,
            vat,
            payment_received_on,
        } => {
            match invoices::add_invoice(
                &file,
                &start_date,
                &end_date,
                &total,
                &vat,
                &payment_received_on,
            ) {
                Ok(()) => {
                    println!("Invoice added");
                }
                Err(err) => {
                    println!("{}", err);
                    process::exit(1);
                }
            }
        }
    }
}
