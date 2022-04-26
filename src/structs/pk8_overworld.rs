use crate::rng::OverworldRNG;
use crate::util::{ABILITIES, GENDER_SYMBOLS, NATURES, SPECIES};
use std::fmt::{Display, Formatter};

const RIBBONS: [&str; 98] = [
    "ChampionKalos",
    "ChampionG3",
    "ChampionSinnoh",
    "BestFriends",
    "Training",
    "BattlerSkillful",
    "BattlerExpert",
    "Effort",
    "Alert",
    "Shock",
    "Downcast",
    "Careless",
    "Relax",
    "Snooze",
    "Smile",
    "Gorgeous",
    "Royal",
    "GorgeousRoyal",
    "Artist",
    "Footprint",
    "Record",
    "Legend",
    "Country",
    "National",
    "Earth",
    "World",
    "Classic",
    "Premier",
    "Event",
    "Birthday",
    "Special",
    "Souvenir",
    "Wishing",
    "ChampionBattle",
    "ChampionRegional",
    "ChampionNational",
    "ChampionWorld",
    "CountMemoryContest",
    "CountMemoryBattle",
    "ChampionG6Hoenn",
    "ContestStar",
    "MasterCoolness",
    "MasterBeauty",
    "MasterCuteness",
    "MasterCleverness",
    "MasterToughness",
    "ChampionAlola",
    "BattleRoyale",
    "BattleTreeGreat",
    "BattleTreeMaster",
    "ChampionGalar",
    "TowerMaster",
    "MasterRank",
    "Lunchtime",
    "Sleep-Time",
    "Dusk",
    "Dawn",
    "Cloudy",
    "Rainy",
    "Stormy",
    "Snowy",
    "Blizzard",
    "Dry",
    "Sandstorm",
    "Misty",
    "Destiny",
    "Fishing",
    "Curry",
    "Uncommon",
    "Rare",
    "Rowdy",
    "Absent-Minded",
    "Jittery",
    "Excited",
    "Charismatic",
    "Calmness",
    "Intense",
    "Zoned-Out",
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
    "Pumped-Up",
    "ZeroEnergy",
    "Prideful",
    "Unsure",
    "Humble",
    "Thorny",
    "Vigor",
    "Slump",
];

pub struct PK8 {
    tid: u16,
    sid: u16,
    data: Vec<u8>,
    pid: u32,
    ec: u32,
    ivs: [u8; 6],
}

impl PK8 {
    pub fn new(tid: u16, sid: u16, data: Vec<u8>) -> Self {
        let mut pkm = Self {
            tid,
            sid,
            data,
            pid: 0,
            ec: 0,
            ivs: [0, 0, 0, 0, 0, 0],
        };
        (pkm.ec, pkm.pid, pkm.ivs) = OverworldRNG::calculate_from_pkm(&pkm);
        pkm
    }

    pub fn get_tid(&self) -> u16 {
        self.tid
    }

    pub fn get_sid(&self) -> u16 {
        self.sid
    }

    pub fn species(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x0..0x2]).try_into().unwrap())
    }

    pub fn alt_form(&self) -> u8 {
        self.data[0x2]
    }

    pub fn level(&self) -> u8 {
        self.data[0x4]
    }

    pub fn nature(&self) -> u8 {
        self.data[0x8]
    }

    pub fn gender(&self) -> u8 {
        self.data[0xA]
    }

    pub fn ability(&self) -> u8 {
        self.data[0xC] - 1
    }

    pub fn mark(&self) -> u8 {
        self.data[0x16]
    }

    pub fn brilliant(&self) -> bool {
        self.data[0x20] != 0
    }

    pub fn set_ivs(&self) -> u8 {
        self.data[0x12]
    }

    pub fn set_shininess(&self) -> u8 {
        self.data[0x6] + 1
    }

    pub fn shiny_type(&self) -> u8 {
        PK8::get_shiny_type(((self.sid as u32) << 16) | self.tid as u32, self.pid)
    }

    pub fn seed(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x18..0x1C]).try_into().unwrap())
    }

    pub fn get_shiny_type(ot_id: u32, pid: u32) -> u8 {
        let xor = (ot_id >> 16) ^ (ot_id & 0xFFFF) ^ (pid >> 16) ^ (pid & 0xFFFF);
        if xor > 15 {
            0
        } else if xor == 0 {
            2
        } else {
            1
        }
    }
}

impl Display for PK8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let shiny_type = self.shiny_type();
        let shiny_flag = if shiny_type == 0 {
            ""
        } else if shiny_type == 1 {
            "⋆ "
        } else {
            "◇ "
        };
        write!(f,
               "EC: {:X}  PID: {:X}  {}{}{}\nLevel: {}\nNature: {}  Ability: {}  Gender: {}\nIVs: {:?}\nMark: {}\n{}\n",
               self.ec,
               self.pid,
               shiny_flag,
               SPECIES[self.species() as usize].trim(),
               {
                   if self.alt_form() > 0 {
                       format!("-{}", self.alt_form())
                   } else {
                       "".to_string()
                   }
               },
               self.level(),
               NATURES[self.nature() as usize].trim(),
               ABILITIES[self.ability() as usize].trim(),
               GENDER_SYMBOLS[self.gender() as usize - 1],
               self.ivs,
               {
                   if self.mark() != 255 {
                       RIBBONS[self.mark() as usize]
                   } else {
                       ""
                   }
               },
               {
                   if self.brilliant() {
                       "Brilliant!\n"
                   } else {
                       ""
                   }
               }
        )
    }
}
