// src/lib.rs
pub mod args;
pub mod generator;
pub mod io_utils;
pub mod sort; // 新增

pub use args::Args;
pub use generator::GenType;
pub use sort::{SortConfig, compute_sort_key, sort_lines};
