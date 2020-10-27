#![allow(unused)]

#[macro_use]
extern crate lalrpop_util;

use anyhow::Result;

mod table;
mod parser;
mod cli;

fn main() -> Result<()> {
    cli::run()
}
