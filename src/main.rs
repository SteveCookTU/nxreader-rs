use clap::{ArgEnum, Parser, Subcommand};
use nxreader::swsh;
use sysbot_rs::SysBotClient;

#[derive(Subcommand)]
enum Command {
    CheckBox,
    CheckDen {
        #[clap(long)]
        do_research: bool,
        #[clap(short, long)]
        max_results: Option<usize>,
    },
    CheckHorse,
}

#[derive(Copy, Clone, ArgEnum)]
enum Game {
    Swsh,
}

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    addr: String,
    #[clap(short, long)]
    port: u16,
    #[clap(arg_enum, short, long)]
    game: Game,
    #[clap(subcommand)]
    command: Command,
}

fn main() {
    let args: Args = Args::parse();

    if let Ok(client) = SysBotClient::connect(&args.addr, args.port) {
        match &args.command {
            Command::CheckBox => match &args.game {
                Game::Swsh => {
                    swsh::check_box(client);
                }
            },
            Command::CheckDen {
                do_research,
                max_results,
            } => match &args.game {
                Game::Swsh => {
                    swsh::check_den(client, do_research, max_results.unwrap_or_default());
                }
            },
            Command::CheckHorse => match &args.game {
                Game::Swsh => {
                    swsh::check_horse(client);
                }
            }
        }
    }
}
