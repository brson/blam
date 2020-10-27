use anyhow::Result;
use structopt::StructOpt;
use std::path::PathBuf;
use std::fs;
use log::info;

use crate::parser;
use crate::typecheck;

pub fn run() -> Result<()> {
    env_logger::init();
    dispatch(Opts::from_args())
}

#[derive(StructOpt, Debug)]
struct Opts {
    #[structopt(subcommand)]
    command: OptCommand,
    #[structopt(flatten)]
    common: CommonOpts,
}

#[derive(StructOpt, Debug)]
enum OptCommand {
    Parse(ParseCommand),
    TypeCheck(TypeCheckCommand),
}

#[derive(StructOpt, Debug)]
struct CommonOpts {
}

#[derive(StructOpt, Debug)]
struct ParseCommand {
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

#[derive(StructOpt, Debug)]
struct TypeCheckCommand {
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

struct Command<C> {
    command: C,
    common: CommonOpts,
}

fn dispatch(opts: Opts) -> Result<()> {
    let Opts { command, common } = opts;
    match command {
        OptCommand::Parse(command) => {
            run_parse(Command { command, common })
        }
        OptCommand::TypeCheck(command) => {
            run_typecheck(Command { command, common })
        }
    }
}

fn run_parse(cmd: Command<ParseCommand>) -> Result<()> {
    let schema = fs::read_to_string(&cmd.command.file)?;
    let ast = parser::parse(&schema)?;

    info!("{:#?}", ast);
    
    Ok(())
}

fn run_typecheck(cmd: Command<TypeCheckCommand>) -> Result<()> {
    let schema = fs::read_to_string(&cmd.command.file)?;
    let ast = parser::parse(&schema)?;
    let typeinfo = typecheck::check(&ast)?;

    info!("{:#?}", typeinfo);

    Ok(())
}
