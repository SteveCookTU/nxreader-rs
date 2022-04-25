use crate::structs::swsh::swsh_reader::read_box;
use std::io;
use std::io::{stdout, Write};
use std::str::FromStr;
use sysbot_rs::SysBotClient;

pub fn check_box(client: SysBotClient) {
    loop {
        let mut input = String::new();
        print!("\nWhich box would you like to check? ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let id = usize::from_str(input.trim()).unwrap();
        let storage_box = read_box(id, &client);
        print!("\n{}", storage_box);
        stdout().flush().unwrap();
        input.clear();
        print!("Continue? (y/n) ");
        stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.to_lowercase().trim() != "y" {
            break;
        }
        input.clear();
    }
}
