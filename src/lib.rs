// src/lib.rs

pub mod args;
pub mod generator;
pub mod io_utils;
pub mod sort;
pub mod unique;
pub use unique::{ProcessMode, process_sorted_lines, generate_stats, get_all_groups};
pub use args::Args;
pub use generator::GenType;
pub use sort::{SortConfig, compute_sort_key, sort_lines};
