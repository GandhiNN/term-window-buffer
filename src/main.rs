mod display;
mod table;
mod termsize;

use table::Table;

const CSV: &str = "../files/annual-enterprise-survey-2024-financial-year-provisional.csv";

fn main() {
    let us = termsize::get_term_size();
    println!("{:#?}", us);
    let tab = Table::new(CSV);
    let _ = display::display(&tab.unwrap());
}
