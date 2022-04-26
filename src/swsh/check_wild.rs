use crate::structs::swsh::swsh_reader::read_wild;
use std::io;
use std::io::{stdout, Write};
use sysbot_rs::SysBotClient;

pub fn check_wild(client: SysBotClient) {
    loop {
        let pk8 = read_wild(&client);
        if pk8.is_valid() && pk8.ec() != 0 {
            println!("{}", pk8);
        } else {
            println!("No battle started\n");
        }
        print!("Check again? (y/n) ");
        stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        if input.to_lowercase().trim() == "n" {
            break;
        }
    }
}
