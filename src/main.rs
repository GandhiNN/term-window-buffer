mod dataframe;
mod display;
mod termsize;

use dataframe::DataFrame;

const CSV: &str = "files/annual-enterprise-survey-2024-financial-year-provisional.csv";

fn main() {
    let us = termsize::get_term_size();
    // println!("{:#?}", us);
    let tab = DataFrame::new(CSV);
    // println!("{:#?}", tab);
    let df = tab.unwrap().data;
    let _ = display::display(&df);
}
