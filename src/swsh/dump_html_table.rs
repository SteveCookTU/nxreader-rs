use crate::structs::swsh::nest_hole_distribution_encounter_8_archive_generated::structure::root_as_nest_hole_distribution_encounter_8archive;
use crate::structs::swsh::nest_hole_distribution_reward_8_archive_generated::structure::{
    root_as_nest_hole_distribution_reward_8archive, NestHoleDistributionReward8Archive,
};
use crate::structs::swsh::{DenSpawn, ALOLA_LIST, GALAR_LIST, LOCAL_BONUS, LOCAL_DROPS};
use crate::util::{ABILITIES, FORMS, ITEMS, MOVES, NATURES, PERSONAL_TABLE, SPECIES, TR_MOVES};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub fn dump_html_table(use_large_image: bool, island: u8) {
    let file = if island == 0 {
        "normal_encount".to_string()
    } else {
        format!("normal_encount_rigel{island}")
    };
    let mut event_table_file = File::open(file).expect("Failed to open file");
    let mut event_table_raw = Vec::new();
    event_table_file
        .read_to_end(&mut event_table_raw)
        .expect("Failed to read bytes from file");
    let event_encounters =
        root_as_nest_hole_distribution_encounter_8archive(&event_table_raw[32..])
            .expect("Invalid flatbuffer");

    let mut event_drops_file = File::open("drop_rewards").expect("Failed to open file");
    let mut event_drops_raw = Vec::new();
    event_drops_file
        .read_to_end(&mut event_drops_raw)
        .expect("Failed to read bytes from file");
    let drop_rewards = root_as_nest_hole_distribution_reward_8archive(&event_drops_raw[32..])
        .expect("Invalid flatbuffer");

    let mut event_bonus_file = File::open("bonus_rewards").expect("Failed to open file");
    let mut event_bonus_raw = Vec::new();
    event_bonus_file
        .read_to_end(&mut event_bonus_raw)
        .expect("Failed to read bytes from file");
    let bonus_rewards = root_as_nest_hole_distribution_reward_8archive(&event_bonus_raw[32..])
        .expect("Invalid flatbuffer");

    if event_encounters.tables().unwrap().len() != 2
        || event_encounters
            .tables()
            .unwrap()
            .get(1)
            .entries()
            .unwrap()
            .len()
            != event_encounters
                .tables()
                .unwrap()
                .get(0)
                .entries()
                .unwrap()
                .len()
    {
        println!("Not a standard table");
    }

    let mut output = OpenOptions::new()
        .create(true)
        .write(true)
        .open("table.html")
        .unwrap();
    output.write_all(br#"<table border="1" class="table table-striped table-bordered table-condensed" style="text-align:center; margin:auto">"#).unwrap();
    output.write_all(b"<tbody>").unwrap();
    let table_header = format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}",
        th("".to_string()),
        th("Pokemon".to_string()),
        th("Chance".to_string()),
        th("Games".to_string()),
        th("Level".to_string()),
        th("Perfect<br>IVs".to_string()),
        th("Shield".to_string()),
        th("Dynamax<br>Level".to_string()),
        th("Dynamax<br>Boost".to_string()),
        th("Moves".to_string()),
        th("Drop".to_string()),
        th("Bonus".to_string()),
        th("Comment".to_string())
    );
    output.write_all(tr(table_header).as_bytes()).unwrap();

    for star in 0..5 {
        let star = 4 - star;
        let mut stars = "&#9733".to_string();
        for _ in 0..star {
            stars = format!("{}&#9733", stars);
        }
        output
            .write_all(format!("\t\t<td colspan=\"13\">{stars}</td>").as_bytes())
            .unwrap();
        for index in (0..event_encounters
            .tables()
            .unwrap()
            .get(0)
            .entries()
            .unwrap()
            .len())
            .rev()
        {
            let entry1: DenSpawn = event_encounters
                .tables()
                .unwrap()
                .get(0)
                .entries()
                .unwrap()
                .get(index)
                .into();
            let entry2: DenSpawn = event_encounters
                .tables()
                .unwrap()
                .get(1)
                .entries()
                .unwrap()
                .get(index)
                .into();
            if is_the_same(&entry1, &entry2)
                && entry1.probabilities[star as usize] == entry2.probabilities[star as usize]
                && entry1.probabilities[star as usize] > 0
            {
                output
                    .write_all(
                        tr(format!(
                            "{}{}{}",
                            get_msg_1(&entry1, star, use_large_image),
                            td("SWSH".to_string(), "".to_string()),
                            get_msg_2(&entry1, star, &drop_rewards, &bonus_rewards)
                        ))
                        .as_bytes(),
                    )
                    .unwrap();
            } else {
                if entry1.probabilities[star as usize] > 0 {
                    output
                        .write_all(
                            tr(format!(
                                "{}{}{}",
                                get_msg_1(&entry1, star, use_large_image),
                                td("SWSH".to_string(), "".to_string()),
                                get_msg_2(&entry1, star, &drop_rewards, &bonus_rewards)
                            ))
                            .as_bytes(),
                        )
                        .unwrap();
                }
                if entry2.probabilities[star as usize] > 0 {
                    output
                        .write_all(
                            tr(format!(
                                "{}{}{}",
                                get_msg_1(&entry2, star, use_large_image),
                                td("SWSH".to_string(), "".to_string()),
                                get_msg_2(&entry2, star, &drop_rewards, &bonus_rewards)
                            ))
                            .as_bytes(),
                        )
                        .unwrap();
                }
            }
        }
    }

    output.write_all(b"</tbody></table>").unwrap();
}

fn tr(content: String) -> String {
    format!("\t<tr>\n{}\t</tr>", content)
}

fn th(content: String) -> String {
    format!("\t\t<th>{}</th>\n", content)
}

fn td(content: String, style: String) -> String {
    format!("\t\t<td{}>{}</td>\n", style, content)
}

fn is_the_same(entry1: &DenSpawn, entry2: &DenSpawn) -> bool {
    !(entry1.species != entry2.species
        || entry1.alt_form != entry2.alt_form
        || entry1.is_gigantamax != entry2.is_gigantamax
        || entry1.shiny_flag.as_ref().unwrap() != entry2.shiny_flag.as_ref().unwrap())
}

fn get_pm_image(
    species: u16,
    form: u8,
    can_gmax: bool,
    is_shiny: bool,
    use_large_image: bool,
) -> String {
    let mut filename = format!("{:0>3}", species);
    if species == 849 && form == 1 && !can_gmax {
        filename = format!("{}-1", filename);
    }
    if can_gmax && species != 868 {
        filename = format!("{}-gi", filename);
    }
    if form >= 1 {
        if species == 678 || species == 876 {
            filename = format!("{}-f", filename);
        } else if ALOLA_LIST.contains(&species) && form == 1 {
            filename = format!("{}-a", filename);
        } else if GALAR_LIST.contains(&species) {
            filename = format!("{}-g", filename);
        }
    }

    if use_large_image {
        let url = if is_shiny {
            format!("https://www.serebii.net/Shiny/SWSH/{}.png", filename)
        } else {
            format!(
                "https://www.serebii.net/swordshield/pokemon/{}.png",
                filename
            )
        };
        format!(
            "<img src=\"{}\" alt=\"{}\" width=\"100\" height=\"100\">",
            url, filename
        )
    } else {
        let url = format!("https://www.serebii.net/pokedex-swsh/icon/{}.png", filename);
        format!("<img src=\"{}\" alt=\"{}\">", url, filename)
    }
}

fn get_pm_name(species: u16, form: u8, can_gmax: bool, is_shiny: bool) -> String {
    let mut t = SPECIES[species as usize].trim().to_string();
    if species == 849
        || species == 869
        || species == 678
        || species == 876
        || ((GALAR_LIST.contains(&species) || ALOLA_LIST.contains(&species)) && form != 0)
    {
        let form_text = FORMS[PERSONAL_TABLE
            .get_form_name_index(species as usize, form as usize)
            .unwrap()]
        .trim();
        t = format!("{}<br><small>{}</small>", t, form_text);
    }
    if can_gmax {
        t = format!("{}<br><small>Gigantamax</small>", t);
    }
    if is_shiny {
        t = format!("{}<br><small>Shiny</small>", t);
    }
    t
}

fn get_item_image(item_id: u16) -> String {
    let filename = ITEMS[item_id as usize]
        .replace(' ', "")
        .replace('’', "'")
        .trim()
        .to_lowercase();
    let url = format!("https://www.serebii.net/itemdex/sprites/{}.png", filename);
    format!("<img src=\"{}\" alt=\"{}\">", url, filename)
}

fn get_item_name(item_id: u16) -> String {
    let txt = format!(
        "{}{}",
        get_item_image(item_id),
        ITEMS[item_id as usize].trim()
    );
    if (1130..=1229).contains(&item_id) {
        let tr = item_id - 1130;
        format!("{} ({})", txt, TR_MOVES[tr as usize].trim())
    } else {
        txt
    }
}

fn get_msg_1(spawn: &DenSpawn, rank: u8, use_large_image: bool) -> String {
    format!(
        "{}{}{}",
        td(
            get_pm_image(
                spawn.species as u16,
                spawn.alt_form as u8,
                spawn.is_gigantamax,
                spawn.shiny_flag.unwrap() == 2,
                use_large_image
            ),
            "".to_string()
        ),
        td(
            get_pm_name(
                spawn.species as u16,
                spawn.alt_form as u8,
                spawn.is_gigantamax,
                spawn.shiny_flag.unwrap() == 2
            ),
            "".to_string()
        ),
        td(
            format!("{}%", spawn.probabilities[rank as usize]),
            "".to_string()
        )
    )
}

fn get_msg_2(
    spawn: &DenSpawn,
    rank: u8,
    drop_rewards: &NestHoleDistributionReward8Archive,
    bonus_rewards: &NestHoleDistributionReward8Archive,
) -> String {
    let pi = PERSONAL_TABLE.get_form_entry(spawn.species as usize, spawn.alt_form as usize);
    let mut txt = format!(
        "{}{}{}{}{}",
        td(spawn.level.unwrap().to_string(), "".to_string()),
        td(spawn.flawless_ivs.to_string(), "".to_string()),
        td(spawn.shield.unwrap().to_string(), "".to_string()),
        td(spawn.dynamax_level.unwrap().to_string(), "".to_string()),
        td(spawn.dynamax_boost.unwrap().to_string(), "".to_string())
    );
    let sep = "<br>";
    let mut move_txt = if spawn.move3.unwrap() > 0 {
        format!(
            "{}{sep}{}{sep}{}{sep}{}",
            MOVES[spawn.move0.unwrap() as usize].trim(),
            MOVES[spawn.move1.unwrap() as usize].trim(),
            MOVES[spawn.move2.unwrap() as usize].trim(),
            MOVES[spawn.move3.unwrap() as usize].trim()
        )
    } else if spawn.move2.unwrap() > 0 {
        format!(
            "{}{sep}{}{sep}{}",
            MOVES[spawn.move0.unwrap() as usize].trim(),
            MOVES[spawn.move1.unwrap() as usize].trim(),
            MOVES[spawn.move2.unwrap() as usize].trim()
        )
    } else if spawn.move1.unwrap() > 0 {
        format!(
            "{}{sep}{}",
            MOVES[spawn.move0.unwrap() as usize].trim(),
            MOVES[spawn.move1.unwrap() as usize].trim()
        )
    } else {
        MOVES[spawn.move0.unwrap() as usize].trim().to_string()
    };

    if spawn.additional_move1_rate.unwrap() > 0 {
        move_txt = format!(
            "{}<br><br>{} ({}% - {}PP)",
            move_txt,
            MOVES[spawn.additional_move1.unwrap() as usize].trim(),
            spawn.additional_move1_rate.unwrap(),
            spawn.additional_move1_pp.unwrap()
        );
    }

    if spawn.additional_move2_rate.unwrap() > 0 {
        move_txt = format!(
            "{}<br>{} ({}% - {}PP)",
            move_txt,
            MOVES[spawn.additional_move2.unwrap() as usize].trim(),
            spawn.additional_move2_rate.unwrap(),
            spawn.additional_move2_pp.unwrap()
        );
    }

    txt = format!("{}{}", txt, td(move_txt, "".to_string()));

    let drop_id = spawn.drop_table_id;
    let mut drop_msg = String::new();
    let mut style = "";
    for drop_table in LOCAL_DROPS.tables().unwrap() {
        if drop_id == drop_table.table_id() {
            for entry in drop_table.entries().unwrap() {
                if entry.values().unwrap().get(rank as usize) > 0 {
                    drop_msg = format!(
                        "{}{}% {}<br>",
                        drop_msg,
                        entry.values().unwrap().get(rank as usize),
                        get_item_name(entry.item_id() as u16)
                    );
                }
            }
        }
    }

    for drop_table in drop_rewards.tables().unwrap() {
        if drop_id == drop_table.table_id() {
            style = " style = \"background-color:#ffe4c3\"";
            for entry in drop_table.entries().unwrap() {
                let value = match rank {
                    0 => entry.value_0(),
                    1 => entry.value_1(),
                    2 => entry.value_2(),
                    3 => entry.value_3(),
                    _ => entry.value_4(),
                };

                if value > 0 {
                    drop_msg = format!(
                        "{}{}% {}<br>",
                        drop_msg,
                        value,
                        get_item_name(entry.item_id())
                    );
                }
            }
        }
    }

    txt = format!(
        "{}{}",
        txt,
        td(
            drop_msg[..(drop_msg.len() - 4)].to_string(),
            style.to_string()
        )
    );

    let bonus_id = spawn.bonus_table_id;
    let mut bonus_txt = "".to_string();

    for bonus_table in LOCAL_BONUS.tables().unwrap() {
        if bonus_id == bonus_table.table_id() {
            style = "";
            for entry in bonus_table.entries().unwrap() {
                if entry.values().unwrap().get(rank as usize) > 0 {
                    bonus_txt = format!(
                        "{}{}x{}<br>",
                        bonus_txt,
                        entry.values().unwrap().get(rank as usize),
                        get_item_name(entry.item_id() as u16)
                    );
                }
            }
        }
    }

    for bonus_table in bonus_rewards.tables().unwrap() {
        if bonus_id == bonus_table.table_id() {
            style = " style = \"background-color:#ffe4c3\"";
            for entry in bonus_table.entries().unwrap() {
                let value = match rank {
                    0 => entry.value_0(),
                    1 => entry.value_1(),
                    2 => entry.value_2(),
                    3 => entry.value_3(),
                    _ => entry.value_4(),
                };

                if value > 0 {
                    bonus_txt = format!(
                        "{}{}x{}<br>",
                        bonus_txt,
                        value,
                        get_item_name(entry.item_id())
                    );
                }
            }
        }
    }

    txt = format!(
        "{}{}",
        txt,
        td(
            bonus_txt[..(bonus_txt.len() - 4)].to_string(),
            style.to_string()
        )
    );

    let mut comment = match spawn.ability {
        4 => "".to_string(),
        3 => "No Hidden Ability<br>".to_string(),
        2 => format!(
            "Ability: H-{}<br>",
            ABILITIES[pi.ability_h() as usize].trim()
        ),
        1 => format!(
            "Ability: 2-{}<br>",
            ABILITIES[pi.ability_2() as usize].trim()
        ),
        _ => format!(
            "Ability: 1-{}<br>",
            ABILITIES[pi.ability_1() as usize].trim()
        ),
    };

    if spawn.nature.unwrap() != 25 {
        format!(
            "{}Nature: {}<br>",
            comment,
            NATURES[spawn.nature.unwrap() as usize].trim()
        );
    }

    if spawn.shiny_flag.unwrap() == 1 {
        comment = format!("{}Cannot be shiny<br>", comment);
    }

    txt = format!(
        "{}{}",
        txt,
        td(
            {
                if comment.is_empty() {
                    "-".to_string()
                } else {
                    comment[..(comment.len() - 4)].to_string()
                }
            },
            "".to_string()
        )
    );

    txt
}
