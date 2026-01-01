use crate::menus::{ipv4_menu, ipv6_menu};
use crate::utils::ip_classes::check_ip_classes;
use std::io::{self, Write};

pub fn main_menu() {
    loop {
        println!("\n--- Main Menu ---");
        println!("(a) Kontrollo Klasat dhe IP speciale");
        println!("(b) Llogaritje mbi IPv4");
        println!("(c) Llogaritje mbi IPv6");
        println!("(d) Quit");

        print!("Zgjidh një opsion: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "a" => check_ip_classes(),
            "b" => ipv4_menu::ipv4_menu(),
            "c" => ipv6_menu::ipv6_menu(),
            "d" => {
                println!("Faleminderit që përdorët tool-in!");
                break;
            },
            _ => println!("Zgjedhje e pavlefshme!"),
        }
    }
}
