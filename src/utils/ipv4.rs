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
	let input = read_input("Enter IPv4 (format x.x.x.x/24): ");
	if let Some((ip, prefix)) = ipv4_str_to_u32(&input) {
		let mask = mask_from_prefix_v4(prefix);
		let network = ip & mask;
		println!("Network: {}/{}", u32_to_ipv4_string(network), prefix);
	} else { println!("Invalid IPv4 input") }
}

pub fn calculate_broadcast_address() {
	let input = read_input("Enter IPv4 (format x.x.x.x/24): ");
	if let Some((ip, prefix)) = ipv4_str_to_u32(&input) {
		let mask = mask_from_prefix_v4(prefix);
		let network = ip & mask;
		let broadcast = network | (!mask);
		println!("Broadcast: {}", u32_to_ipv4_string(broadcast));
	} else { println!("Invalid IPv4 input") }
}

pub fn calculate_ip_range() {
	let input = read_input("Enter IPv4 (format x.x.x.x/24): ");
	if let Some((ip, prefix)) = ipv4_str_to_u32(&input) {
		let mask = mask_from_prefix_v4(prefix);
		let network = ip & mask;
		let broadcast = network | (!mask);
		let host_bits = 32 - prefix as u32;
		if host_bits == 0 {
			println!("No usable hosts in /32");
			return;
		}
		if host_bits == 1 {
			println!("/31 network (typically point-to-point): {} - {}",
				u32_to_ipv4_string(network), u32_to_ipv4_string(broadcast));
			return;
		}
		let first = network + 1;
		let last = broadcast - 1;
		println!("First host: {}", u32_to_ipv4_string(first));
		println!("Last host: {}", u32_to_ipv4_string(last));
	} else { println!("Invalid IPv4 input") }
}

pub fn calculate_number_of_hosts() {
	let input = read_input("Enter IPv4 (format x.x.x.x/24): ");
	if let Some((_ip, prefix)) = ipv4_str_to_u32(&input) {
		let host_bits = 32 - prefix as u32;
		if host_bits == 0 {
			println!("Hosts: 1 (single address)");
		} else if host_bits == 1 {
			println!("Hosts: 2 (point-to-point /31)");
		} else {
			let hosts = (1u128 << host_bits) - 2;
			println!("Hosts: {}", hosts);
		}
	} else { println!("Invalid IPv4 input") }
}

pub fn calculate_subnet_mask_from_hosts() {
	let input = read_input("Enter required number of hosts: ");
	if let Ok(req_hosts) = input.parse::<u128>() {
		if req_hosts == 0 {
			println!("At least 1 host required");
			return;
		}
		let mut bits = 1u32;
		loop {
			if bits >= 32 {
				println!("Too many hosts");
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
	} else { println!("Invalid number") }
}

pub fn calculate_subnet_mask_from_subnets() {
	let parent = read_input("Enter parent network (format x.x.x.x/24): ");
	let subnets = read_input("Enter required number of subnets: ");
	if let Some((_ip, parent_prefix)) = ipv4_str_to_u32(&parent) {
		if let Ok(subs) = subnets.parse::<u32>() {
			let mut bits = 0u8;
			while (1u128 << bits) < subs as u128 { bits += 1; }
			let new_prefix = parent_prefix + bits;
			if new_prefix > 32 { println!("Cannot create that many subnets") }
			else {
				let mask = mask_from_prefix_v4(new_prefix);
				println!("New prefix: /{}  Mask: {}", new_prefix, u32_to_ipv4_string(mask));
			}
		} else { println!("Invalid subnets number") }
	} else { println!("Invalid parent network") }
}

// i: CIDR to Subnet Mask Conversion
pub fn cidr_to_subnet_mask() {
	let cidr = read_input("Enter CIDR prefix (0-32): ");
	if let Ok(prefix) = cidr.parse::<u8>() {
		if prefix > 32 { println!("CIDR must be 0-32"); return; }
		let mask = mask_from_prefix_v4(prefix);
		println!("Subnet Mask: {}", u32_to_ipv4_string(mask));
	} else { println!("Invalid CIDR") }
}

// j: Subnet Mask to CIDR Conversion
pub fn subnet_mask_to_cidr() {
	let input = read_input("Enter subnet mask (x.x.x.x): ");
	if let Some((mask, _)) = ipv4_str_to_u32(&input) {
		let mut prefix = 0u8;
		let mut test = 0x80000000u32;
		while (mask & test) != 0 && prefix < 32 {
			prefix += 1;
			test >>= 1;
		}
		println!("CIDR: /{}", prefix);
	} else { println!("Invalid subnet mask") }
}

// k: Wildcard Mask Calculation
pub fn wildcard_mask_calculation() {
	let input = read_input("Enter CIDR (x.x.x.x/24) or just mask (x.x.x.x): ");
	if let Some((_ip, prefix)) = ipv4_str_to_u32(&input) {
		let mask = mask_from_prefix_v4(prefix);
		let wildcard = !mask;
		println!("Wildcard Mask: {}", u32_to_ipv4_string(wildcard));
	} else { println!("Invalid input") }
}

// l: VLSM Calculation
pub fn vlsm_calculation() {
	println!("VLSM: Variable Length Subnet Mask");
	let parent = read_input("Enter parent network (x.x.x.x/24): ");
	let num_subnets = read_input("Enter number of subnets needed: ");
	if let Some((ip, parent_prefix)) = ipv4_str_to_u32(&parent) {
		if let Ok(subs) = num_subnets.parse::<u32>() {
			let mut bits_needed = 0u8;
			while (1u128 << bits_needed) < subs as u128 { bits_needed += 1; }
			let new_prefix = parent_prefix + bits_needed;
			if new_prefix > 32 { println!("Cannot subdivide further"); return; }
			let mask = mask_from_prefix_v4(parent_prefix);
			let network = ip & mask;
			let subnet_size = 1u32 << (32 - new_prefix as u32);
			for i in 0..subs.min(16) {
				let subnet_ip = network + (subnet_size * i);
				println!("Subnet {}: {}/{}", i + 1, u32_to_ipv4_string(subnet_ip), new_prefix);
			}
			if subs > 16 { println!("... and {} more", subs - 16); }
		} else { println!("Invalid number") }
	} else { println!("Invalid parent network") }
}

// m: Subnetting
pub fn subnetting() {
	println!("Subnetting: divide network into smaller subnets");
	let network = read_input("Enter network (x.x.x.x/24): ");
	let num_subnets = read_input("Enter number of subnets: ");
	if let Some((ip, prefix)) = ipv4_str_to_u32(&network) {
		if let Ok(subs) = num_subnets.parse::<u32>() {
			let mut bits = 0u8;
			while (1u128 << bits) < subs as u128 { bits += 1; }
			let new_prefix = prefix + bits;
			if new_prefix > 32 { println!("Cannot subnet further"); return; }
			let mask = mask_from_prefix_v4(prefix);
			let net = ip & mask;
			let subnet_size = 1u32 << (32 - new_prefix as u32);
			println!("Subnets (/{}):", new_prefix);
			for i in 0..subs.min(10) {
				let subnet_ip = net + (subnet_size * i);
				println!("  {}/{}", u32_to_ipv4_string(subnet_ip), new_prefix);
			}
			if subs > 10 { println!("  ... and {} more", subs - 10); }
		} else { println!("Invalid number") }
	} else { println!("Invalid network") }
}

// n: Supernetting
pub fn supernetting() {
	println!("Supernetting: combine networks into larger supernet");
	let net1 = read_input("Enter first network (x.x.x.x/24): ");
	let net2 = read_input("Enter second network (x.x.x.x/24): ");
	if let (Some((ip1, p1)), Some((ip2, p2))) = (ipv4_str_to_u32(&net1), ipv4_str_to_u32(&net2)) {
		if p1 != p2 { println!("Networks must have same prefix"); return; }
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
	} else { println!("Invalid networks") }
}

// o: DHCP Range Calculation
pub fn dhcp_range_calculation() {
	let network = read_input("Enter network (x.x.x.x/24): ");
	if let Some((ip, prefix)) = ipv4_str_to_u32(&network) {
		let mask = mask_from_prefix_v4(prefix);
		let net = ip & mask;
		let broadcast = net | (!mask);
		let first_host = net + 1;
		let last_host = broadcast - 1;
		println!("DHCP Pool:");
		println!("  Start: {}", u32_to_ipv4_string(first_host));
		println!("  End: {}", u32_to_ipv4_string(last_host));
		let total = if last_host >= first_host { last_host - first_host + 1 } else { 0 };
		println!("  Total IPs: {}", total);
	} else { println!("Invalid network") }
}

// p: IP belong check
pub fn ip_belong_check() {
	let network = read_input("Enter network (x.x.x.x/24): ");
	let check_ip = read_input("Enter IP to check (x.x.x.x): ");
	if let (Some((net_ip, prefix)), Some((check, _))) = (ipv4_str_to_u32(&network), ipv4_str_to_u32(&check_ip)) {
		let mask = mask_from_prefix_v4(prefix);
		let network_addr = net_ip & mask;
		let check_addr = check & mask;
		if network_addr == check_addr {
			println!("✓ IP {} belongs to network {}/{}", u32_to_ipv4_string(check), u32_to_ipv4_string(network_addr), prefix);
		} else {
			println!("✗ IP {} does NOT belong to network {}/{}", u32_to_ipv4_string(check), u32_to_ipv4_string(network_addr), prefix);
		}
	} else { println!("Invalid input") }
}
