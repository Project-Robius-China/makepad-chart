pub mod table;
pub mod csv_parser;

pub use table::{DataTable, Value};
pub use csv_parser::parse_csv;
