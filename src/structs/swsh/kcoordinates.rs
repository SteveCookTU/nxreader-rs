use crate::structs::pk8_overworld::PK8;
use crate::structs::swsh::swsh_reader::read_k_coordinates_block;
use sysbot_rs::types::PeekArgs;
use sysbot_rs::SysBotClient;

pub struct KCoordinates {
    data: Vec<u8>,
}

impl KCoordinates {
    pub fn refresh(self, client: &SysBotClient) -> Self {
        read_k_coordinates_block(client)
    }

    pub fn read_ow_pokemon_from_block(
        &self,
        client: &SysBotClient,
        tid: u16,
        sid: u16,
    ) -> Vec<PK8> {
        let mut pk8s = Vec::new();
        let mut i = 8;
        let mut j = 0;
        let mut last_index = 8;
        while i < self.data.len() {
            if j == 12 && self.data[i - 64] != 0 && self.data[i - 68] != 255 {
                let bytes = &self.data[(i - 68)..(i - 68 + 56)];
                j = 0;
                i = last_index + 8;
                if let Some(pkm) = self.read_ow_pokemon(0, 0, bytes, client, tid, sid) {
                    pk8s.push(pkm)
                }
            }

            if self.data[i] == 255 {
                if i % 8 == 0 {
                    last_index = i;
                }

                i += 1;
                j += 1;
            } else {
                j = 0;
                if i == last_index {
                    i += 8;
                    last_index = i;
                } else {
                    i = last_index + 8;
                    last_index = i;
                }
            }
        }
        pk8s
    }

    pub fn read_ow_pokemon(
        &self,
        target: u16,
        start_offset: usize,
        mon_data: &[u8],
        client: &SysBotClient,
        tid: u16,
        sid: u16,
    ) -> Option<PK8> {
        let mut data = Vec::new();
        let mut offset = start_offset;
        let mut i = 0;

        if target != 0 {
            data = client
                .peek(PeekArgs {
                    addr: offset as u64,
                    size: 56,
                })
                .unwrap();
            let mut species = u16::from_le_bytes((&data[0..2]).try_into().unwrap());
            offset += 192;
            i += 1;
            while target != 0 && species != 0 && target != species && i < 20 {
                data = client
                    .peek(PeekArgs {
                        addr: offset as u64,
                        size: 56,
                    })
                    .unwrap();
                species = u16::from_le_bytes((&data[0..2]).try_into().unwrap());
                i += 1;
            }
        } else if !mon_data.is_empty() {
            data = mon_data.to_vec();
        }

        if !data.is_empty() && data[20] == 1 {
            return Some(PK8::new(tid, sid, data));
        }

        None
    }
}

impl From<Vec<u8>> for KCoordinates {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}
