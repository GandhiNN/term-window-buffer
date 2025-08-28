mod dataframe;
mod display;
mod termsize;

use dataframe::DataFrame;
use std::error::Error;

const CSV: &str = "files/annual-enterprise-survey-2024-financial-year-provisional.csv";

fn main() -> Result<(), Box<dyn Error>> {
    let us = termsize::get_term_size();
    println!("{:#?}", us);
    let df = DataFrame::from_csv(CSV)?;

    // let df = tab.unwrap().data;
    // let _ = display::display(&df);
    df.display();
    Ok(())
}
