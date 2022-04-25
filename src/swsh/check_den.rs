use crate::rng::{Raid, Xoroshiro};
use crate::structs::swsh::swsh_reader::{read_dens, read_my_status_8};
use crate::structs::swsh::DenSpawn;
use crate::util::SPECIES;
use std::io::{stdout, Write};
use sysbot_rs::SysBotClient;

pub fn check_den(client: SysBotClient, do_research: &bool, max_results: usize) {
    let mut seed: Option<u64> = None;
    let mut pieced_spawn: Option<DenSpawn> = None;
    let mut pieced_shiny_lock: Option<u8> = None;
    let my_status_8 = read_my_status_8(&client);
    let is_sword = my_status_8.is_sword();
    let dens = read_dens(&client);
    println!();
    for (i, den) in dens.iter().enumerate() {
        if den.is_active() {
            let spawn = den.get_spawn(i, is_sword);
            let mut curr_shiny_lock = 0;
            let mut info = if i > 189 {
                format!("[CT] denID: {}", i - 189)
            } else if i > 99 {
                format!("[IoA] denID: {}", i - 99)
            } else {
                format!("denID: {}", i + 1)
            };
            info = format!(
                "{}\t{}★\tSpecies: {} {}{}{}",
                info,
                den.stars(),
                SPECIES[spawn.species as usize].trim(),
                {
                    if spawn.is_gigantamax {
                        "G-Max"
                    } else {
                        ""
                    }
                },
                {
                    if den.is_event() {
                        curr_shiny_lock = spawn.shiny_flag.unwrap();
                        "\tEvent"
                    } else {
                        ""
                    }
                },
                {
                    if den.is_wishing_piece() {
                        seed = Some(den.seed());
                        pieced_spawn = Some(spawn.clone());
                        pieced_shiny_lock = Some(curr_shiny_lock as u8);
                        if curr_shiny_lock != 2 {
                            format!(
                                "\tNextShinyFrame: {}",
                                Raid::get_next_shiny_frame(den.seed())
                            )
                        } else {
                            "\tNext Shiny Frame: 0".to_string()
                        }
                    } else {
                        "".to_string()
                    }
                }
            );
            let raid = Raid::new(
                den.seed(),
                my_status_8.tid(),
                my_status_8.sid(),
                spawn.flawless_ivs as u8,
                curr_shiny_lock as u8,
                spawn.ability as u8,
                spawn.gender as u8,
                spawn.species as u16,
                spawn.alt_form as u8,
            );
            println!("{}\n{}", info, raid);
            stdout().flush().unwrap();
        }
    }
    if let Some(seed) = seed {
        if *do_research {
            println!("Wishing Piece Den Predicition:");
            let mut seed = seed;
            let mut i = 0;
            while i < max_results {
                let raid = Raid::new(
                    seed,
                    my_status_8.tid(),
                    my_status_8.sid(),
                    pieced_spawn.as_ref().unwrap().flawless_ivs as u8,
                    *pieced_shiny_lock.as_ref().unwrap(),
                    pieced_spawn.as_ref().unwrap().ability as u8,
                    pieced_spawn.as_ref().unwrap().gender as u8,
                    pieced_spawn.as_ref().unwrap().species as u16,
                    pieced_spawn.as_ref().unwrap().alt_form as u8,
                );
                seed = Xoroshiro::new(seed).next_u64();
                println!("Frame: {i}\n{}", raid);
                i += 1;
            }
        }
    }
}
