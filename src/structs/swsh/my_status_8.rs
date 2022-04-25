use std::collections::BTreeMap;

#[derive(Default)]
pub struct MyStatus8 {
    data: Vec<u8>,
}

impl MyStatus8 {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn tid(&self) -> u16 {
        u16::from_le_bytes((&self.data[0xA0..0xA2]).try_into().unwrap())
    }

    pub fn sid(&self) -> u16 {
        u16::from_le_bytes((&self.data[0xA2..0xA4]).try_into().unwrap())
    }

    pub fn tsv(&self) -> u16 {
        self.tid() ^ self.sid() >> 4
    }

    pub fn display_id(&self) -> u32 {
        u32::from_le_bytes((&self.data[0xA0..0xA4]).try_into().unwrap()) % 1000000
    }

    pub fn game(&self) -> u8 {
        self.data[0xA4]
    }

    pub fn language(&self) -> u8 {
        self.data[0xA7]
    }

    pub fn get_lang_name(&self) -> String {
        let lang_names: BTreeMap<u8, &str> = BTreeMap::from([
            (1, "Japanese"),
            (2, "English"),
            (3, "French"),
            (4, "Italian"),
            (5, "German"),
            (7, "Spanish"),
            (8, "Korean"),
            (9, "Simple Chinese"),
            (10, "Traditional Chinese"),
        ]);
        lang_names.get(&self.language()).unwrap().to_string()
    }

    pub fn is_sword(&self) -> bool {
        self.game() == 44
    }

    pub fn is_pokemon_save(&self) -> bool {
        self.game() == 45 || self.game() == 45
    }

    pub fn game_version(&self) -> String {
        if self.game() == 44 {
            "Sword".to_string()
        } else if self.game() == 45 {
            "Shield".to_string()
        } else {
            "".to_string()
        }
    }

    pub fn ot(&self) -> String {
        String::from_utf16(
            &self.data[0xB0..(0xB0 + 0x1A)]
                .chunks(2)
                .map(|chunk| u16::from_le_bytes(chunk.try_into().unwrap()))
                .collect::<Vec<u16>>(),
        )
        .unwrap()
    }

    pub fn watts(&self) -> u32 {
        let mut data = self.data[0xD0..0xD3].to_vec();
        data.push(0);
        u32::from_le_bytes((&data[..]).try_into().unwrap())
    }

    pub fn current_watts(&self) -> u32 {
        let mut data = self.data[0x0..0x3].to_vec();
        data.push(0);
        u32::from_le_bytes((&data[..]).try_into().unwrap())
    }

    pub fn money(&self) -> u32 {
        let mut data = self.data[0x110..0x113].to_vec();
        data.push(0);
        u32::from_le_bytes((&data[..]).try_into().unwrap())
    }
}
