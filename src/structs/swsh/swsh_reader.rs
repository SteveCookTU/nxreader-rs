use crate::structs::pk8;
use crate::structs::pk8::PK8;
use crate::structs::swsh::{Den, KCoordinates, MyStatus8, StorageBox, DEN_COUNT, DEN_SIZE};
use sysbot_rs::types::PeekArgs;
use sysbot_rs::SysBotClient;

const PK8_STORED_SIZE: usize = 0x148;
const PK8_PARTY_SIZE: usize = 0x158;

pub fn read_box(mut r#box: usize, client: &SysBotClient) -> StorageBox {
    if r#box > 32 {
        r#box = 32;
    }

    let address = 0x45075880 + ((r#box - 1) * 30 * PK8_PARTY_SIZE);
    StorageBox::new(
        r#box,
        client
            .peek(PeekArgs {
                addr: address as u64,
                size: PK8_PARTY_SIZE * 30,
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
    data.extend(client.peek(PeekArgs { addr: 0, size: 0 }).unwrap());
    data.into()
}

pub fn read_horse(client: &SysBotClient) -> PK8 {
    client
        .peek(PeekArgs {
            addr: 0x450CAE28,
            size: pk8::STORED_SIZE,
        })
        .unwrap()
        .into()
}

pub fn read_legend(client: &SysBotClient) -> PK8 {
    client
        .peek(PeekArgs {
            addr: 0x886BC348,
            size: pk8::STORED_SIZE,
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
