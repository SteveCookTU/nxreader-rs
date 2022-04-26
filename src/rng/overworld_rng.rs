use crate::rng::{Filter, Xoroshiro};
use crate::structs::pk8_overworld::PK8;

const PERSONALITY_MARKS: [&str; 28] = [
    "Rowdy",
    "AbsentMinded",
    "Jittery",
    "Excited",
    "Charismatic",
    "Calmness",
    "Intense",
    "ZonedOut",
    "Joyful",
    "Angry",
    "Smiley",
    "Teary",
    "Upbeat",
    "Peeved",
    "Intellectual",
    "Ferocious",
    "Crafty",
    "Scowling",
    "Kindly",
    "Flustered",
    "PumpedUp",
    "ZeroEnergy",
    "Prideful",
    "Unsure",
    "Humble",
    "Thorny",
    "Vigor",
    "Slump",
];

pub struct OverworldRNG {
    rng: Xoroshiro,
    advance: usize,
    tid: u16,
    sid: u16,
    shiny_charm: bool,
    mark_charm: bool,
    weather_active: bool,
    is_fishing: bool,
    is_static: bool,
    forced_ability: bool,
    flawless_ivs: u8,
    is_shiny_locked: bool,
    min_level: u8,
    max_level: u8,
    diff_held_item: bool,
    egg_move_count: u8,
    brilliant_thresh: u16,
    brilliant_rolls: u8,
    cute_charm: bool,
    filter: Filter,
}

impl OverworldRNG {
    pub fn calculate_from_pkm(pkm: &PK8) -> (u32, u32, [u8; 6]) {
        let (ec, pid, ivs, _, _) = OverworldRNG::calculate_fixed(
            pkm.seed() as u64,
            pkm.get_tid() ^ pkm.get_sid(),
            pkm.set_shininess() != 3,
            pkm.set_ivs(),
        );
        (ec, pid, ivs)
    }

    pub fn calculate_fixed(
        fixed_seed: u64,
        tsv: u16,
        shiny: bool,
        forced_ivs: u8,
    ) -> (u32, u32, [u8; 6], u8, u8) {
        let mut rng = Xoroshiro::new(fixed_seed);
        let ec = rng.next();
        let mut pid = rng.next();
        if !shiny {
            if (((pid >> 16) ^ (pid & 0xFFFF)) ^ (tsv as u32)) < 16 {
                pid ^= 0x10000000;
            }
        } else if (((pid >> 16) ^ (pid & 0xFFFF)) ^ (tsv as u32)) > 16 {
            pid = (((tsv as u32) ^ (pid & 0xFFFF)) << 16) | (pid & 0xFFFFF);
        }

        let mut ivs = [0u8; 6];
        for _ in 0..forced_ivs {
            let mut index = rng.rand_max(6);
            while ivs[index as usize] != 0 {
                index = rng.rand_max(6);
            }
            ivs[index as usize] = 31;
        }

        for i in &mut ivs {
            if *i == 0 {
                *i = rng.rand_max(32) as u8;
            }
        }

        let height = (rng.rand_max(0x81) + rng.rand_max(0x80)) as u8;
        let weight = (rng.rand_max(0x81) + rng.rand_max(0x80)) as u8;

        (ec, pid, ivs, height, weight)
    }
}
