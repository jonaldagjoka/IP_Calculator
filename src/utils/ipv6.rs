use std::io::{self, Write};
use crate::utils::conversions::{ipv6_str_to_u128, u128_to_ipv6_string, mask_from_prefix_v6};

fn read_input(prompt: &str) -> String {
	print!("{}", prompt);
	io::stdout().flush().ok();
	let mut s = String::new();
	io::stdin().read_line(&mut s).ok();
	s.trim().to_string()
}

// a: Llogarit Network Prefix (NetID)
pub fn calculate_netid() {
	let input = read_input("Vendos IPv6 (format xxxx:...:/64): ");
	if let Some((ip, prefix)) = ipv6_str_to_u128(&input) {
		let mask = mask_from_prefix_v6(prefix);
		let network = ip & mask;
		println!("Network Prefix: {}/{}", u128_to_ipv6_string(network), prefix);
	} else { println!("Input i pavlefshëm") }
}

// b: Llogarit IP Range
pub fn calculate_ip_range() {
	let input = read_input("Vendos IPv6 (format xxxx:...:/64): ");
	if let Some((ip, prefix)) = ipv6_str_to_u128(&input) {
		let mask = mask_from_prefix_v6(prefix);
		let network = ip & mask;
		let last = network | (!mask);
		println!("First Address: {}", u128_to_ipv6_string(network));
		println!("Last Address: {}", u128_to_ipv6_string(last));
	} else { println!("Input i pavlefshëm") }
}

// d: IPv6 Expansion (shkruaj formën e plotë)
pub fn expand_ipv6() {
	let input = read_input("Vendos IPv6 në formë të shkurtuar (p.sh. 2001:db8::1/64): ");
	
	if let Some((ip, prefix)) = ipv6_str_to_u128(&input) {
		// Convert to full expanded form manually
		let expanded = format!(
			"{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}:{:04x}",
			(ip >> 112) & 0xffff,
			(ip >> 96) & 0xffff,
			(ip >> 80) & 0xffff,
			(ip >> 64) & 0xffff,
			(ip >> 48) & 0xffff,
			(ip >> 32) & 0xffff,
			(ip >> 16) & 0xffff,
			ip & 0xffff
		);
		println!("Forma e plotë: {}/{}", expanded, prefix);
	} else { 
		println!("Input i pavlefshëm") 
	}
}


// e: IPv6 Compression (shkurto adresën)
pub fn compress_ipv6() {
	let input = read_input("Vendos IPv6 në formë të plotë (p.sh. 2001:0db8:0000:0000:0000:0000:0000:0001): ");
	if let Some((ip, prefix)) = ipv6_str_to_u128(&input) {
		let compressed = u128_to_ipv6_string(ip);
		println!("Forma e shkurtuar: {}/{}", compressed, prefix);
	} else { println!("Input i pavlefshëm") }
}

// f: Hex to Decimal Conversion
pub fn hex_to_decimal() {
	let input = read_input("Vendos një segment IPv6 në hex (p.sh. 2001): ");
	if let Ok(hex_val) = u16::from_str_radix(&input, 16) {
		println!("Decimal: {}", hex_val);
	} else {
		println!("Input i pavlefshëm - duhet të jetë hexadecimal");
	}
}

// g: Decimal to Hex Conversion
pub fn decimal_to_hex() {
	let input = read_input("Vendos një vlerë decimal (0-65535): ");
	if let Ok(dec_val) = input.parse::<u16>() {
		println!("Hexadecimal: {:x}", dec_val);
	} else {
		println!("Input i pavlefshëm - duhet të jetë numër decimal");
	}
}

// i: IPv6 Address Type Identifier
pub fn address_type_identifier() {
	let input = read_input("Vendos IPv6 address: ");
	if let Some((ip, _prefix)) = ipv6_str_to_u128(&input) {
		let addr_type = if (ip >> 120) as u8 == 0xff {
			"Multicast"
		} else if ((ip >> 118) & 0x3ff) == 0x3fa {
			"Link-Local"
		} else if (ip >> 120) as u8 & 0xfe == 0xfc {
			"Unique Local (Private)"
		} else if ip == 0 {
			"Unspecified"
		} else if ip == 1 {
			"Loopback"
		} else {
			"Global Unicast"
		};
		println!("Address Type: {}", addr_type);
	} else { println!("Input i pavlefshëm") }
}

pub fn generate_eui64() {
	let mac = read_input("Vendos MAC address (format: xx:xx:xx:xx:xx:xx): ");
	let prefix = read_input("Vendos network prefix (p.sh. 2001:db8::/64): ");
	
	// Parse MAC address
	let mac_parts: Vec<&str> = mac.split(':').collect();
	if mac_parts.len() != 6 {
		println!("MAC address i pavlefshëm");
		return;
	}
	
	let mut mac_bytes = [0u8; 6];
	for (i, part) in mac_parts.iter().enumerate() {
		if let Ok(byte) = u8::from_str_radix(part, 16) {
			mac_bytes[i] = byte;
		} else {
			println!("MAC address i pavlefshëm");
			return;
		}
	}
	
	// Flip the 7th bit (universal/local bit)
	mac_bytes[0] ^= 0x02;
	
	// Build EUI-64: MAC[0:3] + FF:FE + MAC[3:6]
	let interface_id = ((mac_bytes[0] as u64) << 56)
		| ((mac_bytes[1] as u64) << 48)
		| ((mac_bytes[2] as u64) << 40)
		| (0xff << 32)
		| (0xfe << 24)
		| ((mac_bytes[3] as u64) << 16)
		| ((mac_bytes[4] as u64) << 8)
		| (mac_bytes[5] as u64);
	
	// Parse network prefix
	if let Some((network_ip, net_prefix)) = ipv6_str_to_u128(&prefix) {
		let mask = mask_from_prefix_v6(net_prefix);
		let network = network_ip & mask;
		let full_address = network | (interface_id as u128);
		
		println!("EUI-64 Address: {}/{}", u128_to_ipv6_string(full_address), net_prefix);
	} else {
		println!("Network prefix i pavlefshëm");
	}
}

// l: Subnetting
pub fn subnetting() {
	let network = read_input("Vendos rrjetin (xxxx:...:/64): ");
	let num_subnets = read_input("Vendos numrin e subneteve: ");
	if let Some((ip, prefix)) = ipv6_str_to_u128(&network) {
		if let Ok(subs) = num_subnets.parse::<u32>() {
			let mut bits = 0u8;
			while (1u128 << bits) < subs as u128 { bits += 1; }
			let new_prefix = prefix + bits;
			if new_prefix > 128 { println!("Nuk mund të nënndahet më tej"); return; }
			let mask = mask_from_prefix_v6(prefix);
			let net = ip & mask;
			let subnet_size = if new_prefix < 128 { 1u128 << (128 - new_prefix as u32) } else { 1 };
			println!("Subnete (/{}):", new_prefix);
			for i in 0..subs.min(10) {
				let subnet_ip = net + (subnet_size * i as u128);
				println!("  {}/{}", u128_to_ipv6_string(subnet_ip), new_prefix);
			}
			if subs > 10 { println!("  ... dhe {} të tjera", subs - 10); }
		} else { println!("Input i pavlefshëm") }
	} else { println!("Input i pavlefshëm") }
}

// m: Supernetting
pub fn supernetting() {
	let net1 = read_input("Vendos rrjetin e parë (xxxx:...:/64): ");
	let net2 = read_input("Vendos rrjetin e dytë (xxxx:...:/64): ");
	if let (Some((ip1, p1)), Some((ip2, p2))) = (ipv6_str_to_u128(&net1), ipv6_str_to_u128(&net2)) {
		if p1 != p2 { println!("Rjetat duhet të kenë të njëjtin prefiks"); return; }
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
	} else { println!("Input i pavlefshëm") }
}

// n: DHCP Range Calculation
pub fn dhcp_range_calculation() {
	let network = read_input("Vendos rrjetin (xxxx:...:/64): ");
	if let Some((ip, prefix)) = ipv6_str_to_u128(&network) {
		let mask = mask_from_prefix_v6(prefix);
		let net = ip & mask;
		let broadcast = net | (!mask);
		let host_bits = 128 - prefix as u32;
		println!("IPv6 Prefix Range:");
		println!("  First: {}", u128_to_ipv6_string(net));
		println!("  Last: {}", u128_to_ipv6_string(broadcast));
		if host_bits < 128 {
			println!("  Total addresses: 2^{}", host_bits);
		} else {
			println!("  Total addresses: 2^128 (extremely large)");
		}
	} else { println!("Input i pavlefshëm") }
}