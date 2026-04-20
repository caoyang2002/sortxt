// src/lib.rs
pub mod args;
pub mod sort;
pub mod io_utils;
pub mod generator;   // 新增

pub use args::Args;
pub use sort::{SortConfig, sort_lines, compute_sort_key};
pub use generator::GenType;
