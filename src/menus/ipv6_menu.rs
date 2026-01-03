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
        println!("  (c) Llogarit Numër Hostesh (2^(128-prefix))");
        println!("  (d) IPv6 Expansion (shkruaj formën e plotë)");
        println!("  (e) IPv6 Compression (shkurto adresën)");
        println!("  (f) Hex to Decimal Conversion");
        println!("  (g) Decimal to Hex Conversion");
        println!("  (h) IPv6 Address Type Identifier");
        println!("  (i) Generate Link-Local Address");
        println!("  (j) Generate EUI-64 Address");
        println!("  (k) Subnetting");
        println!("  (l) Supernetting");
        println!("  (m) DHCP Range Calculation");
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
            "c" => ipv6::calculate_number_of_hosts(),
            "d" => ipv6::expand_ipv6(),
            "e" => ipv6::compress_ipv6(),
            "f" => ipv6::hex_to_decimal(),
            "g" => ipv6::decimal_to_hex(),
            "h" => ipv6::address_type_identifier(),
            "i" => ipv6::generate_link_local(),
            "j" => ipv6::generate_eui64(),
            "k" => ipv6::subnetting(),
            "l" => ipv6::supernetting(),
            "m" => ipv6::dhcp_range_calculation(),
            "q" => break,
            _ => println!("Zgjedhje e pavlefshme!"),
        }
    }
}