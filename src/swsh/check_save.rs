use crate::structs::swsh::swsh_reader::read_my_status_8;
use sysbot_rs::SysBotClient;

pub fn check_save(client: SysBotClient) {
    let my_status_8 = read_my_status_8(&client);
    println!("{}\n", my_status_8);
}
