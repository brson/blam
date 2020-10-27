use anyhow::Result;
use structopt::StructOpt;
use std::path::PathBuf;

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
struct CommonOpts {
}

#[derive(StructOpt, Debug)]
enum OptCommand {
    Parse(ParseCommand),
}

#[derive(StructOpt, Debug)]
struct ParseCommand {
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
    }
}

fn run_parse(cmd: Command<ParseCommand>) -> Result<()> {
    panic!()
}
