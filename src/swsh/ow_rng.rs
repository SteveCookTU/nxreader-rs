use crate::rng::{Filter, OverworldRNG, Xoroshiro};
use crate::structs::swsh::swsh_reader::{read_my_status_8, read_rng};
use sysbot_rs::SysBotClient;

pub fn ow_rng(client: SysBotClient) {
    let my_status_8 = read_my_status_8(&client);
    let (s0, s1) = read_rng(&client);
    let mut rng = Xoroshiro::from_state(s0, s1);
    let filter = Filter::default()
        .set_iv_min([0, 0, 0, 0, 0, 0])
        .set_iv_max([31, 31, 31, 31, 31, 31])
        .set_slot_min(0)
        .set_slot_max(24)
        .set_brilliant(true);
    let mut predict = OverworldRNG::default()
        .set_state(s0, s1)
        .set_tid(my_status_8.tid())
        .set_sid(my_status_8.sid())
        .set_shiny_charm(true)
        .set_mark_charm(true)
        .set_weather_active(true)
        .set_min_level(60)
        .set_max_level(60)
        .set_egg_move_count(3)
        .set_brilliant_info(500)
        .set_filter(filter);

    let mut advances = 0;
    println!("Advance {advances}, State {:X}{:X}", s0, s1);
    let mut result = predict.generate();
    while result.is_none() {
        result = predict.generate();
    }
    println!("{}", result.as_ref().unwrap());
    loop {
        let read = read_rng(&client);
        while rng.get_state() != read {
            rng.next();
            advances += 1;
            if rng.get_state() == read {
                if advances >= predict.advance {
                    result = predict.generate();
                    while result.is_none() || advances >= predict.advance {
                        result = predict.generate();
                    }
                }
                let (s0, s1) = rng.get_state();
                println!("Advance {advances}, State {:X}{:X}", s0, s1);
                println!("{}", result.as_ref().unwrap());
            }
        }
    }
}
