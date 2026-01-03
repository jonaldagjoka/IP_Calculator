use std::io::{self, Write};
use crate::utils::conversions::{ipv6_str_to_u128, u128_to_ipv6_string, mask_from_prefix_v6};

fn read_input(prompt: &str) -> String {
	print!("{}", prompt);
	io::stdout().flush().ok();
	let mut s = String::new();
	io::stdin().read_line(&mut s).ok();
	s.trim().to_string()
}

pub fn calculate_netid() {
	let input = read_input("Enter IPv6 (format xxxx:...:/64): ");
	if let Some((ip, prefix)) = ipv6_str_to_u128(&input) {
		let mask = mask_from_prefix_v6(prefix);
		let network = ip & mask;
		println!("Network: {}/{}", u128_to_ipv6_string(network), prefix);
	} else { println!("Invalid IPv6 input") }
}

pub fn calculate_broadcast_address() {
	println!("IPv6 has no broadcast address; computing last address in prefix instead.");
	let input = read_input("Enter IPv6 (format xxxx:...:/64): ");
	if let Some((ip, prefix)) = ipv6_str_to_u128(&input) {
		let mask = mask_from_prefix_v6(prefix);
		let network = ip & mask;
		let last = network | (!mask);
		println!("Last address in prefix: {}", u128_to_ipv6_string(last));
	} else { println!("Invalid IPv6 input") }
}

pub fn calculate_ip_range() {
	let input = read_input("Enter IPv6 (format xxxx:...:/64): ");
	if let Some((ip, prefix)) = ipv6_str_to_u128(&input) {
		let mask = mask_from_prefix_v6(prefix);
		let network = ip & mask;
		let last = network | (!mask);
		println!("First: {}", u128_to_ipv6_string(network));
		println!("Last: {}", u128_to_ipv6_string(last));
	} else { println!("Invalid IPv6 input") }
}

pub fn calculate_number_of_hosts() {
	let input = read_input("Enter IPv6 (format xxxx:...:/64): ");
	if let Some((_ip, prefix)) = ipv6_str_to_u128(&input) {
		let host_bits = 128 - prefix as u32;
		if host_bits >= 64 {
			println!("Hosts: 2^{} (very large)", host_bits);
		} else {
			let hosts = (1u128 << host_bits) - 1u128; // don't subtract 2 in IPv6
			println!("Hosts (approx): {}", hosts);
		}
	} else { println!("Invalid IPv6 input") }
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
			if bits >= 128 {
				println!("Too many hosts");
				return;
			}
			let available = 1u128 << bits;
			if available >= req_hosts {
				break;
			}
			bits += 1;
		}
		let prefix = 128u32 - bits;
		println!("Prefix: /{}", prefix);
	} else { println!("Invalid number") }
}

pub fn calculate_subnet_mask_from_subnets() {
	let parent = read_input("Enter parent network (format xxxx:...:/64): ");
	let subnets = read_input("Enter required number of subnets: ");
	if let Some((_ip, parent_prefix)) = ipv6_str_to_u128(&parent) {
		if let Ok(subs) = subnets.parse::<u32>() {
			let mut bits = 0u8;
			while (1u128 << bits) < subs as u128 { bits += 1; }
			let new_prefix = parent_prefix + bits;
			if new_prefix > 128 { println!("Cannot create that many subnets") }
			else { println!("New prefix: /{}", new_prefix); }
		} else { println!("Invalid subnets number") }
	} else { println!("Invalid parent network") }
}

// i: CIDR to Prefix Conversion
pub fn cidr_to_prefix() {
	let cidr = read_input("Enter CIDR prefix (0-128): ");
	if let Ok(prefix) = cidr.parse::<u8>() {
		if prefix > 128 { println!("CIDR must be 0-128"); return; }
		println!("Prefix: /{}", prefix);
	} else { println!("Invalid CIDR") }
}

// j: Prefix to CIDR (same as above for IPv6)
pub fn prefix_to_cidr() {
	let input = read_input("Enter prefix length (0-128): ");
	if let Ok(prefix) = input.parse::<u8>() {
		if prefix > 128 { println!("Prefix must be 0-128"); return; }
		println!("CIDR: /{}", prefix);
	} else { println!("Invalid prefix") }
}

// k: Wildcard Mask Calculation
pub fn wildcard_mask_calculation() {
	let input = read_input("Enter IPv6 prefix (xxxx:...:/64): ");
	if let Some((_ip, prefix)) = ipv6_str_to_u128(&input) {
		let mask = mask_from_prefix_v6(prefix);
		let wildcard = !mask;
		println!("Wildcard (inverted mask): {}", u128_to_ipv6_string(wildcard));
	} else { println!("Invalid input") }
}

// l: VLSM Calculation
pub fn vlsm_calculation() {
	println!("VLSM: Variable Length Subnet Mask");
	let parent = read_input("Enter parent network (xxxx:...:/64): ");
	let num_subnets = read_input("Enter number of subnets needed: ");
	if let Some((ip, parent_prefix)) = ipv6_str_to_u128(&parent) {
		if let Ok(subs) = num_subnets.parse::<u32>() {
			let mut bits_needed = 0u8;
			while (1u128 << bits_needed) < subs as u128 { bits_needed += 1; }
			let new_prefix = parent_prefix + bits_needed;
			if new_prefix > 128 { println!("Cannot subdivide further"); return; }
			let mask = mask_from_prefix_v6(parent_prefix);
			let network = ip & mask;
			let subnet_size = 1u128 << (128 - new_prefix as u32);
			for i in 0..subs.min(16) {
				let subnet_ip = network + (subnet_size * i as u128);
				println!("Subnet {}: {}/{}", i + 1, u128_to_ipv6_string(subnet_ip), new_prefix);
			}
			if subs > 16 { println!("... and {} more", subs - 16); }
		} else { println!("Invalid number") }
	} else { println!("Invalid parent network") }
}

// m: Subnetting
pub fn subnetting() {
	println!("Subnetting: divide network into smaller subnets");
	let network = read_input("Enter network (xxxx:...:/64): ");
	let num_subnets = read_input("Enter number of subnets: ");
	if let Some((ip, prefix)) = ipv6_str_to_u128(&network) {
		if let Ok(subs) = num_subnets.parse::<u32>() {
			let mut bits = 0u8;
			while (1u128 << bits) < subs as u128 { bits += 1; }
			let new_prefix = prefix + bits;
			if new_prefix > 128 { println!("Cannot subnet further"); return; }
			let mask = mask_from_prefix_v6(prefix);
			let net = ip & mask;
			let subnet_size = 1u128 << (128 - new_prefix as u32);
			println!("Subnets (/{}):", new_prefix);
			for i in 0..subs.min(10) {
				let subnet_ip = net + (subnet_size * i as u128);
				println!("  {}/{}", u128_to_ipv6_string(subnet_ip), new_prefix);
			}
			if subs > 10 { println!("  ... and {} more", subs - 10); }
		} else { println!("Invalid number") }
	} else { println!("Invalid network") }
}

// n: Supernetting
pub fn supernetting() {
	println!("Supernetting: combine networks into larger supernet");
	let net1 = read_input("Enter first network (xxxx:...:/64): ");
	let net2 = read_input("Enter second network (xxxx:...:/64): ");
	if let (Some((ip1, p1)), Some((ip2, p2))) = (ipv6_str_to_u128(&net1), ipv6_str_to_u128(&net2)) {
		if p1 != p2 { println!("Networks must have same prefix"); return; }
		let xor = ip1 ^ ip2;
		let mut bits = 0u32;
		for i in (0..128).rev() {
			if ((xor >> i) & 1) != 0 {
				bits = 128 - i - 1;
				break;
			}
		}
		let supernet_prefix = (128 - bits - 1).max(0);
		let mask = mask_from_prefix_v6(supernet_prefix as u8);
		let supernet = ip1 & mask;
		println!("Supernet: {}/{}", u128_to_ipv6_string(supernet), supernet_prefix);
	} else { println!("Invalid networks") }
}

// o: DHCP Range Calculation
pub fn dhcp_range_calculation() {
	let network = read_input("Enter network (xxxx:...:/64): ");
	if let Some((ip, prefix)) = ipv6_str_to_u128(&network) {
		let mask = mask_from_prefix_v6(prefix);
		let net = ip & mask;
		let broadcast = net | (!mask);
		let host_bits = 128 - prefix as u32;
		println!("IPv6 Prefix Range:");
		println!("  First: {}", u128_to_ipv6_string(net));
		println!("  Last: {}", u128_to_ipv6_string(broadcast));
		if host_bits < 128 {
			let _total = 1u128 << host_bits;
			println!("  Total addresses: 2^{}", host_bits);
		} else {
			println!("  Total addresses: 2^128 (extremely large)");
		}
	} else { println!("Invalid network") }
}
