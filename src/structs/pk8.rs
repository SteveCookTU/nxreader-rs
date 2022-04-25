use crate::util::{ABILITIES, GENDER_SYMBOLS, MOVES, NATURES, SPECIES};
use std::fmt::{Display, Formatter};
use std::fs;

pub const STORED_SIZE: usize = 0x148;
pub const PARTY_SIZE: usize = 0x158;
pub const BLOCK_SIZE: usize = 0x50;

const BLOCK_POSITION: [u8; 128] = [
    0, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 1, 0, 2, 3, 1, 0, 3, 2,
    2, 0, 1, 3, 3, 0, 1, 2, 2, 0, 3, 1, 3, 0, 2, 1, 1, 2, 0, 3, 1, 3, 0, 2, 2, 1, 0, 3, 3, 1, 0, 2,
    2, 3, 0, 1, 3, 2, 0, 1, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 0,
    0, 1, 2, 3, 0, 1, 3, 2, 0, 2, 1, 3, 0, 3, 1, 2, 0, 2, 3, 1, 0, 3, 2, 1, 1, 0, 2, 3, 1, 0, 3, 2,
];

const BLOCK_POSITION_INVERT: [u8; 32] = [
    0, 1, 2, 4, 3, 5, 6, 7, 12, 18, 13, 19, 8, 10, 14, 20, 16, 22, 9, 11, 15, 21, 17, 23, 0, 1, 2,
    4, 3, 5, 6, 7,
];

pub struct PK8 {
    data: Vec<u8>,
}

impl From<Vec<u8>> for PK8 {
    fn from(data: Vec<u8>) -> Self {
        let mut pk8 = Self { data };
        if pk8.is_encrypted() {
            pk8.decrypt();
        }
        pk8
    }
}

impl PK8 {
    pub fn ec(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x0..0x4]).try_into().unwrap())
    }

    pub fn checksum(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x6..0x8]).try_into().unwrap())
    }

    pub fn species(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x8..0xA]).try_into().unwrap())
    }

    pub fn held_item(&self) -> u16 {
        u16::from_le_bytes((&self.data[0xA..0xC]).try_into().unwrap())
    }

    pub fn sid_tid(&self) -> u32 {
        u32::from_le_bytes((&self.data[0xC..0x10]).try_into().unwrap())
    }

    pub fn ability(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x14..0x16]).try_into().unwrap())
    }

    pub fn ability_num(&self) -> u8 {
        self.data[0x16] & 0x7
    }

    pub fn get_ability_string(&self) -> String {
        let ability = self.ability_num();
        if ability < 4 {
            ability.to_string()
        } else {
            "H".to_string()
        }
    }

    pub fn can_gigantamax(&self) -> bool {
        (self.data[0x16] & 16) != 0
    }

    pub fn pid(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x1C..0x20]).try_into().unwrap())
    }

    pub fn nature(&self) -> u8 {
        self.data[0x20]
    }

    pub fn stat_nature(&self) -> u8 {
        self.data[0x21]
    }

    pub fn gender(&self) -> u8 {
        (self.data[0x22] >> 2) & 0x3
    }

    pub fn alt_form(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x24..0x26]).try_into().unwrap())
    }

    pub fn evs(&self) -> [u8; 6] {
        [
            self.data[0x26],
            self.data[0x27],
            self.data[0x28],
            self.data[0x2A],
            self.data[0x2B],
            self.data[0x29],
        ]
    }

    pub fn move_1(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x72..0x74]).try_into().unwrap())
    }

    pub fn move_2(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x74..0x76]).try_into().unwrap())
    }

    pub fn move_3(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x76..0x78]).try_into().unwrap())
    }

    pub fn move_4(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x78..0x7A]).try_into().unwrap())
    }

    pub fn current_hp_stat(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x8A..0x8C]).try_into().unwrap())
    }

    pub fn iv32(&self) -> u32 {
        u32::from_le_bytes((&self.data[0x8C..0x90]).try_into().unwrap())
    }

    pub fn language(&self) -> u8 {
        self.data[0xE2]
    }

    pub fn ball(&self) -> u8 {
        self.data[0x124]
    }

    pub fn home_tracker(&self) -> u64 {
        u64::from_le_bytes((&self.data[0x135..0x13D]).try_into().unwrap())
    }

    pub fn battle_stats(&self) -> (u8, [u16; 6]) {
        (
            self.data[0x148],
            [
                u16::from_le_bytes((&self.data[0x14A..0x14C]).try_into().unwrap()),
                u16::from_le_bytes((&self.data[0x14C..0x14E]).try_into().unwrap()),
                u16::from_le_bytes((&self.data[0x14E..0x150]).try_into().unwrap()),
                u16::from_le_bytes((&self.data[0x152..0x154]).try_into().unwrap()),
                u16::from_le_bytes((&self.data[0x154..0x156]).try_into().unwrap()),
                u16::from_le_bytes((&self.data[0x150..0x152]).try_into().unwrap()),
            ],
        )
    }

    pub fn is_egg(&self) -> bool {
        ((self.iv32() >> 31) & 1) == 1
    }

    pub fn ivs(&self) -> [u8; 6] {
        let iv32 = self.iv32();
        [
            (iv32 & 0x1F) as u8,
            ((iv32 >> 5) & 0x1F) as u8,
            ((iv32 >> 10) & 0x1F) as u8,
            ((iv32 >> 20) & 0x1F) as u8,
            ((iv32 >> 25) & 0x1F) as u8,
            ((iv32 >> 15) & 0x1F) as u8,
        ]
    }

    pub fn mark(&self) -> String {
        if self.data[0x3A] > 5 {
            let marks = ["Lunchtime", "Sleepy-Time", "Dusk"];
            marks[f64::log2(f64::from(self.data[0x3A])) as usize - 5].to_string()
        } else if self.data[0x3B] > 0 {
            let marks = [
                "Dawn",
                "Cloudy",
                "Rainy",
                "Stormy",
                "Snowy",
                "Blizzard",
                "Dry",
                "Sandstorm",
            ];
            marks[f64::log2(f64::from(self.data[0x3B])) as usize].to_string()
        } else if self.data[0x40] > 0 {
            let marks = [
                "Misty",
                "Destiny",
                "Fishing",
                "Curry",
                "Uncommon",
                "Rare",
                "Rowdy",
                "Absent-Minded",
            ];
            marks[f64::log2(f64::from(self.data[0x40])) as usize].to_string()
        } else if self.data[0x41] > 0 {
            let marks = [
                "Jittery",
                "Excited",
                "Charismatic",
                "Calmness",
                "Intense",
                "Zoned-Out",
                "Joyful",
                "Angry",
            ];
            marks[f64::log2(f64::from(self.data[0x41])) as usize].to_string()
        } else if self.data[0x42] > 0 {
            let marks = [
                "Smiley",
                "Teary",
                "Upbeat",
                "Peeved",
                "Intellectual",
                "Ferocious",
                "Crafty",
                "Scowling",
            ];
            marks[f64::log2(f64::from(self.data[0x42])) as usize].to_string()
        } else if self.data[0x43] > 0 {
            let marks = [
                "Kindly",
                "Flustered",
                "Pumped-Up",
                "ZeroEnergy",
                "Prideful",
                "Unsure",
                "Humble",
                "Thorny",
            ];
            marks[f64::log2(f64::from(self.data[0x43])) as usize].to_string()
        } else if self.data[0x44] > 0 {
            let marks = ["Vigor", "Slump"];
            marks[f64::log2(f64::from(self.data[0x44])) as usize].to_string()
        } else {
            "".to_string()
        }
    }

    pub fn calc_checksum(&self) -> u16 {
        let mut chk: u16 = 0;
        for i in (8..STORED_SIZE).step_by(2) {
            chk = chk.wrapping_add(u16::from_le_bytes(
                (&self.data[i..(i + 2)]).try_into().unwrap(),
            ));
        }
        chk
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

    pub fn shiny_type(&self) -> u8 {
        PK8::get_shiny_type(self.sid_tid(), self.pid())
    }

    pub fn shiny_string(&self) -> String {
        let shiny_type = self.shiny_type();
        if shiny_type == 0 {
            "None".to_string()
        } else if shiny_type == 1 {
            "Star".to_string()
        } else {
            "Square".to_string()
        }
    }

    pub fn save(&self, filename: &str) {
        fs::write(filename, &self.data).unwrap()
    }

    pub fn is_valid(&self) -> bool {
        self.checksum() == self.calc_checksum() && !self.is_encrypted()
    }

    pub fn is_encrypted(&self) -> bool {
        u16::from_le_bytes((&self.data[0x70..0x72]).try_into().unwrap()) != 0
            && u16::from_le_bytes((&self.data[0xC0..0xC2]).try_into().unwrap()) != 0
    }

    pub fn decrypt(&mut self) {
        let seed = self.ec();
        let sv = (seed >> 13) & 0x1F;

        self.crypt_pkm(seed as usize);
        self.shuffle(sv as usize);
    }

    fn crypt_pkm(&mut self, seed: usize) {
        self.crypt(seed, 8, STORED_SIZE);
        if self.data.len() == PARTY_SIZE {
            self.crypt(seed, STORED_SIZE, PARTY_SIZE)
        }
    }

    fn crypt(&mut self, mut seed: usize, start: usize, end: usize) {
        let mut i = start;
        while i < end {
            seed = seed.wrapping_mul(0x41C64E6D).wrapping_add(0x00006073);
            self.data[i] ^= (seed >> 16) as u8;
            i += 1;
            self.data[i] ^= (seed >> 24) as u8;
            i += 1;
        }
    }

    fn shuffle(&mut self, sv: usize) {
        let idx = 4 * sv;
        let sdata = self.data.clone();
        for block in 0..4 {
            let ofs = BLOCK_POSITION[idx + block] as usize;
            self.data.splice(
                (8 + BLOCK_SIZE * block)..(8 + BLOCK_SIZE * (block + 1)),
                sdata[(8 + BLOCK_SIZE * ofs)..(8 + BLOCK_SIZE * (ofs + 1))]
                    .iter()
                    .cloned(),
            );
        }
    }

    pub fn refresh_checksum(&mut self) {
        self.data
            .splice(0x6..0x8, self.calc_checksum().to_le_bytes());
    }

    pub fn encrypt(&mut self) -> &Vec<u8> {
        self.refresh_checksum();
        let seed = self.ec();
        let sv = (seed >> 13) & 0x1F;

        self.shuffle(BLOCK_POSITION_INVERT[sv as usize] as usize);
        self.crypt_pkm(seed as usize);
        &self.data
    }
}

impl Display for PK8 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_valid() {
            let shiny_type = self.shiny_type();
            let shiny_flag = if shiny_type == 0 {
                ""
            } else if shiny_type == 1 {
                "⋆ "
            } else {
                "◇ "
            };
            write!(f,
                   "EC: {:X}  PID: {:X}  {}{}{}{}\nNature: {}({})  Ability: {}({})  Gender: {}\nIVs: {:?}, EVs: {:?}\nMoves: {} / {} / {} / {}\n",
                   self.ec(),
                   self.pid(),
                   shiny_flag,
                   {
                       if self.can_gigantamax() {
                           "G-"
                       } else {
                           ""
                       }
                   },
                   SPECIES[self.species() as usize].trim(),
                   {
                       if self.alt_form() > 0 {
                           format!("-{}", self.alt_form())
                       } else {
                           "".to_string()
                       }
                   },
                   NATURES[self.nature() as usize].trim(),
                   NATURES[self.stat_nature() as usize].trim(),
                   ABILITIES[self.ability() as usize].trim(),
                   {
                       if self.ability_num() < 4 {
                           self.ability_num().to_string()
                       } else {
                           "H".to_string()
                       }
                   },
                   GENDER_SYMBOLS[self.gender() as usize],
                   self.ivs(),
                   self.evs(),
                   MOVES[self.move_1() as usize].trim(),
                   MOVES[self.move_2() as usize].trim(),
                   MOVES[self.move_3() as usize].trim(),
                   MOVES[self.move_4() as usize].trim()
            )
        } else {
            write!(f, "Invalid Data")
        }
    }
}
