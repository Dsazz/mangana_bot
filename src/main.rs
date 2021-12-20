#![deny(clippy::all, clippy::pedantic, clippy::cognitive_complexity)]
#![allow(clippy::non_ascii_literal)]
#![warn(unused_extern_crates)]

use std::io::Write;
use dotenv::dotenv;
use env_logger::{Builder as LogBuilder, Env};

use crate::domain::pool::run_db_migrations;
use crate::helper::repeat_work;

mod helper;
mod domain;
mod application;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate anyhow;

use crate::application::task::start_bot::StartBot;
use crate::application::task::parse_job::job;

static PARSE_INTERVAL: u64 = 1800_u64;
static APP_VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() {
    dotenv().ok(); // Read .env and set env variables with this
    run_db_migrations();

    LogBuilder::from_env(Env::default().default_filter_or("info"))
        .format(|buf, record| {
            writeln!(buf, "{}[{}v]: {}", record.level(), APP_VERSION, record.args())
        })
        .write_style(env_logger::WriteStyle::Auto)
        .init();

    let start_bot = StartBot::new();
    tokio::join!(
        start_bot.execute(),
        repeat_work(job, PARSE_INTERVAL)
    );
}
