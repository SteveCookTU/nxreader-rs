use crate::structs::swsh::PersonalTable;
use lazy_static::lazy_static;

const ABILITIES_RAW: &str = include_str!("text_Abilities_en.txt");
const NATURES_RAW: &str = include_str!("text_Natures_en.txt");
const SPECIES_RAW: &str = include_str!("text_Species_en.txt");
const MOVES_RAW: &str = include_str!("text_Moves_en.txt");
const ITEMS_RAW: &str = include_str!("text_Items_en.txt");
const TYPES_RAW: &str = include_str!("text_Types_en.txt");
const FORMS_RAW: &[u8] = include_bytes!("text_Forms_en.txt");
const TR_TYPES_RAW: &str = include_str!("text_TRTypes_en.txt");
const TR_MOVES_RAW: &str = include_str!("text_TRMoves_en.txt");
pub const GENDER_SYMBOLS: [char; 3] = ['♂', '♀', '-'];
const PERSONAL_TABLE_RAW: &[u8] = include_bytes!("personal_swsh");

lazy_static! {
    pub static ref ABILITIES: Vec<&'static str> = ABILITIES_RAW.split('\n').collect::<Vec<&str>>();
    pub static ref NATURES: Vec<&'static str> = NATURES_RAW.split('\n').collect::<Vec<&str>>();
    pub static ref SPECIES: Vec<&'static str> = SPECIES_RAW.split('\n').collect::<Vec<&str>>();
    pub static ref MOVES: Vec<&'static str> = MOVES_RAW.split('\n').collect::<Vec<&str>>();
    pub static ref ITEMS: Vec<&'static str> = ITEMS_RAW.split('\n').collect::<Vec<&str>>();
    pub static ref FORMS: Vec<String> = {
        let forms_u16 = FORMS_RAW
            .chunks(2)
            .map(|chunk| {
                if chunk.len() == 2 {
                    ((chunk[0] as u16) << 2) + (chunk[1] as u16)
                } else {
                    chunk[0] as u16
                }
            })
            .collect::<Vec<u16>>();
        String::from_utf16_lossy(&forms_u16)
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    };
    pub static ref TR_TYPES: Vec<&'static str> = TR_TYPES_RAW.split('\n').collect::<Vec<&str>>();
    pub static ref TR_MOVES: Vec<&'static str> = TR_MOVES_RAW.split('\n').collect::<Vec<&str>>();
    pub static ref PERSONAL_TABLE: PersonalTable = PersonalTable::new(PERSONAL_TABLE_RAW.to_vec());
}
