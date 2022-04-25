const GALAR_LIST: [u16; 19] = [
    52, 77, 78, 79, 80, 83, 110, 122, 144, 145, 146, 199, 222, 263, 264, 554, 555, 562, 618,
];
const ALOLA_LIST: [u16; 18] = [
    19, 20, 26, 27, 28, 37, 38, 50, 51, 52, 53, 74, 75, 76, 88, 89, 103, 105,
];

const PERSONAL_INFO_SIZE: usize = 0xB0;

pub struct PersonalInfo {
    data: Vec<u8>,
}

impl PersonalInfo {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn type_1(&self) -> u8 {
        self.data[0x06]
    }

    pub fn type_2(&self) -> u8 {
        self.data[0x07]
    }

    pub fn item_1(&self) -> u16 {
        u16::from_le_bytes((&self.data[0xC..0xE]).try_into().unwrap())
    }

    pub fn item_2(&self) -> u16 {
        u16::from_le_bytes((&self.data[0xE..0x10]).try_into().unwrap())
    }

    pub fn item_3(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x10..0x12]).try_into().unwrap())
    }

    pub fn gender(&self) -> u8 {
        self.data[0x12]
    }

    pub fn ability_1(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x18..0x1A]).try_into().unwrap())
    }

    pub fn ability_2(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x1A..0x1C]).try_into().unwrap())
    }

    pub fn ability_h(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x1C..0x1E]).try_into().unwrap())
    }

    pub fn abilities(&self) -> [u16; 3] {
        [self.ability_1(), self.ability_2(), self.ability_h()]
    }

    pub fn form_stats_index(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x1E..0x20]).try_into().unwrap())
    }

    pub fn form_count(&self) -> u8 {
        self.data[0x20]
    }

    pub fn form_index(&self, species: usize, form: usize) -> usize {
        if form == 0 {
            return species;
        }
        if self.form_stats_index() == 0 {
            return species;
        }
        if form > self.form_count() as usize {
            return species;
        }
        self.form_stats_index() as usize + form - 1
    }

    pub fn base_species(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x56..0x58]).try_into().unwrap())
    }

    pub fn base_species_form(&self) -> u16 {
        u16::from_le_bytes((&self.data[0x58..0x5A]).try_into().unwrap())
    }
}

pub struct PersonalTable {
    table: Vec<PersonalInfo>,
}

impl PersonalTable {
    pub fn new(data: Vec<u8>) -> Self {
        let table = data
            .chunks(PERSONAL_INFO_SIZE)
            .map(|chunk| PersonalInfo::new(chunk.to_vec()))
            .collect::<Vec<PersonalInfo>>();
        Self { table }
    }

    pub fn get_form_index(&self, mut species: usize, form: usize) -> usize {
        if species >= self.table.len() {
            species = 0;
        }
        self.table[species].form_index(species, form)
    }

    pub fn get_form_entry(&self, species: usize, form: usize) -> &PersonalInfo {
        &self.table[self.get_form_index(species, form)]
    }

    pub fn get_form_name_index(&self, species: usize, form: usize) -> Option<usize> {
        if species == 678 || species == 876 {
            if form != 0 {
                Some(1004)
            } else {
                Some(678)
            }
        } else if ALOLA_LIST.contains(&(species as u16)) && form <= 1 {
            if form != 0 {
                Some(810)
            } else {
                Some(1)
            }
        } else if GALAR_LIST.contains(&(species as u16)) {
            if form != 0 {
                Some(1068)
            } else {
                Some(1)
            }
        } else if form == 0 {
            Some(species)
        } else if species == 710 {
            Some(1005 + form)
        } else if species == 711 {
            Some(1008 + form)
        } else if species == 849 && form == 1 {
            Some(1072)
        } else if species == 869 {
            Some(1072 + form)
        } else if species == 479 {
            Some(916 + form)
        } else if [422, 423].contains(&(species as u16)) {
            Some(911)
        } else {
            None
        }
    }
}
