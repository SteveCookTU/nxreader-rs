use crate::structs::swsh::swsh_reader::read_party;
use std::io::{stdout, Write};
use sysbot_rs::SysBotClient;

pub fn check_party(client: SysBotClient) {
    let pkms = read_party(&client);
    for (i, pkm) in pkms.iter().enumerate() {
        print!("Slot: {}\n", i + 1);
        if pkm.is_valid() && pkm.ec() != 0 {
            print!("{}\n", pkm);
        } else {
            print!("Empty\n\n");
        }
    }
    stdout().flush().unwrap();
}
