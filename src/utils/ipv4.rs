use std::io::{self, Write};
use crate::utils::conversions::{ipv4_str_to_u32, u32_to_ipv4_string, mask_from_prefix_v4};

fn read_input(prompt: &str) -> String {
	print!("{}", prompt);
	io::stdout().flush().ok();
	let mut s = String::new();
	io::stdin().read_line(&mut s).ok();
	s.trim().to_string()
}

pub fn calculate_netid() {
	let input = read_input("Vendos IPv4 (format x.x.x.x/24): ");
	if let Some((ip, prefix)) = ipv4_str_to_u32(&input) {
		let mask = mask_from_prefix_v4(prefix);
		let network = ip & mask;
		println!("Network: {}/{}", u32_to_ipv4_string(network), prefix);
	} else { println!("Invalid IPv4 input") }
}

pub fn calculate_broadcast_address() {
	let input = read_input("Vendos IPv4 (format x.x.x.x/24): ");
	if let Some((ip, prefix)) = ipv4_str_to_u32(&input) {
		let mask = mask_from_prefix_v4(prefix);
		let network = ip & mask;
		let broadcast = network | (!mask);
		println!("Broadcast: {}", u32_to_ipv4_string(broadcast));
	} else { println!("Invalid IPv4 input") }
}

pub fn calculate_subnet_mask_from_hosts() {
    let input = read_input("Vendos numrin e hosteve: ");
    if let Ok(req_hosts) = input.parse::<u32>() {
        if req_hosts == 0 {
            println!("Vendos të paktën 1 host");
            return;
        }
        
        let total_needed = req_hosts + 2;
        
        let mut bits = 0u32;
        let mut available_hosts = 1u32;
        
        while available_hosts < total_needed {
            bits += 1;
            if bits > 30 {  
                println!("Numër shumë i madh i hosteve");
                return;
            }
            available_hosts = 2u32.pow(bits);
        }
        
        let prefix = 32 - bits;
        let mask = mask_from_prefix_v4(prefix as u8);
        
        println!("Numri i hosteve të kërkuar: {}", req_hosts);
        println!("Numri i hosteve të disponueshëm: {}", available_hosts - 2);
        println!("Prefix: /{}", prefix);
        println!("Subnet Mask: {}", u32_to_ipv4_string(mask));
    } else {
        println!("Numër i pavlefshëm")
    }
}

pub fn calculate_subnet_mask_from_subnets() {
    let parent = read_input("Vendos rrjetin kryesor (format x.x.x.x/24): ");
    let subnets = read_input("Vendos numrin e subnetave të kërkuara: ");

    if let Some((_ip, parent_prefix)) = ipv4_str_to_u32(&parent) {
        if let Ok(subs) = subnets.parse::<u32>() {
            let mut bits = 0u8;
            while (1u128 << bits) < subs as u128 {
                bits += 1;
            }

            let new_prefix = parent_prefix + bits;
            if new_prefix > 32 {
                println!("Numri i subnetave është shumë i madh për këtë rrjet");
                return;
            }

            let mask = mask_from_prefix_v4(new_prefix);
            println!(
                "New prefix: /{}  Mask: {}",
                new_prefix,
                u32_to_ipv4_string(mask)
            );
        } else {
            println!("Numër i pavlefshëm i subnetave");
        }
    } else {
        println!("Rjeti kryesor i pavlefshëm");
    }
}


pub fn cidr_to_subnet_mask() {
	let cidr = read_input("Vendos prefiksin CIDR (0-32): ");
	if let Ok(prefix) = cidr.parse::<u8>() {
		if prefix > 32 { println!("CIDR duhet te jete 0-32"); return; }
		let mask = mask_from_prefix_v4(prefix);
		println!("Subnet Mask: {}", u32_to_ipv4_string(mask));
	} else { println!("CIDR i pavlefshëm") }
}

pub fn subnet_mask_to_cidr() {
	let input = read_input("Vendos subnet mask (x.x.x.x): ");
	if let Some((mask, _)) = ipv4_str_to_u32(&input) {
		let mut prefix :u8= 0;
		let mut zero_seen=false;

		for i in (0..32).rev(){
			let bit =(mask>>i)&1;
			if bit==1{
				if zero_seen{
					println!("Subnet mask INVALID (1 pas 0)");
                    return;
				}
				prefix+=1;
			}else{
				zero_seen=true;
			}
		}
		println!("CIDR: /{}", prefix);
	} else { println!("Subnet mask e pavlefshme") }
}
fn parse_ipv4_only(s: &str) -> Option<u32> {
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 4 {
        return None;
    }

    let mut ip = 0u32;
    for part in parts {
        let octet = part.parse::<u8>().ok()?;
        ip = (ip << 8) | octet as u32;
    }
    Some(ip)
}

pub fn wildcard_mask_calculation() {
    let input = read_input("Vendos CIDR (x.x.x.x/24) ose Subnet Mask (x.x.x.x): ");

    if input.contains('/') {
        if let Some((_ip, prefix)) = ipv4_str_to_u32(&input) {
            let mask = mask_from_prefix_v4(prefix);
            let wildcard = !mask;
            println!("Wildcard Mask: {}", u32_to_ipv4_string(wildcard));
        } else {
            println!("CIDR i pavlefshëm");
        }
        return;
    }

    if let Some(mask) = parse_ipv4_only(&input) {
        // validim subnet mask
        let mut zero_seen = false;
        for i in (0..32).rev() {
            let bit = (mask >> i) & 1;
            if bit == 1 {
                if zero_seen {
                    println!("Subnet mask INVALID (1 pas 0)");
                    return;
                }
            } else {
                zero_seen = true;
            }
        }

        let wildcard = !mask;
        println!("Wildcard Mask: {}", u32_to_ipv4_string(wildcard));
    } else {
        println!("Input i pavlefshëm");
    }
}



pub fn vlsm_calculation() {
    println!("VLSM: Variable Length Subnet Mask");

    let parent = read_input("Vendos rrjetin kryesor (x.x.x.x/24): ");
    let num_subnets = read_input("Vendos numrin e subneteve të kërkuara: ");

    if let Some((ip, parent_prefix)) = ipv4_str_to_u32(&parent) {
        if let Ok(subs) = num_subnets.parse::<u32>() {

            let mut bits = 0u8;
            while (1u128 << bits) < subs as u128 {
                bits += 1;
            }

            let new_prefix = parent_prefix + bits;
            if new_prefix > 32 {
                println!("Nuk mund të ndahet më tej");
                return;
            }

            let parent_mask = mask_from_prefix_v4(parent_prefix);
            let network = ip & parent_mask;

            let subnet_size = 1u32 << (32 - new_prefix);

            println!("New Prefix: /{}", new_prefix);

            for i in 0..subs.min(16) {
                let subnet_ip = network + subnet_size * i;
                println!(
                    "Subnet {}: {}/{}",
                    i + 1,
                    u32_to_ipv4_string(subnet_ip),
                    new_prefix
                );
            }

            if subs > 16 {
                println!("... and {} more", subs - 16);
            }

        } else {
            println!("Numër i pavlefshëm i subnetave");
        }
    } else {
        println!("Rjeti kryesor i pavlefshëm");
    }
}

pub fn supernetting() {
	println!("Supernetting");
    println!("Kushtet:");
    println!("- Rrjetet të jenë të njëpasnjëshme");
    println!("- Numri i rrjeteve fuqi e dyshit");
    println!("- Të kenë të njëjtin prefiks");
	println!("______________________________________");
    println!("Supernetting me x rrjete");

    let n: usize = read_input("Vendos numrin e rrjeteve: ")
        .parse()
        .unwrap_or(0);

    if n < 2 || (n & (n - 1)) != 0 {
        println!(" Numri i rrjeteve duhet të jetë fuqi e dyshit (2,4,8,16,32,64,128,...");
        return;
    }

    let mut networks: Vec<(u32, u8)> = Vec::new();

    for i in 0..n {
        let input = read_input(&format!("Vendos rrjetin {} (x.x.x.x/24): ", i + 1));
        if let Some(net) = ipv4_str_to_u32(&input) {
            networks.push(net);
        } else {
            println!(" Input i pavlefshëm");
            return;
        }
    }

    let prefix = networks[0].1;

    if networks.iter().any(|(_, p)| *p != prefix) {
        println!("Të gjitha rrjetet duhet të kenë të njëjtin prefiks");
        return;
    }

    let mask = mask_from_prefix_v4(prefix);
    let block_size = 1u32 << (32 - prefix);

    let mut ips: Vec<u32> = networks.iter().map(|(ip, _)| *ip).collect();
    ips.sort();

    for ip in &ips {
        if ip & mask != *ip {
            println!(" IP nuk është Network ID");
            return;
        }
    }

    for i in 0..ips.len() - 1 {
        if ips[i + 1] != ips[i] + block_size {
            println!(" Rrjetet nuk janë të njëpasnjëshme");
            return;
        }
    }
    let bits=n.trailing_zeros() as u8;
    let new_prefix = prefix - bits;
    let supernet_mask = mask_from_prefix_v4(new_prefix);
    let supernet_ip = ips[0] & supernet_mask;

    println!(
        " Supernet: {}/{}",
        u32_to_ipv4_string(supernet_ip),
        new_prefix
    );
}


pub fn dhcp_range_calculation() {
	let network = read_input("Vendos rrjetin (x.x.x.x/24): ");
	if let Some((ip, prefix)) = ipv4_str_to_u32(&network) {
		let mask = mask_from_prefix_v4(prefix);
		let net = ip & mask;
		let broadcast = net | (!mask);
		let first_host = net + 1;
		let last_host = broadcast - 1;
		if prefix == 31 {
			println!("Rrjeti nuk ka hoste të përdorshëm");
			println!("Host: 2 IP ({} dhe {})", u32_to_ipv4_string(net), u32_to_ipv4_string(broadcast));
			println!("Point-to-point /31");
			return;
		}
	    if prefix == 32 {
			println!("Rrjeti ka vetëm një adresë të disponueshme");
			println!("Host: 1 IP ({})", u32_to_ipv4_string(net));
			println!("single address /32");
			return;
		}
		println!("DHCP Pool:");
		println!("  Hosti i pare: {}", u32_to_ipv4_string(first_host));
		println!("  Hosti i fundit: {}", u32_to_ipv4_string(last_host));
		let total = if last_host >= first_host { last_host - first_host + 1 } else { 0 };
		println!("  Hoste Totale: {}", total);
	} else { println!("Rjeti i pavlefshëm"); }
}

pub fn ip_belong_check() {
	let network = read_input("Vendos rrjetin (x.x.x.x/24): ");
	let check_ip = read_input("Vendos IP-në për kontroll (x.x.x.x): ");
	if let (Some((net_ip, prefix)), Some((check, _))) = (ipv4_str_to_u32(&network), ipv4_str_to_u32(&check_ip)) {
		let mask = mask_from_prefix_v4(prefix);
		let network_addr = net_ip & mask;
		let check_addr = check & mask;
		if network_addr == check_addr {
			println!("✓ IP {} i perket rrjetit {}/{}", u32_to_ipv4_string(check), u32_to_ipv4_string(network_addr), prefix);
		} else {
			println!("✗ IP {} NUK i perket rrjetit {}/{}", u32_to_ipv4_string(check), u32_to_ipv4_string(network_addr), prefix);
		}
	} else { println!("Input i pavlefshëm"); }
}
