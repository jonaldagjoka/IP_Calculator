use std::io::{self, Write};
use crate::utils::ipv4;
use crate::utils::ip_classes::check_ip_classes;
use crate::utils::conversions as conv;

pub fn ipv4_menu(){
    loop{
        println!("  ┌────────────────────────────────────────────────────┐");
println!("  │               IPv4 Calculator                      │");
println!("  │        (the classic that still works)              │");
println!("  └────────────────────────────────────────────────────┘\n");
println!("  (*) Kontrollo Klasat dhe IP speciale");
println!("  (a) Network ID");
println!("  (b) Broadcast Address");
println!("  (c) DHCP Range");
println!("  (d) Kontrollo nëse IP i përket një rrjeti");
println!("  (e) Llogarit Subnet Mask nga Numri i Hosteve");
println!("  (f) Llogarit Subnet Mask nga Numri i Subneteve");
println!("  (g) Decimal -> Binary");
println!("  (h) Binary -> Decimal");
println!("  (i) CIDR -> Subnet Mask");
println!("  (j) Subnet Mask -> CIDR");
println!("  (k) Wildcard Mask");
println!("  (l) VLSM");
println!("  (m) Supernetting");
println!("  (q) Kthehu në Menu Kryesore");
println!("  ────────────────────────────────────────────────────");
print!("\n  Zgjidh një opsion: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        match choice {
            "*" => check_ip_classes(),
            "a" => ipv4::calculate_netid(),
            "b" => ipv4::calculate_broadcast_address(),
            "c" => ipv4::dhcp_range_calculation(),
            "d" => ipv4::ip_belong_check(),
            "e" => ipv4::calculate_subnet_mask_from_hosts(),
            "f" => ipv4::calculate_subnet_mask_from_subnets(),
            "g" => {
                print!("Vendos IPv4 (x.x.x.x or x.x.x.x/xx): ");
                io::stdout().flush().ok();
                let mut s = String::new();
                io::stdin().read_line(&mut s).ok();
                let s = s.trim();
                if let Some(bin) = conv::ipv4_to_binary_string(s) {
                    println!("Binary: {}", bin);
                } else { println!("Invalid IPv4 input") }
            },
            "h" => {
                print!("Vendos IPv4 në binar (aaaaaaaa.bbbbbbbb.cccccccc.dddddddd): ");
                io::stdout().flush().ok();
                let mut s = String::new();
                io::stdin().read_line(&mut s).ok();
                let s = s.trim();
                if let Some(dec) = conv::ipv4_binary_to_decimal(s) {
                    println!("Decimal IPv4: {}", dec);
                } else { println!("Invalid binary input") }
            },
            "i" => ipv4::cidr_to_subnet_mask(),
            "j" => ipv4::subnet_mask_to_cidr(),
            "k" => ipv4::wildcard_mask_calculation(),
            "l" => ipv4::vlsm_calculation(),
            "m" => ipv4::supernetting(),
            "q" => break,
            _ => println!("Zgjedhje e pavlefshme!"),
        }
    }
}