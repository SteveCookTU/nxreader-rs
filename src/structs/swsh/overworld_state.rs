use crate::util::{GENDER_SYMBOLS, NATURES};
use std::fmt::{Display, Formatter};

#[derive(Default)]
pub struct OverworldState {
    advance: usize,
    full_seed: (u64, u64),
    pub(crate) fixed_seed: u32,
    is_static: bool,
    pub(crate) mark: Option<String>,
    pub(crate) brilliant: bool,
    hide_ability: bool,
    pub(crate) slot_rand: u8,
    level: u8,
    pub(crate) nature: u8,
    pub(crate) ability: u8,
    pub(crate) ec: u32,
    pub(crate) pid: u32,
    pub(crate) xor: u16,
    pub(crate) ivs: [u8; 6],
    pub(crate) gender: u8,
    pub(crate) height: Option<u8>,
    pub(crate) weight: Option<u8>,
}

impl OverworldState {
    pub fn set_full_seed(mut self, state: (u64, u64)) -> Self {
        self.full_seed = state;
        self
    }

    pub fn set_fixed_seed(mut self, seed: u32) -> Self {
        self.fixed_seed = seed;
        self
    }

    pub fn set_advance(mut self, advance: usize) -> Self {
        self.advance = advance;
        self
    }

    pub fn set_is_static(mut self, is_static: bool) -> Self {
        self.is_static = is_static;
        self
    }

    pub fn set_hide_ability(mut self, hide_ability: bool) -> Self {
        self.hide_ability = hide_ability;
        self
    }

    pub fn set_gender(mut self, gender: u8) -> Self {
        self.gender = gender;
        self
    }

    pub fn set_slot_rand(mut self, slot_rand: u8) -> Self {
        self.slot_rand = slot_rand;
        self
    }

    pub fn set_level(mut self, level: u8) -> Self {
        self.level = level;
        self
    }

    pub fn set_mark(mut self, mark: String) -> Self {
        self.mark = Some(mark);
        self
    }

    pub fn set_brilliant(mut self, brilliant: bool) -> Self {
        self.brilliant = brilliant;
        self
    }

    pub fn set_nature(mut self, nature: u8) -> Self {
        self.nature = nature;
        self
    }

    pub fn headings(&self) -> (Vec<String>, Vec<u8>) {
        let mut sizes = vec![100];
        let mut headings = vec!["Advance".to_string()];
        if !self.is_static {
            headings.extend_from_slice(&[
                "Level".to_string(),
                "Slot Rand".to_string(),
                "Aura".to_string(),
            ]);
            sizes.extend_from_slice(&[80, 80, 80]);
        }
        headings.extend_from_slice(&[
            "EC".to_string(),
            "PID".to_string(),
            "Shiny".to_string(),
            "Nature".to_string(),
        ]);
        sizes.extend_from_slice(&[100, 100, 60, 60]);

        if !self.hide_ability {
            headings.extend_from_slice(&["Ability".to_string()]);
            sizes.extend_from_slice(&[60]);
        }

        headings.extend_from_slice(&[
            "Gender".to_string(),
            "IVs".to_string(),
            "Mark".to_string(),
            "Height".to_string(),
            "Weight".to_string(),
        ]);
        sizes.extend_from_slice(&[60, 100, 60, 60, 60]);

        (headings, sizes)
    }

    pub fn row(&self) -> Vec<String> {
        let mut vec = vec![self.advance.to_string()];
        if !self.is_static {
            vec.extend_from_slice(&[
                self.level.to_string(),
                self.slot_rand.to_string(),
                self.brilliant.to_string(),
            ]);
        }
        vec.extend_from_slice(&[
            format!("{:0>8X}", self.ec),
            format!("{:0>8X}", self.ec),
            {
                if self.xor >= 16 {
                    "No".to_string()
                } else if self.xor == 0 {
                    "Square".to_string()
                } else {
                    "Star".to_string()
                }
            },
            NATURES[self.nature as usize].trim().to_string(),
        ]);
        if !self.hide_ability {
            vec.push(self.ability.to_string());
        }
        vec.extend_from_slice(&[
            GENDER_SYMBOLS[self.gender as usize].to_string(),
            {
                self.ivs
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join("/")
            },
            {
                if let Some(mark) = self.mark.as_ref() {
                    mark.to_string()
                } else {
                    "None".to_string()
                }
            },
            {
                if let Some(height) = self.height {
                    height.to_string()
                } else {
                    0.to_string()
                }
            },
            {
                if let Some(weight) = self.weight {
                    weight.to_string()
                } else {
                    0.to_string()
                }
            },
        ]);
        vec
    }
}

impl Display for OverworldState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.is_static {
            write!(
                f,
                "{} {:0>8X} {:0>8X} {} {} {}{} {} {} {}/{}",
                self.advance,
                self.ec,
                self.pid,
                {
                    if self.xor >= 16 {
                        "No"
                    } else if self.xor == 0 {
                        "Square"
                    } else {
                        "Star"
                    }
                },
                NATURES[self.nature as usize].trim(),
                {
                    if !self.hide_ability {
                        format!("{} ", self.ability)
                    } else {
                        "".to_string()
                    }
                },
                GENDER_SYMBOLS[self.gender as usize],
                {
                    self.ivs
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join("/")
                },
                {
                    if let Some(mark) = self.mark.as_ref() {
                        mark.to_string()
                    } else {
                        "None".to_string()
                    }
                },
                {
                    if let Some(height) = self.height {
                        height
                    } else {
                        0
                    }
                },
                {
                    if let Some(weight) = self.weight {
                        weight
                    } else {
                        0
                    }
                }
            )
        } else {
            write!(
                f,
                "{} {} {} {}{:0>8X} {:0>8X} {} {} {}{} {} {} {}/{}",
                self.advance,
                self.level,
                self.slot_rand,
                {
                    if self.brilliant {
                        "Brilliant! "
                    } else {
                        ""
                    }
                },
                self.ec,
                self.pid,
                {
                    if self.xor >= 16 {
                        "No"
                    } else if self.xor == 0 {
                        "Square"
                    } else {
                        "Star"
                    }
                },
                NATURES[self.nature as usize].trim(),
                {
                    if !self.hide_ability {
                        format!("{} ", self.ability)
                    } else {
                        "".to_string()
                    }
                },
                GENDER_SYMBOLS[self.gender as usize],
                {
                    self.ivs
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join("/")
                },
                {
                    if let Some(mark) = self.mark.as_ref() {
                        mark.to_string()
                    } else {
                        "None".to_string()
                    }
                },
                {
                    if let Some(height) = self.height {
                        height
                    } else {
                        0
                    }
                },
                {
                    if let Some(weight) = self.weight {
                        weight
                    } else {
                        0
                    }
                }
            )
        }
    }
}
