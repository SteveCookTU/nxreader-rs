use crate::rng::Xoroshiro;
use crate::util::{GENDER_SYMBOLS, NATURES, PERSONAL_TABLE};
use std::fmt::{Display, Formatter};

const TOXTRICITY_AMPED_NATURES: [u8; 13] = [3, 4, 2, 8, 9, 19, 22, 11, 13, 14, 0, 6, 24];
const TOXTRICITY_LOW_KEY_NATURES: [u8; 12] = [1, 5, 7, 10, 12, 15, 16, 17, 18, 20, 21, 23];

pub struct Raid {
    seed: u64,
    ec: u32,
    pid: u32,
    shiny_type: String,
    ivs: [u8; 6],
    ability: u8,
    gender: u8,
    nature: u8,
}

impl Raid {
    pub fn new(
        seed: u64,
        tid: u16,
        sid: u16,
        flawless_iv: u8,
        shiny_lock: u8,
        mut ability: u8,
        mut gender: u8,
        species: u16,
        alt_form: u8,
    ) -> Self {
        let pi = PERSONAL_TABLE.get_form_entry(species as usize, alt_form as usize);
        let mut r = Xoroshiro::new(seed);
        let ec = r.next();
        let ot_id = r.next();
        let mut pid = r.next();

        let psv = Raid::get_shiny_value(pid);
        let pid_shiny_type = Raid::get_shiny_xor(pid) ^ tid ^ sid;
        let tsv = Raid::get_shiny_value((tid ^ sid) as u32);

        let mut shiny_type = "None".to_string();

        if shiny_lock == 0 {
            let seed_shiny_type = Raid::get_shiny_type(pid, ot_id);
            let ftsv = Raid::get_shiny_value(ot_id);
            if ftsv == psv {
                if seed_shiny_type == 1 {
                    shiny_type = "Star".to_string();
                    if psv != tsv || pid_shiny_type == 0 {
                        pid = Raid::get_final_pid(pid, tid, sid, seed_shiny_type);
                    }
                } else if seed_shiny_type == 2 {
                    shiny_type = "Square".to_string();
                    if psv != tsv || pid_shiny_type != 0 {
                        pid = Raid::get_final_pid(pid, tid, sid, seed_shiny_type);
                    }
                }
            } else if psv == tsv {
                pid ^= 0x10000000;
            }
        } else if shiny_lock == 1 {
            if psv == tsv {
                pid ^= 0x10000000;
            }
        } else {
            if pid_shiny_type >= 16 || pid_shiny_type != 0 {
                pid = Raid::get_final_pid(pid, tid, sid, 2);
            }
            shiny_type = "Square".to_string()
        }
        let mut i = 0;
        let mut ivs = [0, 0, 0, 0, 0, 0];
        while i < flawless_iv {
            let stat = r.rand_max(6);
            if ivs[stat as usize] == 0 {
                ivs[stat as usize] = 31;
                i += 1;
            }
        }

        for i in 0..6 {
            if ivs[i as usize] == 0 {
                ivs[i as usize] = (r.next() & 0x1F) as u8;
            }
        }

        if ability == 4 {
            ability = r.rand_max(4) as u8 + 1;
        } else if ability == 3 {
            ability = (r.next() & 1) as u8 + 1;
        } else {
            ability += 1;
        }

        if gender == 0 {
            let ratio = pi.gender();
            if ratio == 255 {
                gender = 2;
            } else if ratio == 254 {
                gender = 1;
            } else if ratio == 0 {
                gender = 0;
            } else if (r.rand_max(0xFF) as u8 + 1) < ratio {
                gender = 1;
            } else {
                gender = 0;
            }
        } else {
            gender -= 1;
        }

        let nature = if species != 849 {
            r.rand_max(25) as u8
        } else if alt_form == 0 {
            TOXTRICITY_AMPED_NATURES[r.rand_max(14) as usize]
        } else {
            TOXTRICITY_LOW_KEY_NATURES[r.rand_max(13) as usize]
        };

        Self {
            seed,
            ec,
            pid,
            shiny_type,
            ivs,
            ability,
            gender,
            nature,
        }
    }

    pub fn get_shiny_value(pid: u32) -> u16 {
        Raid::get_shiny_xor(pid) >> 4
    }

    pub fn get_shiny_xor(val: u32) -> u16 {
        (val >> 16) as u16 ^ (val as u16)
    }

    pub fn get_shiny_type(pid: u32, ot_id: u32) -> u8 {
        let p = Raid::get_shiny_xor(pid);
        let t = Raid::get_shiny_xor(ot_id);
        if p == t {
            2
        } else if (p ^ t) < 16 {
            1
        } else {
            0
        }
    }

    pub fn get_final_pid(pid: u32, tid: u16, sid: u16, seed_shiny_type: u8) -> u32 {
        let high_pid = (pid as u16) ^ tid ^ sid ^ ((2 - seed_shiny_type) as u16);
        ((high_pid as u32) << 16) | (pid & 0xFFFF)
    }

    pub fn get_next_shiny_frame(mut seed: u64) -> usize {
        for i in 0..99999 {
            let mut r = Xoroshiro::new(seed);
            seed = r.next_u64();
            let ot_id = r.next();
            let pid = r.next();
            let shiny_type = Raid::get_shiny_type(pid, ot_id);
            if shiny_type != 0 {
                return i;
            }
        }
        99999
    }
}

impl Display for Raid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Seed: {:0<16X}\tShinyType: {}\t EC: {:0<8X}\tPID: {:0<8X}",
            self.seed, self.shiny_type, self.ec, self.pid
        )?;
        writeln!(
            f,
            "Ability: {}\tGender: {}\tNature: {}\tIVs: {:?}",
            {
                if self.ability == 4 {
                    "H".to_string()
                } else {
                    self.ability.to_string()
                }
            },
            GENDER_SYMBOLS[self.gender as usize],
            NATURES[self.nature as usize].trim(),
            self.ivs
        )
    }
}
