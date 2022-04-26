use crate::structs::swsh::swsh_reader::{
    read_event_block_bonus_rewards, read_event_block_crystal_encounter,
    read_event_block_drop_rewards, read_event_block_raid_encounter,
    read_event_block_raid_encounter_ct, read_event_block_raid_encounter_ioa,
};
use sysbot_rs::SysBotClient;

pub fn dumper(client: SysBotClient) {
    println!("Dumping bonus_rewards...");
    let _ = read_event_block_bonus_rewards(&client, true);
    println!("Dumping dai_encount...");
    let _ = read_event_block_crystal_encounter(&client, true);
    println!("Dumping drop_rewards...");
    let _ = read_event_block_drop_rewards(&client, true);
    println!("Dumping normal_encount...");
    let _ = read_event_block_raid_encounter(&client, true);
    println!("Dumping normal_encount_rigel1...");
    let _ = read_event_block_raid_encounter_ioa(&client, true);
    println!("Dumping normal_encount_rigel2...");
    let _ = read_event_block_raid_encounter_ct(&client, true);
    println!("\nDump Completed!\n");
}
