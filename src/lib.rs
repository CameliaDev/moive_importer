//! Library surface for `moive_importer` (spelling kept for crate/repo name).
//!
//! Binary CLI lives in `main.rs`; conversion logic is in `movie_converter`.

pub mod movie_converter;

pub use movie_converter::convert_txt_to_json;