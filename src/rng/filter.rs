use crate::structs::swsh::OverworldState;
use crate::util;

const SHINY_LIST: [&str; 3] = ["Star", "Square", "Star/Square"];

#[derive(Default)]
pub struct Filter {
    iv_min: Option<[u8; 6]>,
    iv_max: Option<[u8; 6]>,
    abilities: Option<Vec<u8>>,
    shininess: Option<usize>,
    slot_min: Option<u8>,
    slot_max: Option<u8>,
    natures: Option<Vec<u8>>,
    marks: Option<Vec<String>>,
    brilliant: Option<bool>,
    gender: Option<u8>,
    height_min: Option<u8>,
    height_max: Option<u8>,
    weight_min: Option<u8>,
    weight_max: Option<u8>,
}

impl Filter {
    pub fn set_iv_min(mut self, iv_min: [u8; 6]) -> Self {
        self.iv_min = Some(iv_min);
        self
    }

    pub fn set_iv_max(mut self, iv_max: [u8; 6]) -> Self {
        self.iv_max = Some(iv_max);
        self
    }

    pub fn set_abilities(mut self, abilities: Vec<u8>) -> Self {
        self.abilities = Some(abilities);
        self
    }

    pub fn set_shininess(mut self, shininess: usize) -> Self {
        self.shininess = Some(shininess);
        self
    }

    pub fn set_slot_min(mut self, slot_min: u8) -> Self {
        self.slot_min = Some(slot_min);
        self
    }

    pub fn set_slot_max(mut self, slot_max: u8) -> Self {
        self.slot_max = Some(slot_max);
        self
    }

    pub fn set_natures(mut self, natures: Vec<String>) -> Self {
        self.natures = Some(
            util::NATURES
                .iter()
                .enumerate()
                .filter_map(|(i, nature)| {
                    if natures.contains(&nature.to_string()) {
                        Some(i as u8)
                    } else {
                        None
                    }
                })
                .collect::<Vec<u8>>(),
        );
        self
    }

    pub fn set_marks(mut self, marks: Vec<String>) -> Self {
        self.marks = Some(marks);
        self
    }

    pub fn set_brilliant(mut self, brilliant: bool) -> Self {
        self.brilliant = Some(brilliant);
        self
    }

    pub fn set_gender(mut self, gender: u8) -> Self {
        self.gender = Some(gender);
        self
    }

    pub fn set_height_min(mut self, height_min: u8) -> Self {
        self.height_min = Some(height_min);
        self
    }

    pub fn set_height_max(mut self, height_max: u8) -> Self {
        self.height_min = Some(height_max);
        self
    }

    pub fn set_weight_min(mut self, weight_min: u8) -> Self {
        self.weight_min = Some(weight_min);
        self
    }

    pub fn set_weight_max(mut self, weight_max: u8) -> Self {
        self.weight_min = Some(weight_max);
        self
    }

    pub fn compare_ivs(&self, state: &OverworldState) -> bool {
        if let Some(iv_min) = self.iv_min {
            if let Some(iv_max) = self.iv_max {
                for i in 0..6 {
                    if !(iv_min[i]..=iv_max[i]).contains(&state.ivs[i]) {
                        return false;
                    }
                }
            }
        }
        true
    }

    pub fn compare_brilliant(&self, state: &OverworldState) -> bool {
        if let Some(brilliant) = self.brilliant {
            return state.brilliant || !brilliant;
        }

        true
    }

    pub fn compare_fixed(&self, state: &OverworldState) -> bool {
        if let Some(shininess) = self.shininess {
            if shininess == 0 && state.xor == 0 {
                return false;
            }
        }

        if let Some(weight_min) = self.weight_min {
            if let Some(weight_max) = self.weight_max {
                if let Some(height_min) = self.height_min {
                    if let Some(height_max) = self.height_max {
                        if let Some(weight) = state.weight {
                            if let Some(height) = state.height {
                                if !((weight_min..=weight_max).contains(&weight)
                                    && (height_min..=height_max).contains(&height))
                                {
                                    return false;
                                }
                            }
                        }
                    }
                }
            }
        }

        self.compare_ivs(state)
    }

    pub fn compare_slot(&self, state: &OverworldState) -> bool {
        if let Some(slot_min) = self.slot_min {
            if let Some(slot_max) = self.slot_max {
                return (slot_min..=slot_max).contains(&state.slot_rand);
            }
        }

        true
    }

    pub fn compare_mark(&self, state: &OverworldState) -> bool {
        if let Some(marks) = &self.marks {
            if let Some(mark) = &state.mark {
                return marks.contains(mark);
            }
        }
        true
    }

    pub fn compare_shiny(&self, shiny: bool) -> bool {
        if self.shininess.is_some() {
            return shiny;
        }
        true
    }

    pub fn compare_ability(&self, state: &OverworldState) -> bool {
        if let Some(abilities) = &self.abilities {
            return abilities.contains(&state.ability);
        }
        true
    }

    pub fn compare_nature(&self, state: &OverworldState) -> bool {
        if let Some(natures) = &self.natures {
            return natures.contains(&state.nature);
        }
        true
    }

    pub fn compare_gender(&self, state: &OverworldState) -> bool {
        !(self.gender.is_some() && self.gender.unwrap() != state.gender)
    }
}
