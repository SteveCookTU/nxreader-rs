use crate::structs::pk8::{PARTY_SIZE, PK8, STORED_SIZE};
use crate::structs::swsh::{Den, KCoordinates, MyStatus8, StorageBox, DEN_COUNT, DEN_SIZE};
use crate::structs::SystemLanguage;
use std::fs::OpenOptions;
use std::io::Write;
use sysbot_rs::types::PeekArgs;
use sysbot_rs::SysBotClient;

pub fn read_box(mut r#box: usize, client: &SysBotClient) -> StorageBox {
    if r#box > 32 {
        r#box = 32;
    }

    let address = 0x45075880 + ((r#box - 1) * 30 * PARTY_SIZE);
    StorageBox::new(
        r#box,
        client
            .peek(PeekArgs {
                addr: address as u64,
                size: PARTY_SIZE * 30,
            })
            .unwrap(),
    )
}

pub fn read_dens(client: &SysBotClient) -> Vec<Den> {
    let address = 0x450C8A70;
    let den_data = client
        .peek(PeekArgs {
            addr: address as u64,
            size: (DEN_SIZE * 99) + (DEN_SIZE * (90 + 11)) + (DEN_SIZE * (86 + 32)),
        })
        .unwrap();
    let mut dens = Vec::with_capacity(DEN_COUNT);
    for i in 0..DEN_COUNT {
        if i > 189 {
            dens.push(Den::new(
                den_data[((i + 32) * DEN_SIZE)..((i + 33) * DEN_SIZE)].to_vec(),
            ));
        } else if i > 99 {
            dens.push(Den::new(
                den_data[((i + 11) * DEN_SIZE)..((i + 12) * DEN_SIZE)].to_vec(),
            ));
        } else {
            dens.push(Den::new(
                den_data[(i * DEN_SIZE)..((i + 1) * DEN_SIZE)].to_vec(),
            ))
        }
    }
    dens
}

pub fn read_my_status_8(client: &SysBotClient) -> MyStatus8 {
    let mut data = client
        .peek(PeekArgs {
            addr: 0x45068F18,
            size: 0x110,
        })
        .unwrap();
    data.pop().unwrap();
    data.append(
        &mut client
            .peek(PeekArgs {
                addr: 0x45072DF4,
                size: 0x3,
            })
            .unwrap(),
    );
    data.into()
}

pub fn read_horse(client: &SysBotClient) -> PK8 {
    client
        .peek(PeekArgs {
            addr: 0x450CAE28,
            size: STORED_SIZE,
        })
        .unwrap()
        .into()
}

pub fn read_legend(client: &SysBotClient) -> PK8 {
    client
        .peek(PeekArgs {
            addr: 0x886BC348,
            size: STORED_SIZE,
        })
        .unwrap()
        .into()
}

pub fn read_k_coordinates_block(client: &SysBotClient) -> KCoordinates {
    client
        .peek(PeekArgs {
            addr: 0x4505B3C0,
            size: 0x6010,
        })
        .unwrap()
        .into()
}

pub fn read_party(client: &SysBotClient) -> Vec<PK8> {
    let data = client
        .peek(PeekArgs {
            addr: 0x450C68B0,
            size: 6 * PARTY_SIZE,
        })
        .unwrap();

    data.chunks_exact(PARTY_SIZE)
        .map(|chunk| PK8::from(chunk.to_vec()))
        .collect::<Vec<PK8>>()
}

pub fn read_wild(client: &SysBotClient) -> PK8 {
    client
        .peek(PeekArgs {
            addr: 0x8FEA3648,
            size: STORED_SIZE,
        })
        .unwrap()
        .into()
}

pub fn get_event_offset(client: &SysBotClient) -> i16 {
    let system_lang: SystemLanguage = client.get_system_language().unwrap().into();
    match system_lang {
        SystemLanguage::Zhcn | SystemLanguage::Zhhans => -0xE00,
        SystemLanguage::Zhtw | SystemLanguage::Zhhant => -0xE60,
        SystemLanguage::Ko => -0xA00,
        SystemLanguage::It => -0x80,
        SystemLanguage::Ja => 0x160,
        SystemLanguage::Fr | SystemLanguage::Frca => 0x1F0,
        SystemLanguage::Es | SystemLanguage::Es419 => 0x1C0,
        SystemLanguage::De => 0x2D0,
        _ => 0,
    }
}

pub fn dump_event_block_bonus_rewards(client: &SysBotClient) {
    let mut data = client
        .peek(PeekArgs {
            addr: 0x2FA03F78u64.wrapping_add(get_event_offset(client) as u64),
            size: 0x116C4,
        })
        .unwrap();
    data.pop();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("bonus_rewards")
        .unwrap();
    file.write_all(&data).unwrap();
}

pub fn dump_event_block_crystal_encounter(client: &SysBotClient) {
    let mut data = client
        .peek(PeekArgs {
            addr: 0x2F9ED788u64.wrapping_add(get_event_offset(client) as u64),
            size: 0x1241C,
        })
        .unwrap();
    data.pop();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("dai_encount")
        .unwrap();
    file.write_all(&data).unwrap();
}

pub fn dump_event_block_drop_rewards(client: &SysBotClient) {
    let mut data = client
        .peek(PeekArgs {
            addr: 0x2F9FFC58u64.wrapping_add(get_event_offset(client) as u64),
            size: 0x426C,
        })
        .unwrap();
    data.pop();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("drop_rewards")
        .unwrap();
    file.write_all(&data).unwrap();
}

pub fn dump_event_block_raid_encounter(client: &SysBotClient) {
    let mut data = client
        .peek(PeekArgs {
            addr: 0x2F9EB300u64.wrapping_add(get_event_offset(client) as u64),
            size: 0x23D4,
        })
        .unwrap();
    data.pop();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("normal_encount")
        .unwrap();
    file.write_all(&data).unwrap();
}

pub fn dump_event_block_raid_encounter_ioa(client: &SysBotClient) {
    let mut data = client
        .peek(PeekArgs {
            addr: 0x2FA156F0u64.wrapping_add(get_event_offset(client) as u64),
            size: 0x23D4,
        })
        .unwrap();
    data.pop();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("normal_encount_rigel1")
        .unwrap();
    file.write_all(&data).unwrap();
}

pub fn dump_event_block_raid_encounter_ct(client: &SysBotClient) {
    let mut data = client
        .peek(PeekArgs {
            addr: 0x2FA17B78u64.wrapping_add(get_event_offset(client) as u64),
            size: 0x23D4,
        })
        .unwrap();
    data.pop();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("normal_encount_rigel2")
        .unwrap();
    file.write_all(&data).unwrap();
}
