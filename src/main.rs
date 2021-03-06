use crate::Game::Swsh;
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
    CheckLegend,
    CheckOverworldPokemon,
    CheckParty,
    CheckSave,
    CheckWild,
    Dumper,
    DumpHtmlTable {
        #[clap(long)]
        large_images: bool,
        #[clap(short, long)]
        island: Option<u8>,
        #[clap(long)]
        dump: bool,
    },
    DumpWildAreaEvent {
        #[clap(long)]
        dump: bool,
        #[clap(short, long)]
        island: Option<u8>,
    },
    OwRng,
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
                Swsh => swsh::check_box(client),
            },
            Command::CheckDen {
                do_research,
                max_results,
            } => match &args.game {
                Swsh => swsh::check_den(client, do_research, max_results.unwrap_or_default()),
            },
            Command::CheckHorse => match &args.game {
                Swsh => swsh::check_horse(client),
            },
            Command::CheckLegend => match &args.game {
                Swsh => swsh::check_legend(client),
            },
            Command::CheckOverworldPokemon => match &args.game {
                Swsh => swsh::check_overworld_pokemon(client),
            },
            Command::CheckParty => match &args.game {
                Swsh => swsh::check_party(client),
            },
            Command::CheckSave => match &args.game {
                Swsh => swsh::check_save(client),
            },
            Command::CheckWild => match &args.game {
                Swsh => swsh::check_wild(client),
            },
            Command::Dumper => match &args.game {
                Swsh => swsh::dumper(client),
            },
            Command::DumpHtmlTable {
                large_images,
                island,
                dump,
            } => match &args.game {
                Swsh => swsh::dump_html_table(
                    client,
                    *large_images,
                    if let Some(island) = island {
                        *island
                    } else {
                        0
                    },
                    *dump,
                ),
            },
            Command::DumpWildAreaEvent { dump, island } => match &args.game {
                Swsh => swsh::dump_wild_area_event(
                    client,
                    if let Some(island) = island {
                        *island
                    } else {
                        0
                    },
                    *dump,
                ),
            },
            Command::OwRng => match &args.game {
                Swsh => swsh::ow_rng(client),
            },
        }
    }
}
