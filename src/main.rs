#![feature(backtrace)]

#[macro_use]
extern crate rocket;

// 1. If I remove SQLx import, backtrace is generated correctly
use sqlx;

// 2. Backtrace is generated correctly in a non async main() function

// 3. Backtrace is generated correctly in #[tokio::main] async main() function, if rocket is
//    removed from Cargo.toml

#[rocket::main]
async fn main() -> Result<(),()> {
    println!("{}", std::backtrace::Backtrace::force_capture());
    Ok(())
}

