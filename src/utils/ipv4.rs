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
	if let Ok(req_hosts) = input.parse::<u128>() {
		if req_hosts == 0 {
			println!("Vendos të paktën 1 host");
			return;
		}
		let mut bits = 1u32;
		loop {
			if bits >= 32 {
				println!("Numër shumë i madh i hosteve");
				return;
			}
			let available_hosts = (1u128 << bits) - 2;
			if available_hosts >= req_hosts {
				break;
			}
			bits += 1;
		}
		let prefix = 32u32 - bits;
		let mask = mask_from_prefix_v4(prefix as u8);
		println!("Prefix: /{}  Mask: {}", prefix, u32_to_ipv4_string(mask));
	} else { println!("Numër i pavlefshëm") }
}

pub fn calculate_subnet_mask_from_subnets() {
	let parent = read_input("Vendos rrjetin kryesor (format x.x.x.x/24): ");
	let subnets = read_input("Vendos numrin e subnetave të kërkuara: ");
	if let Some((_ip, parent_prefix)) = ipv4_str_to_u32(&parent) {
		if let Ok(subs) = subnets.parse::<u32>() {
			let mut bits = 0u8;
			while (1u128 << bits) < subs as u128 { bits += 1; }
			let new_prefix = parent_prefix + bits;
			if new_prefix > 32 { println!("Nuk mund të ndahet më tej"); }
			else {
				let mask = mask_from_prefix_v4(new_prefix);
				println!("New prefix: /{}  Mask: {}", new_prefix, u32_to_ipv4_string(mask));
			}
		} else { println!("Numër i pavlefshëm i subnetave"); }
	} else { println!("Rjeti kryesor i pavlefshëm") }
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
		let mut prefix = 0u8;
		let mut test = 0x80000000u32;
		while (mask & test) != 0 && prefix < 32 {
			prefix += 1;
			test >>= 1;
		}
		println!("CIDR: /{}", prefix);
	} else { println!("Subnet mask e pavlefshme") }
}

pub fn wildcard_mask_calculation() {
	let input = read_input("Vendos CIDR (x.x.x.x/24) ose Subnet Mask (x.x.x.x): ");
	
	// Check if input contains '/' for CIDR notation
	if input.contains('/') {
		// Parse as CIDR
		if let Some((_ip, prefix)) = ipv4_str_to_u32(&input) {
			let mask = mask_from_prefix_v4(prefix);
			let wildcard = !mask;
			println!("Wildcard Mask: {}", u32_to_ipv4_string(wildcard));
		} else {
			println!("Input i pavlefshëm");
		}
	} else {
		// Parse as direct subnet mask
		if let Some((mask, _)) = ipv4_str_to_u32(&input) {
			let wildcard = !mask;
			println!("Wildcard Mask: {}", u32_to_ipv4_string(wildcard));
		} else {
			println!("Input i pavlefshëm");
		}
	}
}

pub fn vlsm_calculation() {
	println!("VLSM: Variable Length Subnet Mask");
	let parent = read_input("Vendos rrjetin kryesor (x.x.x.x/24): ");
	let num_subnets = read_input("Vendos numrin e subneteve të kërkuara: ");
	if let Some((ip, parent_prefix)) = ipv4_str_to_u32(&parent) {
		if let Ok(subs) = num_subnets.parse::<u32>() {
			let mut bits_needed = 0u8;
			while (1u128 << bits_needed) < subs as u128 { bits_needed += 1; }
			let new_prefix = parent_prefix + bits_needed;
			if new_prefix > 32 { println!("Nuk mund të ndahet më tej"); return; }
			let mask = mask_from_prefix_v4(parent_prefix);
			let network = ip & mask;
			let subnet_size = 1u32 << (32 - new_prefix as u32);
			for i in 0..subs.min(16) {
				let subnet_ip = network + (subnet_size * i);
				println!("Subnet {}: {}/{}", i + 1, u32_to_ipv4_string(subnet_ip), new_prefix);
			}
			if subs > 16 { println!("... and {} more", subs - 16); }
		} else { println!("Numër i pavlefshëm i subnetave"); }
	} else { println!("Rjeti kryesor i pavlefshëm"); }
}

pub fn supernetting() {
	println!("Supernetting: krijimi i një rrjeti më të madh nga dy rrjete ekzistuese");
	let net1 = read_input("Vendos rrjetin e parë (x.x.x.x/24): ");
	let net2 = read_input("Vendos rrjetin e dytë (x.x.x.x/24): ");
	if let (Some((ip1, p1)), Some((ip2, p2))) = (ipv4_str_to_u32(&net1), ipv4_str_to_u32(&net2)) {
		if p1 != p2 { println!("Rjetat duhet të kenë të njëjtin prefiks"); return; }
		let xor = ip1 ^ ip2;
		let mut bits = 0u32;
		for i in (0..32).rev() {
			if ((xor >> i) & 1) != 0 {
				bits = 32 - i - 1;
				break;
			}
		}
		let supernet_prefix = (32 - bits - 1).max(0);
		let mask = mask_from_prefix_v4(supernet_prefix as u8);
		let supernet = ip1 & mask;
		println!("Supernet: {}/{}", u32_to_ipv4_string(supernet), supernet_prefix);
	} else { println!("Rjetet e pavlefshme"); }
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
