use crate::rng::{Filter, Xoroshiro};
use crate::structs::pk8_overworld::PK8;
use crate::structs::swsh::OverworldState;

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

#[derive(Default)]
pub struct OverworldRNG {
    rng: Xoroshiro,
    pub advance: usize,
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
    cute_charm: Option<bool>,
    filter: Filter,
}

impl OverworldRNG {
    pub fn set_state(mut self, s0: u64, s1: u64) -> Self {
        self.rng = Xoroshiro::from_state(s0, s1);
        self
    }

    pub fn set_tid(mut self, tid: u16) -> Self {
        self.tid = tid;
        self
    }

    pub fn set_sid(mut self, sid: u16) -> Self {
        self.sid = sid;
        self
    }

    pub fn set_shiny_charm(mut self, shiny_charm: bool) -> Self {
        self.shiny_charm = shiny_charm;
        self
    }

    pub fn set_mark_charm(mut self, mark_charm: bool) -> Self {
        self.mark_charm = mark_charm;
        self
    }

    pub fn set_weather_active(mut self, weather_active: bool) -> Self {
        self.weather_active = weather_active;
        self
    }

    pub fn set_is_fishing(mut self, is_fishing: bool) -> Self {
        self.is_fishing = is_fishing;
        self
    }

    pub fn set_is_static(mut self, is_static: bool) -> Self {
        self.is_static = is_static;
        self
    }

    pub fn set_forced_ability(mut self, forced_ability: bool) -> Self {
        self.forced_ability = forced_ability;
        self
    }

    pub fn set_flawless_ivs(mut self, flawless_ivs: u8) -> Self {
        self.flawless_ivs = flawless_ivs;
        self
    }

    pub fn set_is_shiny_locked(mut self, is_shiny_locked: bool) -> Self {
        self.is_shiny_locked = is_shiny_locked;
        self
    }

    pub fn set_min_level(mut self, min_level: u8) -> Self {
        self.min_level = min_level;
        self
    }

    pub fn set_max_level(mut self, max_level: u8) -> Self {
        self.max_level = max_level;
        self
    }

    pub fn set_diff_held_item(mut self, diff_held_item: bool) -> Self {
        self.diff_held_item = diff_held_item;
        self
    }

    pub fn set_egg_move_count(mut self, egg_move_count: u8) -> Self {
        self.egg_move_count = egg_move_count;
        self
    }

    pub fn set_brilliant_info(mut self, kos: u16) -> Self {
        (self.brilliant_thresh, self.brilliant_rolls) = OverworldRNG::calculate_brilliant_info(kos);
        self
    }

    pub fn set_cute_charm(mut self, cute_charm: bool) -> Self {
        self.cute_charm = Some(cute_charm);
        self
    }

    pub fn set_filter(mut self, filter: Filter) -> Self {
        self.filter = filter;
        self
    }

    pub fn calculate_brilliant_info(kos: u16) -> (u16, u8) {
        if kos >= 500 {
            (30, 6)
        } else if kos >= 300 {
            (30, 5)
        } else if kos >= 200 {
            (30, 4)
        } else if kos >= 100 {
            (30, 3)
        } else if kos >= 50 {
            (25, 2)
        } else if kos >= 20 {
            (20, 1)
        } else if kos >= 1 {
            (15, 1)
        } else {
            (0, 0)
        }
    }

    pub fn tsv(&self) -> u16 {
        self.tid ^ self.sid
    }

    pub fn advance_fast(&mut self, advances: usize) {
        self.advance += advances;
        for _ in 0..advances {
            self.rng.next();
        }
    }

    pub fn generate(&mut self) -> Option<OverworldState> {
        let state = self.generate_filter();
        self.rng.next();
        self.advance += 1;
        state
    }

    pub fn generate_filter(&mut self) -> Option<OverworldState> {
        let mut state = OverworldState::default()
            .set_full_seed(self.rng.get_state())
            .set_advance(self.advance)
            .set_is_static(self.is_static)
            .set_hide_ability(self.forced_ability);

        let mut go = self.rng;

        if self.is_static {
            let lead_rand = go.rand_max(100);
            if let Some(cute_charm) = self.cute_charm {
                if lead_rand <= 65 {
                    if !cute_charm {
                        state = state.set_gender(1);
                    } else {
                        state = state.set_gender(0);
                    }
                    if !self.filter.compare_gender(&state) {
                        return None;
                    }
                }
            }
        } else {
            if !self.is_fishing {
                go.next();
            }
            go.rand_max(100);
            let lead_rand = go.rand_max(100);
            if let Some(cute_charm) = self.cute_charm {
                if lead_rand <= 65 {
                    if !cute_charm {
                        state = state.set_gender(1);
                    } else {
                        state = state.set_gender(0);
                    }
                    if !self.filter.compare_gender(&state) {
                        return None;
                    }
                }
            }

            state = state.set_slot_rand(go.rand_max(100) as u8);

            if !self.filter.compare_slot(&state) {
                return None;
            }

            if self.min_level != self.max_level {
                state = state.set_level(
                    self.min_level
                        + go.rand_max((self.max_level - self.min_level) as u32 + 1) as u8,
                );
            } else {
                state = state.set_level(self.min_level);
            }

            state = state.set_mark(OverworldRNG::rand_mark(
                &mut go,
                self.weather_active,
                self.is_fishing,
                self.mark_charm,
            ));
            let brilliant_rand = go.rand_max(1000) as u16;
            if brilliant_rand < self.brilliant_thresh {
                state = state.set_brilliant(true);
            }
            if !self.filter.compare_brilliant(&state) {
                return None;
            }
        }

        let mut shiny = false;
        if !self.is_shiny_locked {
            let rolls = {
                if self.shiny_charm {
                    3
                } else {
                    1
                }
            } + {
                if state.brilliant {
                    self.brilliant_rolls
                } else {
                    0
                }
            };

            for _ in 0..rolls {
                let mock_pid = go.next();
                shiny = (((mock_pid >> 16) ^ (mock_pid & 0xFFFF)) as u16 ^ self.tsv()) < 16;
                if shiny {
                    break;
                }
            }
        }

        if !self.filter.compare_shiny(shiny) {
            return None;
        }

        if state.gender == 2 {
            state = state.set_gender({
                if go.rand_max(2) == 0 {
                    1
                } else {
                    0
                }
            });
        }
        if !self.filter.compare_gender(&state) {
            return None;
        }

        state = state.set_nature(go.rand_max(25) as u8);

        if !self.filter.compare_nature(&state) {
            return None;
        }

        if !self.is_static && self.diff_held_item {
            go.rand_max(100);
        }

        let mut brilliant_ivs = 0;
        if state.brilliant {
            brilliant_ivs = go.rand_max(2) | 2;
            if self.egg_move_count > 1 {
                go.rand_max(self.egg_move_count as u32);
            }
        }

        state = state.set_fixed_seed(go.next());
        (state.ec, state.pid, state.ivs, state.height, state.weight) =
            OverworldRNG::calculate_fixed(
                state.fixed_seed as u64,
                self.tsv(),
                shiny,
                self.flawless_ivs + brilliant_ivs as u8,
            );
        state.xor = ((state.pid >> 16) ^ (state.pid & 0xFFFF)) as u16 ^ self.tsv();
        if !self.filter.compare_fixed(&state) {
            return None;
        }

        state = state.set_mark(OverworldRNG::rand_mark(
            &mut go,
            self.weather_active,
            self.is_fishing,
            self.mark_charm,
        ));

        if !self.filter.compare_mark(&state) {
            return None;
        }

        self.rng = go;
        Some(state)
    }

    pub fn rand_mark(
        go: &mut Xoroshiro,
        weather_active: bool,
        is_fishing: bool,
        mark_charm: bool,
    ) -> String {
        let max = if mark_charm { 3 } else { 1 };

        for _ in 0..max {
            let rare_rand = go.rand_max(1000);
            let personality_rand = go.rand_max(100);
            let uncommon_rand = go.rand_max(50);
            let weather_rand = go.rand_max(50);
            let time_rand = go.rand_max(50);
            let fish_rand = go.rand_max(25);

            if rare_rand == 0 {
                return "Rare".to_string();
            } else if personality_rand == 0 {
                return PERSONALITY_MARKS[go.rand_max(PERSONALITY_MARKS.len() as u32) as usize]
                    .to_string();
            } else if uncommon_rand == 0 {
                return "Uncommon".to_string();
            } else if weather_rand == 0 && weather_active {
                return "Weather".to_string();
            } else if time_rand == 0 {
                return "Time".to_string();
            } else if fish_rand == 0 && is_fishing {
                return "Fishing".to_string();
            }
        }
        "".to_string()
    }

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
    ) -> (u32, u32, [u8; 6], Option<u8>, Option<u8>) {
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

        (ec, pid, ivs, Some(height), Some(weight))
    }
}
