use crate::structs::swsh::nest_hole_crystal_encounter_8_archive_generated::structure::root_as_nest_hole_crystal_encounter_8archive;
use crate::structs::swsh::nest_hole_distribution_encounter_8_archive_generated::structure::root_as_nest_hole_distribution_encounter_8archive;
use crate::structs::swsh::nest_hole_distribution_reward_8_archive_generated::structure::{
    root_as_nest_hole_distribution_reward_8archive, NestHoleDistributionReward8Archive,
};
use crate::structs::swsh::swsh_reader::{
    read_event_block_bonus_rewards, read_event_block_crystal_encounter,
    read_event_block_drop_rewards, read_event_block_raid_encounter,
    read_event_block_raid_encounter_ct, read_event_block_raid_encounter_ioa,
};
use crate::structs::swsh::{Den, DenSpawn, LOCAL_BONUS, LOCAL_DROPS};
use crate::util::{ITEMS, MOVES, NATURES, SPECIES};
use sysbot_rs::SysBotClient;

pub fn dump_wild_area_event(client: SysBotClient, island: u8, dump: bool) {
    let encounter_data = match island {
        0 => read_event_block_raid_encounter(&client, dump),
        1 => read_event_block_raid_encounter_ioa(&client, dump),
        _ => read_event_block_raid_encounter_ct(&client, dump),
    };

    let event_encounters =
        root_as_nest_hole_distribution_encounter_8archive(&encounter_data[32..]).unwrap();

    let drops_data = read_event_block_drop_rewards(&client, dump);
    let drop_rewards = root_as_nest_hole_distribution_reward_8archive(&drops_data[32..]).unwrap();

    let bonus_data = read_event_block_bonus_rewards(&client, dump);
    let bonus_rewards = root_as_nest_hole_distribution_reward_8archive(&bonus_data[32..]).unwrap();

    let crystal_data = read_event_block_crystal_encounter(&client, dump);
    let crystal_encounters =
        root_as_nest_hole_crystal_encounter_8archive(&crystal_data[32..]).unwrap();

    println!("Raid Encounter Table");
    if event_encounters.tables().unwrap().is_empty() {
        println!("No promoted raid or wrong offset!");
    } else {
        for table in event_encounters.tables().unwrap() {
            println!(
                "Table ID: {}\nGame Version: {}",
                table.table_id(),
                table.game_version()
            );
            for entry in table.entries().unwrap() {
                let mut msg = format!(
                    "{}:\tLv{} {}{}{}\t",
                    entry.entry_index(),
                    entry.level(),
                    {
                        if entry.is_gigantamax() {
                            "G-"
                        } else {
                            ""
                        }
                    },
                    SPECIES[entry.species() as usize].trim(),
                    {
                        if entry.alt_form() > 0 {
                            format!("-{}", entry.alt_form())
                        } else {
                            "".to_string()
                        }
                    }
                );

                if entry.shiny_flag() == 1 {
                    msg = format!("{msg}No Shiny\t");
                } else if entry.shiny_flag() == 2 {
                    msg = format!("{msg}Forced Shiny\t");
                }

                if entry.field_13() > 4 {
                    msg = format!("{msg}Not catchable\t");
                }

                match entry.ability() {
                    4 => {}
                    3 => {
                        msg = format!("{msg}A:(1/2) Only\t");
                    }
                    2 => {
                        msg = format!("{msg}HA Only\t");
                    }
                    _ => {
                        msg = format!("{msg}Ability {} Only\t", entry.ability() + 1);
                    }
                }

                if entry.nature() != 25 {
                    msg = format!("{msg}{}\t", NATURES[entry.nature() as usize].trim());
                }

                msg = format!("{msg}IVs: {}\t", entry.flawless_ivs());

                let rank = entry
                    .probabilities()
                    .unwrap()
                    .iter()
                    .position(|p| p != 0)
                    .unwrap() as u8;

                msg = format!("{msg}{:?}\t", entry.probabilities().unwrap());
                let spawn: DenSpawn = entry.into();
                msg = format!("{msg}{}", get_moves(&spawn));

                println!("{msg}");
                print_drop(entry.drop_table_id(), rank, &drop_rewards);
                print_bonus(entry.bonus_table_id(), rank, &bonus_rewards);
                println!();
            }
        }
    }

    println!("\n\nCrystal Encounter Table");
    if crystal_encounters.tables().unwrap().is_empty() {
        println!("Wrong offset!");
    } else {
        for table in crystal_encounters.tables().unwrap() {
            println!(
                "Table ID: {}\nGame Version: {}",
                table.table_id(),
                table.game_version()
            );
            for (index, entry) in table.entries().unwrap().iter().enumerate() {
                if entry.species() == 0 {
                    continue;
                }
                let mut msg = format!(
                    "{}:\tDynamax Crystal:{}\t{}{}{}\nLv{}\tN:{}\t{}/{}/{}/{}/{}/{}\t",
                    entry.entry_index(),
                    ITEMS[1279 + index].trim(),
                    {
                        if entry.is_gigantamax() != 0 {
                            "G-"
                        } else {
                            ""
                        }
                    },
                    SPECIES[entry.species() as usize].trim(),
                    {
                        if entry.alt_form() > 0 {
                            format!("-{}", entry.alt_form())
                        } else {
                            "".to_string()
                        }
                    },
                    entry.level(),
                    entry.nature(),
                    entry.iv_hp(),
                    entry.iv_atk(),
                    entry.iv_def(),
                    entry.iv_spatk(),
                    entry.iv_spdef(),
                    entry.iv_spe()
                );
                let rank = Den::get_crystal_level(entry.level() as usize).unwrap_or_default();
                let spawn: DenSpawn = entry.into();
                msg = format!("{msg}{}", get_moves(&spawn));
                println!("{msg}");
                print_drop(entry.drop_table_id(), rank, &drop_rewards);
                print_bonus(entry.bonus_table_id(), rank, &bonus_rewards);
                println!();
            }
        }
    }
}

fn print_drop(drop_id: u64, rank: u8, archive: &NestHoleDistributionReward8Archive) {
    let mut msg = "Drop: ".to_string();
    for table in LOCAL_DROPS.tables().unwrap() {
        if drop_id == table.table_id() {
            for entry in table.entries().unwrap() {
                if entry.values().unwrap().get(rank as usize) > 0 {
                    msg = format!(
                        "{}{}({}%)  \t",
                        msg,
                        ITEMS[entry.item_id() as usize].trim(),
                        entry.values().unwrap().get(rank as usize)
                    );
                }
            }
            println!("{msg}");
        }
    }

    for table in archive.tables().unwrap() {
        if drop_id == table.table_id() {
            msg = "Drop(E): ".to_string();
            for entry in table.entries().unwrap() {
                let value = match rank {
                    0 => entry.value_0(),
                    1 => entry.value_1(),
                    2 => entry.value_2(),
                    3 => entry.value_3(),
                    _ => entry.value_4(),
                };
                if value > 0 {
                    msg = format!(
                        "{}{}({value}%)  \t",
                        msg,
                        ITEMS[entry.item_id() as usize].trim()
                    );
                }
            }
            println!("{msg}");
        }
    }
}

fn print_bonus(bonus_id: u64, rank: u8, archive: &NestHoleDistributionReward8Archive) {
    let mut msg = "Bonus: ".to_string();
    for table in LOCAL_BONUS.tables().unwrap() {
        if bonus_id == table.table_id() {
            for entry in table.entries().unwrap() {
                if entry.values().unwrap().get(rank as usize) > 0 {
                    msg = format!(
                        "{}{} x {}\t\t",
                        msg,
                        entry.values().unwrap().get(rank as usize),
                        ITEMS[entry.item_id() as usize].trim()
                    );
                }
            }
            println!("{msg}");
        }
    }

    for table in archive.tables().unwrap() {
        if bonus_id == table.table_id() {
            msg = "Bonus(E): ".to_string();
            for entry in table.entries().unwrap() {
                let value = match rank {
                    0 => entry.value_0(),
                    1 => entry.value_1(),
                    2 => entry.value_2(),
                    3 => entry.value_3(),
                    _ => entry.value_4(),
                };
                if value > 0 {
                    msg = format!(
                        "{}{value} x {}\t\t",
                        msg,
                        ITEMS[entry.item_id() as usize].trim()
                    );
                }
            }
            println!("{msg}");
        }
    }
}

fn get_moves(entry: &DenSpawn) -> String {
    let mut msg = format!(
        "{} / {} / {} / {}  \t",
        MOVES[entry.move0.unwrap() as usize].trim(),
        MOVES[entry.move1.unwrap() as usize].trim(),
        MOVES[entry.move2.unwrap() as usize].trim(),
        MOVES[entry.move3.unwrap() as usize].trim()
    );
    if entry.additional_move1_rate.unwrap() > 0 {
        msg = format!(
            "{msg}({}-{}%-{}PP)",
            MOVES[entry.additional_move1.unwrap() as usize].trim(),
            entry.additional_move1_rate.unwrap(),
            entry.additional_move1_pp.unwrap()
        )
    }
    if entry.additional_move2_rate.unwrap() > 0 {
        msg = format!(
            "{msg} ({}-{}%-{}PP)",
            MOVES[entry.additional_move2.unwrap() as usize].trim(),
            entry.additional_move2_rate.unwrap(),
            entry.additional_move2_pp.unwrap()
        )
    }
    msg
}
