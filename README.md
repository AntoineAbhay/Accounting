# Accounting

The main goal of this script is to compute for a given period of time:
 - the total income 
 - the net total income
 - the total VAT cashed

## Usage

### How to build
You need to have Rust installed (with cargo)
```
cargo build --release
```


### Execute
You can find an example of csv file in this repo: data/invoices_example.csv
```
./target/release/accounting compute -f data/invoices_example.csv -s 2022-01-01 -e 2022-12-31 -c 0.2

./target/release/accounting add -f ./data/invoices_example.csv -s 2022-11-01 -e 2022-11-30 -t 999 -v 20 --payment-received-on 2022-12-10
```

For more help
```
./target/release/accounting -h
```