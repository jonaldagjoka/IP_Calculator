use crate::menus::{ipv4_menu, ipv6_menu};
use std::io::{self, Write};

pub fn main_menu() {
    loop {
        println!("  ┌────────────────────────────────────────────────────┐");
println!("  │               Main Menu                            │");
println!("  │        (choose your weapon)                        │");
println!("  └────────────────────────────────────────────────────┘\n"); 
println!("  (a) Llogaritje mbi IPv4");
println!("  (b) Llogaritje mbi IPv6");
println!("  (q) Quit");
println!("  ────────────────────────────────────────────────────");
print!("\n  Zgjidh nje opsion: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        match choice {
            "a" => ipv4_menu::ipv4_menu(),
            "b" => ipv6_menu::ipv6_menu(),
            "q" => {
                println!("\n  ────────────────────────────────────────────────────");
println!("     * . * coded with chaos by Jonalda Gjoka * . *");
println!("  ────────────────────────────────────────────────────\n");
                break;
            },
            _ => println!("Zgjedhje e pavlefshme!"),
        }
    }
}
