use crate::structs::swsh::swsh_reader::{
    dump_event_block_bonus_rewards, dump_event_block_crystal_encounter,
    dump_event_block_drop_rewards, dump_event_block_raid_encounter,
    dump_event_block_raid_encounter_ct, dump_event_block_raid_encounter_ioa,
};
use sysbot_rs::SysBotClient;

pub fn dumper(client: SysBotClient) {
    println!("Dumping bonus_rewards...");
    dump_event_block_bonus_rewards(&client);
    println!("Dumping dai_encount...");
    dump_event_block_crystal_encounter(&client);
    println!("Dumping drop_rewards...");
    dump_event_block_drop_rewards(&client);
    println!("Dumping normal_encount...");
    dump_event_block_raid_encounter(&client);
    println!("Dumping normal_encount_rigel1...");
    dump_event_block_raid_encounter_ioa(&client);
    println!("Dumping normal_encount_rigel2...");
    dump_event_block_raid_encounter_ct(&client);
    println!("\nDump Completed!\n");
}
