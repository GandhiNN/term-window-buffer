#![allow(unused)]
use crate::dataframe::DataFrame;
use crate::termsize;
use serde::Serialize;
use std::error::Error;
use std::fmt::Debug;
use std::io;
use std::io::{Read, Write};
use tabled::{
    Table, Tabled,
    settings::{Height, Settings, Style, Width, peaker::Priority},
};
pub fn display<T: Tabled + Debug + Serialize>(
    dataframe: &Vec<Vec<T>>,
) -> Result<(), Box<dyn Error>> {
    let term_size = termsize::get_term_size().unwrap_or_else(|| {
        println!("Error: something went wrong");
        std::process::exit(1)
    });
    let height = term_size.rows as usize;
    let width = term_size.cols as usize;

    let mut current_pos: usize = 0;
    let mut records_counter: usize = 0;
    let mut done = false;

    // Convert dataframe to be csv-ready
    let data: Vec<String> = vec![];

    while !done {
        let max = current_pos + height - 1;

        for i in current_pos..max {
            if i >= dataframe.len() {
                done = true;
                break;
            }
            let line = &dataframe[i];

            println!("{:#?}", line);

            if i == max - 1 {
                current_pos = i;
            }
        }

        if !done {
            print!("\x1b[92m[Press Enter for Next page or \"q\" to quit...] \x1b[0m");
            std::io::stdout().flush().unwrap();

            let mut command = String::new();
            io::stdin().read_line(&mut command).unwrap();
            if command.trim() == "q" {
                done = true;
            }
        }
    }
    Ok(())
}
