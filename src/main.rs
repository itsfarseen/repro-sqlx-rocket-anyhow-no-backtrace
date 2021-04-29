#![feature(backtrace)]

// 1. If I remove SQLx import, backtrace is generated correctly
use sqlx;

// 2. Backtrace is generated correctly in a non async main() function

#[tokio::main]
async fn main() -> Result<(),()> {
    println!("{}", std::backtrace::Backtrace::force_capture());
    Ok(())
}

