use std::net::Ipv6Addr;

pub fn ipv4_str_to_u32(s: &str) -> Option<(u32, u8)> {
	let parts: Vec<&str> = s.split('/').collect();
	let ip_part = parts.get(0)?;
	let prefix: u8 = if parts.len() > 1 { parts[1].parse().ok()? } else { 32 };
	let octets: Vec<&str> = ip_part.split('.').collect();
	if octets.len() != 4 || prefix > 32 { return None }
	let mut ip: u32 = 0;
	for &o in &octets {
		let v: u8 = o.parse().ok()?;
		ip = (ip << 8) | (v as u32);
	}
	Some((ip, prefix))
}

pub fn u32_to_ipv4_string(ip: u32) -> String {
	format!("{}.{}.{}.{}",
		(ip >> 24) & 0xff,
		(ip >> 16) & 0xff,
		(ip >> 8) & 0xff,
		ip & 0xff)
}

pub fn mask_from_prefix_v4(prefix: u8) -> u32 {
	if prefix == 0 { 0 }
	else if prefix >= 32 { u32::MAX }
	else { u32::MAX << (32 - prefix) }
}

pub fn ipv6_str_to_u128(s: &str) -> Option<(u128, u8)> {
	let parts: Vec<&str> = s.split('/').collect();
	let ip_part = parts.get(0)?;
	let prefix: u8 = if parts.len() > 1 { parts[1].parse().ok()? } else { 128 };
	let addr: Ipv6Addr = ip_part.parse().ok()?;
	let segs = addr.segments();
	let mut v: u128 = 0;
	for &seg in &segs {
		v = (v << 16) | (seg as u128);
	}
	Some((v, prefix))
}

pub fn u128_to_ipv6_string(ip: u128) -> String {
	let mut segs = [0u16; 8];
	let mut v = ip;
	for i in (0..8).rev() {
		segs[i] = (v & 0xffff) as u16;
		v >>= 16;
	}
	let addr = Ipv6Addr::new(
		segs[0], segs[1], segs[2], segs[3], segs[4], segs[5], segs[6], segs[7]
	);
	addr.to_string()
}

pub fn mask_from_prefix_v6(prefix: u8) -> u128 {
	if prefix == 0 { 0 }
	else if prefix >= 128 { u128::MAX }
	else { u128::MAX << (128 - prefix) }
}

// --- Binary / Decimal conversion helpers ---

pub fn ipv4_to_binary_string(s: &str) -> Option<String> {
	let parts: Vec<&str> = s.split('/').collect();
	let ip_part = parts.get(0)?;
	let octets: Vec<&str> = ip_part.split('.').collect();
	if octets.len() != 4 { return None }
	let mut bins = Vec::with_capacity(4);
	for &o in &octets {
		let v: u8 = o.parse().ok()?;
		bins.push(format!("{:08b}", v));
	}
	Some(bins.join("."))
}

pub fn ipv4_binary_to_decimal(s: &str) -> Option<String> {
	let octs: Vec<&str> = s.split('.').collect();
	if octs.len() != 4 { return None }
	let mut parts = Vec::with_capacity(4);
	for &b in &octs {
		let clean = b.trim();
		if clean.is_empty() { return None }
		let v = u8::from_str_radix(clean, 2).ok()?;
		parts.push(v.to_string());
	}
	Some(parts.join("."))
}

/// Parse a binary string (with optional whitespace) into a decimal `u128`.
pub fn binary_to_decimal_generic(s: &str) -> Option<u128> {
	let clean: String = s.chars().filter(|c| !c.is_whitespace()).collect();
	if clean.is_empty() { return None }
	u128::from_str_radix(&clean, 2).ok()
}

/// Convert a decimal number to its binary representation (no padding).
pub fn decimal_to_binary_generic(n: u128) -> String {
	format!("{:b}", n)
}

pub fn ipv4_octet_to_binary(octet: u8) -> String {
	format!("{:08b}", octet)
}

pub fn ipv4_binary_octet_to_decimal(s: &str) -> Option<u8> {
	u8::from_str_radix(s.trim(), 2).ok()
}

// IPv6 hex/decimal helpers
pub fn ipv6_to_decimal_string(s: &str) -> Option<String> {
	ipv6_str_to_u128(s).map(|(v, _)| v.to_string())
}

pub fn ipv6_decimal_to_string(s: &str) -> Option<String> {
	let clean = s.trim();
	let v = clean.parse::<u128>().ok()?;
	Some(u128_to_ipv6_string(v))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ipv4_parsing_and_mask() {
		let (ip, prefix) = ipv4_str_to_u32("192.168.1.10/24").unwrap();
		assert_eq!(prefix, 24);
		assert_eq!(u32_to_ipv4_string(mask_from_prefix_v4(24)), "255.255.255.0");
		assert_eq!(u32_to_ipv4_string(ip & mask_from_prefix_v4(24)), "192.168.1.0");
	}

	#[test]
	fn test_ipv4_binary_roundtrip() {
		let b = ipv4_to_binary_string("192.168.1.1").unwrap();
		assert_eq!(b, "11000000.10101000.00000001.00000001");
		let dec = ipv4_binary_to_decimal(&b).unwrap();
		assert_eq!(dec, "192.168.1.1");
	}

	#[test]
	fn test_ipv6_parsing_roundtrip() {
		let (v, prefix) = ipv6_str_to_u128("2001:db8::1/64").unwrap();
		assert_eq!(prefix, 64);
		let s = u128_to_ipv6_string(v);
		assert!(s.starts_with("2001:db8"));
		let dec = ipv6_to_decimal_string("2001:db8::1/64").unwrap();
		let back = ipv6_decimal_to_string(&dec).unwrap();
		assert!(back.starts_with("2001:db8"));
	}
}
