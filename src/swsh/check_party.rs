use sysbot_rs::SysBotClient;
use crate::structs::swsh::swsh_reader::read_party;

pub fn check_party(client: SysBotClient) {
    let pkms = read_party(&client);
    for (i, pkm) in pkms.iter().enumerate() {
        println!("Slot: {}", i + 1);
        if pkm.is_valid() && pkm.ec() != 0 {
            println!("{}", pkm);
        } else {
            println!("Empty\n");
        }
    }
}
