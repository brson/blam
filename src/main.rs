#![allow(unused)]

#[macro_use]
extern crate lalrpop_util;

use anyhow::Result;

mod ast;
mod table;
mod parser;
mod cli;
mod typecheck;

fn main() -> Result<()> {
    cli::run()
}
