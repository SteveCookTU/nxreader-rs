use crate::structs::swsh::{Den, MyStatus8, StorageBox, DEN_COUNT, DEN_SIZE};
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

pub fn read_den(mut den_id: usize, client: &SysBotClient) -> Den {
    if den_id > DEN_COUNT + 31 {
        den_id = DEN_COUNT + 31;
    }
    let address = 0x450C8A70 + den_id * DEN_SIZE;
    Den::new(
        client
            .peek(PeekArgs {
                addr: address as u64,
                size: DEN_SIZE,
            })
            .unwrap(),
    )
}

pub fn read_my_status_8(client: &SysBotClient) -> MyStatus8 {
    let mut data = client
        .peek(PeekArgs {
            addr: 0x45068F18,
            size: 0x110,
        })
        .unwrap();
    data.extend(client.peek(PeekArgs { addr: 0, size: 0 }).unwrap());
    MyStatus8::new(data)
}
