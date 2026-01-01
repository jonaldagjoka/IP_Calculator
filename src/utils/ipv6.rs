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
