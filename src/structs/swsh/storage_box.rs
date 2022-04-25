use crate::structs::pk8;
use crate::structs::pk8::PK8;
use std::fmt::{Display, Formatter};

pub struct StorageBox {
    id: usize,
    pkm: Vec<PK8>,
}

impl StorageBox {
    pub fn new(id: usize, data: Vec<u8>) -> Self {
        let mut pkm = Vec::with_capacity(30);
        for i in 0..30 {
            let pk8 =
                PK8::from((&data[(i * pk8::PARTY_SIZE)..((i + 1) * pk8::PARTY_SIZE)]).to_vec());
            pkm.push(pk8)
        }
        Self { id, pkm }
    }
}

impl Display for StorageBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut empty = true;
        for (i, pk8) in self.pkm.iter().enumerate() {
            if pk8.is_valid() && pk8.ec() != 0 {
                writeln!(f, "Box: {} Slot: {}", self.id, i + 1)?;
                writeln!(f, "{}", pk8)?;
                empty = false;
            }
        }
        if empty {
            writeln!(f, "Box is empty")
        } else {
            Ok(())
        }
    }
}
