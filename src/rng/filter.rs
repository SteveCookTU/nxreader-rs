use crate::util;

const SHINY_LIST: [&str; 3] = ["Star", "Square", "Star/Square"];

#[derive(Default)]
pub struct Filter {
    iv_min: Option<[u8; 6]>,
    iv_max: Option<[u8; 6]>,
    abilities: Option<Vec<u8>>,
    shininess: Option<Vec<usize>>,
    slot_min: Option<u8>,
    slot_max: Option<u8>,
    natures: Option<Vec<usize>>,
    marks: Option<Vec<usize>>,
    brilliant: Option<bool>,
    gender: Option<u8>,
    height_min: Option<u8>,
    height_max: Option<u8>,
    weight_min: Option<u8>,
    weight_max: Option<u8>,
}

impl Filter {
    pub fn set_iv_min(&mut self, iv_min: [u8; 6]) -> &mut Self {
        self.iv_min = Some(iv_min);
        self
    }

    pub fn set_iv_max(&mut self, iv_max: [u8; 6]) -> &mut Self {
        self.iv_max = Some(iv_max);
        self
    }

    pub fn set_abilities(&mut self, abilities: Vec<u8>) -> &mut Self {
        self.abilities = Some(abilities);
        self
    }

    pub fn set_shininess(&mut self, shininess: Vec<String>) -> &mut Self {
        self.shininess = Some(
            SHINY_LIST
                .iter()
                .enumerate()
                .filter_map(|(i, shiny)| {
                    if shininess.contains(&shiny.to_string()) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>(),
        );
        self
    }

    pub fn set_slot_min(&mut self, slot_min: u8) -> &mut Self {
        self.slot_min = Some(slot_min);
        self
    }

    pub fn set_slot_max(&mut self, slot_max: u8) -> &mut Self {
        self.slot_max = Some(slot_max);
        self
    }

    pub fn set_natures(&mut self, natures: Vec<String>) -> &mut Self {
        self.natures = Some(
            util::NATURES
                .iter()
                .enumerate()
                .filter_map(|(i, nature)| {
                    if natures.contains(&nature.to_string()) {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>(),
        );
        self
    }

    pub fn set_marks(&mut self, marks: Vec<usize>) -> &mut Self {
        self.marks = Some(marks);
        self
    }

    pub fn set_brilliant(&mut self, brilliant: bool) -> &mut Self {
        self.brilliant = Some(brilliant);
        self
    }

    pub fn set_gender(&mut self, gender: u8) -> &mut Self {
        self.gender = Some(gender);
        self
    }

    pub fn set_height_min(&mut self, height_min: u8) -> &mut Self {
        self.height_min = Some(height_min);
        self
    }

    pub fn set_height_max(&mut self, height_max: u8) -> &mut Self {
        self.height_min = Some(height_max);
        self
    }

    pub fn set_weight_min(&mut self, weight_min: u8) -> &mut Self {
        self.weight_min = Some(weight_min);
        self
    }

    pub fn set_weight_max(&mut self, weight_max: u8) -> &mut Self {
        self.weight_min = Some(weight_max);
        self
    }
}
