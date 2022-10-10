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
You can find an example of csv file in this repo: example.csv
```
./target/release/accouting -f accounting.csv -s 2022-01-01 -e 2022-12-31 -c 0.2
```

For more help
```
./target/release/accouting -h
```