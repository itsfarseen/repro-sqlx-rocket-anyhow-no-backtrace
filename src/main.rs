#![feature(backtrace)]

// If I remove SQLx import, backtrace is generated correctly
use sqlx;

fn main() {
    println!("{}", std::backtrace::Backtrace::force_capture());
}
