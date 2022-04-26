use std::io::{stdout, Write};
use crate::structs::swsh::swsh_reader::{read_k_coordinates_block, read_my_status_8};
use std::thread;
use std::time::Duration;
use sysbot_rs::SysBotClient;

pub fn check_overworld_pokemon(client: SysBotClient) {
    let my_status_8 = read_my_status_8(&client);
    let mut last_info = Vec::new();
    loop {
        let coordinates = read_k_coordinates_block(&client);
        let pkms =
            coordinates.read_ow_pokemon_from_block(&client, my_status_8.tid(), my_status_8.sid());
        let mut info = Vec::with_capacity(pkms.len());
        for pkm in pkms {
            info.push(format!("0x{:0<8X} {}", pkm.seed(), pkm));
        }
        if info != last_info {
            for pkm in &info {
                print!("{}", pkm);
            }
            stdout().flush().unwrap();
            last_info = info;
            println!("-------------------------------");
        }
        thread::sleep(Duration::from_millis(300));
    }
}
