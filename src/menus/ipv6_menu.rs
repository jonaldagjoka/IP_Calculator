use crate::utils::ipv6;
use std::io::{self, Write};

pub fn ipv6_menu(){
    loop {
        println!("  ┌────────────────────────────────────────────────────┐");
        println!("  │               IPv6 Calculator                      │");
        println!("  │        (welcome to the future of networking)       │");
        println!("  └────────────────────────────────────────────────────┘\n");
        println!("  (a) Llogarit Network Prefix (NetID)");
        println!("  (b) Llogarit IP Range");
        println!("  (c) IPv6 Expansion (shkruaj formën e plotë)");
        println!("  (d) IPv6 Compression (shkurto adresën)");
        println!("  (e) Hex to Decimal Conversion");
        println!("  (f) Decimal to Hex Conversion");
        println!("  (g) IPv6 Address Type Identifier");
        println!("  (h) Generate EUI-64 Address");
        println!("  (i) Subnetting");
        println!("  (j) Supernetting");
        println!("  (k) DHCP Range Calculation");
        println!("  (q) Kthehu në Menu Kryesore");
        println!("  ────────────────────────────────────────────────────");

        print!("\n  Zgjidh një opsion: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        
        match choice {
            "a" => ipv6::calculate_netid(),
            "b" => ipv6::calculate_ip_range(),
            "c" => ipv6::expand_ipv6(),
            "d" => ipv6::compress_ipv6(),
            "e" => ipv6::hex_to_decimal(),
            "f" => ipv6::decimal_to_hex(),
            "g" => ipv6::address_type_identifier(),
            "h" => ipv6::generate_eui64(),
            "i" => ipv6::subnetting(),
            "j" => ipv6::supernetting(),
            "k" => ipv6::dhcp_range_calculation(),
            "q" => break,
            _ => println!("Zgjedhje e pavlefshme!"),
        }
    }
}