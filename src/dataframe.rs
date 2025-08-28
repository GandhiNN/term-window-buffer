#![allow(unused)]
use csv;
use std::error::Error;
use std::fs::File;
use tabled::{Table, Tabled};

#[derive(Debug)]
pub struct DataFrame {
    pub header: Vec<String>,
    pub data: Vec<Vec<Record>>,
}

#[derive(Tabled, Debug)]
pub struct Record {
    pub year: i32,
    pub agg_nzsioc: String,
    pub code_nzsioc: String,
    pub name_nzsioc: String,
    pub units: String,
    pub var_code: String,
    pub var_name: String,
    pub var_category: String,
    pub value: i32,
    pub code_anzsic06: String,
}

impl From<csv::StringRecord> for Record {
    fn from(sr: csv::StringRecord) -> Self {
        Self {
            year: { sr[0].parse::<i32>().unwrap_or_else(|_| 9999) },
            agg_nzsioc: sr[1].into(),
            code_nzsioc: sr[2].into(),
            name_nzsioc: sr[3].into(),
            units: sr[4].into(),
            var_code: sr[5].into(),
            var_name: sr[6].into(),
            var_category: sr[7].into(),
            value: { sr[8].parse::<i32>().unwrap_or_else(|_| 9999) },
            code_anzsic06: sr[9].into(),
        }
    }
}

impl DataFrame {
    pub fn new(csv_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(csv_path)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);
        let csv_header = reader.headers()?;

        let header: Vec<String> = csv_header.into_iter().map(|r| r.to_string()).collect();
        let mut records: Vec<Vec<Record>> = vec![];

        for result in reader.records() {
            let data: Vec<Record> = result.into_iter().map(|r| Record::from(r)).collect();
            records.push(data);
        }

        Ok(DataFrame {
            header: header,
            data: records,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CSV: &str = "../files/test.csv";

    #[test]
    fn test_csv_reader() {
        let tab = DataFrame::new(CSV);
        println!("{:#?}", tab);
    }
}
