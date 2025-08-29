#![allow(unused)]
use crate::termsize;
use csv;
use serde::Serialize;
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;
use std::io::{Read, Write};
use tabled::{Table, Tabled};

#[derive(Debug)]
pub struct DataFrame {
    pub header: Vec<String>,
    pub data: Vec<Record>,
}

#[derive(Serialize, Tabled, Debug)]
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
    pub fn from_csv(csv_path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(csv_path)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);
        let csv_header = reader.headers()?;

        let header: Vec<String> = csv_header.into_iter().map(|r| r.to_string()).collect();
        let mut records: Vec<Record> = vec![];

        for result in reader.records() {
            let data: Record = result.map(|r| Record::from(r))?;
            records.push(data);
        }

        Ok(DataFrame {
            header: header,
            data: records,
        })
    }

    pub fn show_csv(&self) {
        let term_size = termsize::get_term_size().unwrap_or_else(|| {
            println!("Error: something went wrong");
            std::process::exit(1)
        });
        let height = term_size.rows as usize;
        let width = term_size.cols as usize;

        let mut current_pos: usize = 0;
        let mut records_counter: usize = 0;
        let mut done = false;

        while !done {
            let max = current_pos + height - 1;

            for i in current_pos..max {
                if i >= self.data.len() {
                    done = true;
                    break;
                }
                if i == 0 {
                    let mut wtr = csv::WriterBuilder::new()
                        .has_headers(true)
                        .from_writer(vec![]);
                    wtr.serialize(&self.data[i]);
                    let line = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
                    print!("{line}");
                } else {
                    let mut wtr = csv::WriterBuilder::new()
                        .has_headers(false)
                        .from_writer(vec![]);
                    wtr.serialize(&self.data[i]);
                    let line = String::from_utf8(wtr.into_inner().unwrap()).unwrap();
                    print!("{line}");
                }

                if i == max - 1 {
                    current_pos = i;
                }
            }

            if !done {
                print!("\x1b[92m[Press Enter for Next page or \"q\" to quit...] \x1b[0m");
                std::io::stdout().flush().unwrap();

                let mut command = String::new();
                std::io::stdin().read_line(&mut command).unwrap();
                if command.trim() == "q" {
                    done = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CSV: &str = "../files/test.csv";

    #[test]
    fn test_csv_reader() {
        let tab = DataFrame::from_csv(CSV);
        println!("{:#?}", tab);
    }
}
