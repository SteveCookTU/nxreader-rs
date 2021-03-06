use crate::structs::swsh::encounter_nest_8_archive_generated::structure::{
    EncounterNest8, EncounterNest8Archive,
};
use crate::structs::swsh::nest_hole_crystal_encounter_8_archive_generated::structure::NestHoleCrystalEncounter8;
use crate::structs::swsh::nest_hole_distribution_encounter_8_archive_generated::structure::{
    root_as_nest_hole_distribution_encounter_8archive, NestHoleDistributionEncounter8,
};
use crate::structs::swsh::nest_hole_reward_8_archive_generated::structure::NestHoleReward8Archive;
use crate::structs::swsh::{
    encounter_nest_8_archive_generated, nest_hole_reward_8_archive_generated,
};
use lazy_static::lazy_static;
use std::fs::File;
use std::io::Read;

pub const DEN_COUNT: usize = 276;
pub const LOCAL_TABLE_RAW: &[u8] = include_bytes!("resources/local_raid");
pub const LOCAL_DROPS_RAW: &[u8] = include_bytes!("resources/local_drop");
pub const LOCAL_BONUS_RAW: &[u8] = include_bytes!("resources/local_bonus");

const EVENT_HASH: u64 = 1721953670860364124;
const DEN_HASHES: [[u64; 2]; 276] = [
    [1675062357515959378, 13439833545771248589],
    [1676893044376552243, 13440787921864346512],
    [1676899641446321509, 4973137107049022145],
    [1676044221399762576, 13438834089701394015],
    [1676051917981160053, 13438837388236278648],
    [1676897442423065087, 13440790120887602934],
    [1676908437539347197, 13440789021375974723],
    [1676046420423018998, 13438839587259535070],
    [1676899641446321509, 4973137107049022145],
    [1677896898492919661, 13439825849189851112],
    [1677881505330124707, 4972153044141962525],
    [1677896898492919661, 13439826948701479323],
    [1676051917981160053, 4973134908025765723],
    [1677896898492919661, 13439825849189851112],
    [1676045320911390787, 13438832990189765804],
    [1676049718957903631, 13438838487747906859],
    [EVENT_HASH, EVENT_HASH],
    [1676048619446275420, 13438843985306047914],
    [1676908437539347197, 13440789021375974723],
    [1676899641446321509, 13439823650166594690],
    [1676899641446321509, 13439823650166594690],
    [1676055216516044686, 13441642242399277234],
    [1676055216516044686, 13441642242399277234],
    [1679871621376808167, 13438843985306047914],
    [1676048619446275420, 13438843985306047914],
    [1676055216516044686, 4973136007537393934],
    [1676895243399808665, 13440791220399231145],
    [1676907338027718986, 13440787921864346512],
    [1676056316027672897, 4973136007537393934],
    [1679872720888436378, 13441636744841136179],
    [1679872720888436378, 13441636744841136179],
    [1676050818469531842, 13438837388236278648],
    [1676046420423018998, 13438842885794419703],
    [1675061258004331167, 13438834089701394015],
    [1675057959469446534, 13438845084817676125],
    [1675056859957818323, 13438840686771163281],
    [1675061258004331167, 4972148646095449681],
    [1675056859957818323, 4972140949514052204],
    [1675055760446190112, 13438839587259535070],
    [1679872720888436378, 13441636744841136179],
    [1677880405818496496, 13439824749678222901],
    [1679872720888436378, 13441636744841136179],
    [1677880405818496496, 13439824749678222901],
    [1677880405818496496, 4973141505095534989],
    [1675055760446190112, 13438839587259535070],
    [1675060158492702956, 13438832990189765804],
    [1676898541934693298, 13439824749678222901],
    [1677894699469663239, 13439829147724735745],
    [1679873820400064589, 13440789021375974723],
    [1676894143888180454, 4972147546583821470],
    [1675059058981074745, 4973140405583906778],
    [1676056316027672897, 13438843985306047914],
    [1675062357515959378, 13439833545771248589],
    [1679873820400064589, 13440789021375974723],
    [1676051917981160053, 4973134908025765723],
    [1676050818469531842, 13438837388236278648],
    [1676891944864924032, 13440791220399231145],
    [1677895798981291450, 13439825849189851112],
    [1679873820400064589, 13440794518934115778],
    [1676046420423018998, 4972146447072193259],
    [1676044221399762576, 13438834089701394015],
    [1675065656050844011, 4972145347560565048],
    [1676049718957903631, 13438842885794419703],
    [1677895798981291450, 13439825849189851112],
    [1676045320911390787, 13438832990189765804],
    [1675057959469446534, 4972142049025680415],
    [1677892500446406817, 13439830247236363956],
    [1675060158492702956, 13438832990189765804],
    [1675064556539215800, 13439831346747992167],
    [1676895243399808665, 13440791220399231145],
    [1675063457027587589, 4973133808514137512],
    [1675063457027587589, 13439833545771248589],
    [1675061258004331167, 4973133808514137512],
    [1676055216516044686, 13441642242399277234],
    [1675056859957818323, 13438840686771163281],
    [1675055760446190112, 13438839587259535070],
    [1677889201911522184, 13439830247236363956],
    [1677890301423150395, 13439831346747992167],
    [1677881505330124707, 13438842885794419703],
    [1676891944864924032, 4973139306072278567],
    [1679871621376808167, 13440795618445743989],
    [1676891944864924032, 13440793419422487567],
    [1677895798981291450, 13440798916980628622],
    [1677893599958035028, 13441641142887649023],
    [1675057959469446534, 13438845084817676125],
    [1676896342911436876, 13439823650166594690],
    [1676898541934693298, 13439828048213107534],
    [1675065656050844011, 13439832446259620378],
    [1677891400934778606, 13441640043376020812],
    [1676897442423065087, 13440790120887602934],
    [1675060158492702956, 13440792319910859356],
    [1676898541934693298, 13439824749678222901],
    [1677891400934778606, 13439830247236363956],
    [1675064556539215800, 13440800016492256833],
    [1676896342911436876, 4973138206560650356],
    [1677894699469663239, 4972151944630334314],
    [1677893599958035028, 13439829147724735745],
    [1675064556539215800, 4972150845118706103],
    [1676056316027672897, 13438843985306047914],
    [1676894143888180454, 13441643341910905445],
    [8769170721942624824, 14477537978666912344],
    [16341001078884806474, 9913932150092391706],
    [7854659797556875545, 5999950843982638879],
    [4780541378243794326, 18345017229883237822],
    [2997411918588892139, 12562706121429926817],
    [6589539950519384197, 3561902408726248099],
    [2447364886159768926, 15632276665898509590],
    [7956530560371257544, 2024757571205803752],
    [13563999851587423716, 502513031628180988],
    [4780539179220537904, 18345015030859981400],
    [4780540278732166115, 18345016130371609611],
    [2997411918588892139, 12562706121429926817],
    [16341001078884806474, 9913932150092391706],
    [14284833672245134656, 7704513452465554544],
    [6672704941776910536, 17951961757311600360],
    [13305292637317525948, 16069264858016261892],
    [2447363786648140715, 15632275566386881379],
    [2447364886159768926, 15632276665898509590],
    [4780541378243794326, 18345017229883237822],
    [7854659797556875545, 5999950843982638879],
    [15818376695778914966, 5701088864462885848],
    [7956530560371257544, 2024757571205803752],
    [16341001078884806474, 9913932150092391706],
    [6672704941776910536, 17951961757311600360],
    [4780540278732166115, 18345016130371609611],
    [6589539950519384197, 3561902408726248099],
    [4780540278732166115, 18345016130371609611],
    [7956530560371257544, 2024757571205803752],
    [13563999851587423716, 502513031628180988],
    [6984833918694526192, 14413583907274219616],
    [4780539179220537904, 18345015030859981400],
    [13305292637317525948, 16069264858016261892],
    [342604449375897784, 8253110425161551320],
    [5830741396702654597, 17953607996949684899],
    [13563999851587423716, 502513031628180988],
    [6162140483756004486, 6162171270081594394],
    [11635283243122928556, 17629394089387610164],
    [14284833672245134656, 7704513452465554544],
    [6984833918694526192, 14413583907274219616],
    [4780540278732166115, 5701094362021026903],
    [342604449375897784, 8253110425161551320],
    [5830741396702654597, 17953607996949684899],
    [4780541378243794326, 18345017229883237822],
    [2447363786648140715, 15632275566386881379],
    [6589539950519384197, 3561902408726248099],
    [12738905581603037598, 5701095461532655114],
    [4780539179220537904, 18345015030859981400],
    [11635283243122928556, 17629394089387610164],
    [6672704941776910536, 17951961757311600360],
    [15818376695778914966, 5701088864462885848],
    [13305292637317525948, 16069264858016261892],
    [8769170721942624824, 14477537978666912344],
    [2997411918588892139, 12562706121429926817],
    [7854659797556875545, 5701093262509398692],
    [2447363786648140715, 15632275566386881379],
    [6984833918694526192, 5701096561044283325],
    [6589539950519384197, 3561902408726248099],
    [8769170721942624824, 14477537978666912344],
    [7725829814153603264, 5701092162997770481],
    [4780546875801935381, 18345022727441378877],
    [4665094036540599430, 11519945754184084270],
    [14284833672245134656, 7704513452465554544],
    [7854659797556875545, 5999950843982638879],
    [11635283243122928556, 17629394089387610164],
    [12738905581603037598, 4426791916416848726],
    [6984833918694526192, 14413583907274219616],
    [13305292637317525948, 16069264858016261892],
    [7725829814153603264, 5701092162997770481],
    [6672704941776910536, 17951961757311600360],
    [5830741396702654597, 17953607996949684899],
    [2447364886159768926, 15632276665898509590],
    [342604449375897784, 8253110425161551320],
    [4780546875801935381, 18345022727441378877],
    [11635283243122928556, 17629394089387610164],
    [16341001078884806474, 9913932150092391706],
    [2447364886159768926, 15632276665898509590],
    [2997411918588892139, 12562706121429926817],
    [4780546875801935381, 18345022727441378877],
    [4780539179220537904, 5701091063486142270],
    [12738905581603037598, 4426791916416848726],
    [13563999851587423716, 502513031628180988],
    [14284833672245134656, 7704513452465554544],
    [4780546875801935381, 18345022727441378877],
    [7956530560371257544, 2024757571205803752],
    [16882931869395424672, 4515385547978135952],
    [16882931869395424672, 4515385547978135952],
    [16882931869395424672, 4515385547978135952],
    [16882931869395424672, 4515385547978135952],
    [16882931869395424672, 4515385547978135952],
    [16882931869395424672, 4515385547978135952],
    [538718828553644332, 10639252279486991937],
    [6189149299220963515, 744948697234498138],
    [7520360650147352417, 3231560995259522968],
    [2756478418053350351, 4769195437400348422],
    [5162770839310267307, 11690997354028679946],
    [7520360650147352417, 3231560995259522968],
    [14439216054291849305, 8284890978883698976],
    [4805937820974168436, 11331443048367529433],
    [11147942343095866771, 1812702195150859522],
    [8444690290455066916, 221992188589330697],
    [16299909383459599211, 4268295780237511370],
    [9125837977236588438, 16150871691787878075],
    [4197853775535533550, 7797506443826343779],
    [5955975221769392477, 14450795946632079964],
    [17302261471610567686, 10041392713565152107],
    [2756478418053350351, 4769195437400348422],
    [1108881309583387371, 2845993206239293002],
    [4408860220788168599, 18001771904838230654],
    [8444690290455066916, 221992188589330697],
    [538718828553644332, 10639252279486991937],
    [6189149299220963515, 744948697234498138],
    [5955975221769392477, 14450795946632079964],
    [11147942343095866771, 1812702195150859522],
    [14439216054291849305, 8284890978883698976],
    [6189149299220963515, 744948697234498138],
    [7520357351612467784, 1345818289025324965],
    [1108881309583387371, 2845993206239293002],
    [7520357351612467784, 1345818289025324965],
    [1716759284250366303, 12829170745926812758],
    [16299909383459599211, 4268295780237511370],
    [4197853775535533550, 7797506443826343779],
    [4805937820974168436, 11331443048367529433],
    [5162770839310267307, 11690997354028679946],
    [17302261471610567686, 10041392713565152107],
    [6189149299220963515, 744948697234498138],
    [8444690290455066916, 221992188589330697],
    [9125837977236588438, 16150871691787878075],
    [1716759284250366303, 12829170745926812758],
    [11147942343095866771, 1812702195150859522],
    [7520360650147352417, 3231560995259522968],
    [14439216054291849305, 8284890978883698976],
    [4197853775535533550, 7797506443826343779],
    [6395957127820208723, 13032247726971474370],
    [9125837977236588438, 16150871691787878075],
    [4408860220788168599, 18001771904838230654],
    [1716759284250366303, 12829170745926812758],
    [538718828553644332, 10639252279486991937],
    [2756478418053350351, 4769195437400348422],
    [1108881309583387371, 2845993206239293002],
    [11147942343095866771, 1812702195150859522],
    [538718828553644332, 10639252279486991937],
    [5162770839310267307, 11690997354028679946],
    [7520357351612467784, 1345818289025324965],
    [4408860220788168599, 18001771904838230654],
    [1108881309583387371, 2845993206239293002],
    [8444690290455066916, 221992188589330697],
    [9125837977236588438, 16150871691787878075],
    [7520357351612467784, 1345818289025324965],
    [5955975221769392477, 14450795946632079964],
    [5162770839310267307, 11690997354028679946],
    [11147942343095866771, 1812702195150859522],
    [14439216054291849305, 8284890978883698976],
    [7520360650147352417, 3231560995259522968],
    [16299909383459599211, 4268295780237511370],
    [4805937820974168436, 11331443048367529433],
    [7520357351612467784, 1345818289025324965],
    [4408860220788168599, 18001771904838230654],
    [16299909383459599211, 4268295780237511370],
    [2756478418053350351, 4769195437400348422],
    [4805937820974168436, 11331443048367529433],
    [5955975221769392477, 14450795946632079964],
    [6189149299220963515, 744948697234498138],
    [4197853775535533550, 7797506443826343779],
    [4197853775535533550, 7797506443826343779],
    [538718828553644332, 10639252279486991937],
    [5955975221769392477, 14450795946632079964],
    [16299909383459599211, 4268295780237511370],
    [8444690290455066916, 221992188589330697],
    [16685003352010291762, 13686551123076485279],
    [17302261471610567686, 10041392713565152107],
    [14439216054291849305, 8284890978883698976],
    [1108881309583387371, 2845993206239293002],
    [1716759284250366303, 12829170745926812758],
    [5162770839310267307, 11690997354028679946],
    [4408860220788168599, 18001771904838230654],
    [4805937820974168436, 11331443048367529433],
];

lazy_static! {
    pub static ref LOCAL_TABLE: EncounterNest8Archive<'static> =
        encounter_nest_8_archive_generated::structure::root_as_encounter_nest_8archive(
            LOCAL_TABLE_RAW
        )
        .expect("Invalid flatbuffer");
    pub static ref LOCAL_DROPS: NestHoleReward8Archive<'static> =
        nest_hole_reward_8_archive_generated::structure::root_as_nest_hole_reward_8archive(
            LOCAL_DROPS_RAW
        )
        .expect("Invalid flatbuffer");
    pub static ref LOCAL_BONUS: NestHoleReward8Archive<'static> =
        nest_hole_reward_8_archive_generated::structure::root_as_nest_hole_reward_8archive(
            LOCAL_BONUS_RAW
        )
        .expect("Invalid flatbuffer");
}

pub const DEN_SIZE: usize = 0x18;

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum DenType {
    Empty,
    Common,
    Rare,
    CommonWish,
    RareWish,
    Event,
}

impl TryFrom<u8> for DenType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DenType::Empty),
            1 => Ok(DenType::Common),
            2 => Ok(DenType::Rare),
            3 => Ok(DenType::CommonWish),
            4 => Ok(DenType::RareWish),
            5 => Ok(DenType::Event),
            _ => Err("Failed to convert to DenType"),
        }
    }
}

pub struct Den {
    data: Vec<u8>,
}

impl Den {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn hash(&self) -> u64 {
        u64::from_le_bytes((&self.data[0x0..0x8]).try_into().unwrap())
    }

    pub fn seed(&self) -> u64 {
        u64::from_le_bytes((&self.data[0x8..0x10]).try_into().unwrap())
    }

    pub fn stars(&self) -> u8 {
        self.data[0x10] + 1
    }

    pub fn rand_roll(&self) -> u8 {
        self.data[0x11]
    }

    pub fn den_type(&self) -> DenType {
        self.data[0x12].try_into().unwrap()
    }

    pub fn flag_byte(&self) -> u8 {
        self.data[0x13]
    }

    pub fn is_active(&self) -> bool {
        self.den_type() != DenType::Empty
    }

    pub fn is_rare(&self) -> bool {
        let den_type = self.den_type();
        den_type == DenType::Rare || den_type == DenType::RareWish
    }

    pub fn is_wishing_piece(&self) -> bool {
        let den_type = self.den_type();
        den_type == DenType::CommonWish || den_type == DenType::RareWish
    }

    pub fn has_watts(&self) -> bool {
        (self.flag_byte() & 1) == 0
    }

    pub fn is_event(&self) -> bool {
        (self.flag_byte() & 2) == 2
    }

    pub fn get_spawn(&self, den_id: usize, is_sword: bool) -> DenSpawn {
        let game_version = if is_sword { 1 } else { 2 };
        let rand_roll = self.rand_roll();
        let rank = self.stars() - 1;

        if self.is_event() {
            let mut event_table_file =
                File::open("normal_encount").expect("Failed to open normal_encount file");
            let mut event_table_raw = Vec::new();
            event_table_file
                .read_to_end(&mut event_table_raw)
                .expect("Failed to read bytes from file");
            let event_table =
                root_as_nest_hole_distribution_encounter_8archive(&event_table_raw[32..])
                    .expect("Invalid flatbuffer");
            for table in event_table.tables().unwrap() {
                if table.game_version() == game_version {
                    let mut probability: u8 = 1;
                    for entry in table.entries().unwrap() {
                        probability += entry.probabilities().unwrap().get(rank as usize) as u8;
                        if probability > rand_roll {
                            return entry.into();
                        }
                    }
                }
            }
        } else {
            let den_hash = DEN_HASHES[den_id][{
                if self.is_rare() {
                    1
                } else {
                    0
                }
            }];
            for table in LOCAL_TABLE.tables().unwrap() {
                if table.table_id() == den_hash && table.game_version() == game_version {
                    let mut probability: u8 = 1;
                    for entry in table.entries().unwrap() {
                        probability += entry.probabilities().unwrap().get(rank as usize) as u8;
                        if probability > rand_roll {
                            return entry.into();
                        }
                    }
                }
            }
        }
        DenSpawn::default()
    }

    pub fn get_crystal_level(level: usize) -> Option<u8> {
        if (15..=20).contains(&level) {
            Some(0)
        } else if (25..=30).contains(&level) {
            Some(1)
        } else if (35..=40).contains(&level) {
            Some(2)
        } else if (45..=50).contains(&level) {
            Some(3)
        } else if (55..=60).contains(&level) {
            Some(4)
        } else {
            None
        }
    }
}

#[derive(Default, Clone)]
pub struct DenSpawn {
    pub entry_index: u32,
    pub species: u32,
    pub alt_form: u32,
    pub level_table_id: Option<u64>,
    pub ability: i8,
    pub is_gigantamax: bool,
    pub drop_table_id: u64,
    pub bonus_table_id: u64,
    pub probabilities: Vec<u32>,
    pub gender: i8,
    pub flawless_ivs: i8,
    pub level: Option<u32>,
    pub dynamax_level: Option<u16>,
    pub shiny_flag: Option<i8>,
    pub nature: Option<i8>,
    pub move0: Option<u32>,
    pub move1: Option<u32>,
    pub move2: Option<u32>,
    pub move3: Option<u32>,
    pub dynamax_boost: Option<f32>,
    pub shield: Option<u32>,
    pub additional_move1_rate: Option<u32>,
    pub additional_move1: Option<u32>,
    pub additional_move1_pp: Option<u32>,
    pub additional_move2_rate: Option<u32>,
    pub additional_move2: Option<u32>,
    pub additional_move2_pp: Option<u32>,
}

impl<'a> From<EncounterNest8<'a>> for DenSpawn {
    fn from(entry: EncounterNest8<'a>) -> Self {
        Self {
            entry_index: entry.entry_index(),
            species: entry.species(),
            alt_form: entry.alt_form(),
            level_table_id: Some(entry.level_table_id()),
            ability: entry.ability(),
            is_gigantamax: entry.is_gigantamax(),
            drop_table_id: entry.drop_table_id(),
            bonus_table_id: entry.bonus_table_id(),
            probabilities: entry.probabilities().unwrap().iter().collect::<Vec<u32>>(),
            gender: entry.gender(),
            flawless_ivs: entry.flawless_ivs(),
            level: None,
            dynamax_level: None,
            shiny_flag: None,
            nature: None,
            move0: None,
            move1: None,
            move2: None,
            move3: None,
            dynamax_boost: None,
            shield: None,
            additional_move1_rate: None,
            additional_move1: None,
            additional_move1_pp: None,
            additional_move2_rate: None,
            additional_move2: None,
            additional_move2_pp: None,
        }
    }
}

impl<'a> From<NestHoleDistributionEncounter8<'a>> for DenSpawn {
    fn from(entry: NestHoleDistributionEncounter8<'a>) -> Self {
        Self {
            entry_index: entry.entry_index(),
            species: entry.species(),
            alt_form: entry.alt_form(),
            level_table_id: None,
            ability: entry.ability(),
            is_gigantamax: entry.is_gigantamax(),
            drop_table_id: entry.drop_table_id(),
            bonus_table_id: entry.bonus_table_id(),
            probabilities: entry.probabilities().unwrap().iter().collect::<Vec<u32>>(),
            gender: entry.gender(),
            flawless_ivs: entry.flawless_ivs(),
            level: Some(entry.level()),
            dynamax_level: Some(entry.dynamax_level()),
            shiny_flag: Some(entry.shiny_flag()),
            nature: Some(entry.nature()),
            move0: Some(entry.move0()),
            move1: Some(entry.move1()),
            move2: Some(entry.move2()),
            move3: Some(entry.move3()),
            dynamax_boost: Some(entry.dynamax_boost()),
            shield: Some(entry.shield()),
            additional_move1_rate: Some(entry.additional_move1_rate()),
            additional_move1: Some(entry.additional_move1()),
            additional_move1_pp: Some(entry.additional_move1_pp()),
            additional_move2_rate: Some(entry.additional_move2_rate()),
            additional_move2: Some(entry.additional_move2()),
            additional_move2_pp: Some(entry.additional_move2_pp()),
        }
    }
}

impl<'a> From<NestHoleCrystalEncounter8<'a>> for DenSpawn {
    fn from(entry: NestHoleCrystalEncounter8<'a>) -> Self {
        Self {
            entry_index: entry.entry_index(),
            species: entry.species(),
            alt_form: entry.alt_form(),
            level_table_id: None,
            ability: entry.ability(),
            is_gigantamax: entry.is_gigantamax() != 0,
            drop_table_id: entry.drop_table_id(),
            bonus_table_id: entry.bonus_table_id(),
            probabilities: vec![],
            gender: 0,
            flawless_ivs: 0,
            level: Some(entry.level()),
            dynamax_level: Some(entry.dynamax_level() as u16),
            shiny_flag: None,
            nature: Some(entry.nature()),
            move0: Some(entry.move0()),
            move1: Some(entry.move1()),
            move2: Some(entry.move2()),
            move3: Some(entry.move3()),
            dynamax_boost: Some(entry.dynamax_boost()),
            shield: Some(entry.shield()),
            additional_move1_rate: Some(entry.additional_move1_rate()),
            additional_move1: Some(entry.additional_move1()),
            additional_move1_pp: Some(entry.additional_move1_pp()),
            additional_move2_rate: Some(entry.additional_move2_rate()),
            additional_move2: Some(entry.additional_move2()),
            additional_move2_pp: Some(entry.additional_move2_pp()),
        }
    }
}
