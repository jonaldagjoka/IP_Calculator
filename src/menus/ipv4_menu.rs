use std::io::{self, Write};
use crate::utils::ipv4;
use crate::utils::conversions as conv;

pub fn ipv4_menu(){
    loop{
        println!("================================");
        println!("|        IPv4 Calculator       |");
        println!("================================");
        println!("(a)Llogairt NetID");
        println!("(b)Llogarit Broadcast Address");
        println!("(c)Llogarit Range të IP");
        println!("(d)Llogarit Numër të Hosteve");
        println!("(e)Llogarit Subnet Mask nga Numri i Hosteve");
        println!("(f)Llogarit Subnet Mask nga Numri i Subneteve");
        println!("(g)Binay to Decimal Conversion");
        println!("(h)Decimal to Binary Conversion");
        println!("(i)CIDR to Subnet Mask Conversion");
        println!("(j)Subnet Mask to CIDR Conversion");
        println!("(k)Wildcard Mask Calculation");
        println!("(l)VLSM Calculation");
        println!("(m)Subnetting");
        println!("(n)Supernetting");
        println!("(o)DHCP Range Calculation");
        println!("(p)IP belong check");
        println!("(q)Kthehu në Menu Kryesore");

        print!("Zgjidh një opsion: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        match choice {
            "a" => ipv4::calculate_netid(),
            "b" => ipv4::calculate_broadcast_address(),
            "c" => ipv4::calculate_ip_range(),
            "d" => ipv4::calculate_number_of_hosts(),
            "e" => ipv4::calculate_subnet_mask_from_hosts(),
            "f" => ipv4::calculate_subnet_mask_from_subnets(),
            "g" => {
                print!("Enter IPv4 (x.x.x.x or x.x.x.x/xx): ");
                io::stdout().flush().ok();
                let mut s = String::new();
                io::stdin().read_line(&mut s).ok();
                let s = s.trim();
                if let Some(bin) = conv::ipv4_to_binary_string(s) {
                    println!("Binary: {}", bin);
                } else { println!("Invalid IPv4 input") }
            },
            "h" => {
                print!("Enter IPv4 in binary (aaaaaaaa.bbbbbbbb.cccccccc.dddddddd): ");
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
            "m" => ipv4::subnetting(),
            "n" => ipv4::supernetting(),
            "o" => ipv4::dhcp_range_calculation(),
            "p" => ipv4::ip_belong_check(),
            "q" => break,
            _ => println!("Zgjedhje e pavlefshme!"),
        }
    }
}