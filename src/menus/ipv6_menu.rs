use crate::menus::ipv6_menu;
use crate::utils::ip_classes::check_ip_classes;
use crate::utils::ipv6;
use crate::utils::conversions as conv;
use std::io::{self, Write};

pub fn ipv6_menu(){
    loop {
        println!("\n----IPv6 Menu ----");
        println!("(a) Llogarit NetID");
        println!("(b) Llogarit Broadcast Address");
        println!("(c) Llogarit Range të IP");
        println!("(d) Llogarit Numër të Hosteve");
        println!("(e) Llogarit Subnet Mask nga Numri i Hosteve");
        println!("(f) Llogarit Subnet Mask nga Numri i Subneteve");
        println!("(g) Hex to Decimal Conversion");
        println!("(h) Decimal to Hex Conversion");
        println!("(i) CIDR to Subnet Mask Conversion");
        println!("(j) Subnet Mask to CIDR Conversion");
        println!("(k) Wildcard Mask Calculation");
        println!("(l) VLSM Calculation");
        println!("(m) Subnetting");
        println!("(n) Supernetting");
        println!("(o) DHCP Range Calculation");
        println!("(q) Kthehu në Menu Kryesore");

        print!("Zgjidh një opsion: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        match choice {
            "a" => ipv6::calculate_netid(),
            "b" => ipv6::calculate_broadcast_address(),
            "c" => ipv6::calculate_ip_range(),
            "d" => ipv6::calculate_number_of_hosts(),
            "e" => ipv6::calculate_subnet_mask_from_hosts(),
            "f" => ipv6::calculate_subnet_mask_from_subnets(),
            "g" => {
                print!("Enter IPv6 (hex) (e.g. 2001:db8::1/64): ");
                io::stdout().flush().ok();
                let mut s = String::new();
                io::stdin().read_line(&mut s).ok();
                let s = s.trim();
                if let Some(dec) = conv::ipv6_to_decimal_string(s) {
                    println!("Decimal: {}", dec);
                } else { println!("Invalid IPv6 input") }
            },
            "h" => {
                print!("Enter IPv6 decimal (u128): ");
                io::stdout().flush().ok();
                let mut s = String::new();
                io::stdin().read_line(&mut s).ok();
                let s = s.trim();
                if let Some(hex) = conv::ipv6_decimal_to_string(s) {
                    println!("IPv6: {}", hex);
                } else { println!("Invalid decimal input") }
            },
            "i" => ipv6::cidr_to_prefix(),
            "j" => ipv6::prefix_to_cidr(),
            "k" => ipv6::wildcard_mask_calculation(),
            "l" => ipv6::vlsm_calculation(),
            "m" => ipv6::subnetting(),
            "n" => ipv6::supernetting(),
            "o" => ipv6::dhcp_range_calculation(),
            "q" => break,
            _ => println!("Zgjedhje e pavlefshme!"),
        }
    }
}