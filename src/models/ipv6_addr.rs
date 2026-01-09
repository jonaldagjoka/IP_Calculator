use std::fmt;
use std::net::Ipv6Addr;

/// Represents an IPv6 address with prefix notation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IPv6Address {
    pub ip: u128,
    pub prefix: u8,
}

impl IPv6Address {
    /// Create a new IPv6 address
    pub fn new(ip: u128, prefix: u8) -> Self {
        IPv6Address { ip, prefix }
    }

    /// Get the prefix mask
    pub fn mask(&self) -> u128 {
        if self.prefix == 0 { 0 }
        else if self.prefix >= 128 { u128::MAX }
        else { u128::MAX << (128 - self.prefix) }
    }

    /// Get the network address
    pub fn network(&self) -> u128 {
        self.ip & self.mask()
    }

    /// Get the last address in the prefix
    pub fn last_address(&self) -> u128 {
        self.network() | !self.mask()
    }

    /// Get the first usable address (usually same as network for IPv6)
    pub fn first_address(&self) -> u128 {
        self.network()
    }

    /// Get the number of addresses in this prefix
    pub fn num_addresses(&self) -> u128 {
        let host_bits = 128 - self.prefix as u32;
        if host_bits >= 128 { u128::MAX }
        else { 1u128 << host_bits }
    }

    /// Check if an IPv6 address belongs to this network
    pub fn contains(&self, other: u128) -> bool {
        (other & self.mask()) == self.network()
    }

    /// Convert u128 to IPv6 string representation
    pub fn ip_to_string(&self) -> String {
        let mut segs = [0u16; 8];
        let mut v = self.ip;
        for i in (0..8).rev() {
            segs[i] = (v & 0xffff) as u16;
            v >>= 16;
        }
        let addr = Ipv6Addr::new(
            segs[0], segs[1], segs[2], segs[3], segs[4], segs[5], segs[6], segs[7]
        );
        addr.to_string()
    }

    /// Format as CIDR notation (xxxx:xxxx::.../prefix)
    pub fn to_cidr_string(&self) -> String {
        format!("{}/{}", self.ip_to_string(), self.prefix)
    }

    /// Get the address type (unicast, multicast, link-local, loopback, etc.)
    pub fn address_type(&self) -> &'static str {
        // Precise checks for common IPv6 address types
        if self.ip == 0 { return "Unspecified"; }
        if self.ip == 1 { return "Loopback"; }

        // Multicast: ff00::/8
        if (self.ip >> 120) as u8 == 0xff { return "Multicast"; }

        // Link-local: fe80::/10 -> top 10 bits 0b1111111010 (0x3fa)
        if ((self.ip >> 118) & 0x3ff) == 0x3fa { return "Link-Local"; }

        // Unique local: fc00::/7 (fc00 or fd00)
        let first_byte = (self.ip >> 120) as u8;
        if first_byte & 0xfe == 0xfc { return "Unique Local"; }

        // Default to global unicast
        "Global Unicast"
    }

    /// Check if the address is multicast
    pub fn is_multicast(&self) -> bool {
        (self.ip >> 120) == 0xff
    }

    /// Check if the address is link-local
    pub fn is_link_local(&self) -> bool {
        let mask: u128=0xffc0_0000_0000_0000_0000_0000_0000_0000;
         let link_local: u128 = 0xfe80_0000_0000_0000_0000_0000_0000_0000;
        (self.ip & mask)==link_local
    }

    /// Check if the address is private/unique local
    pub fn is_private(&self) -> bool {
    
        (self.ip >> 121) == 0b1111110
    }
}

impl fmt::Display for IPv6Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.ip_to_string(), self.prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv6_network() {
        // 2001:db8::/32
        let addr = IPv6Address::new(0x20010db8_00000000_00000000_00000000, 32);
        let net = addr.network();
        assert_eq!(net >> 96, 0x20010db8);
    }

    #[test]
    fn test_ipv6_contains() {
        // 2001:db8::/32
        let addr = IPv6Address::new(0x20010db8_00000000_00000000_00000001, 32);
        // 2001:db8::1 should be in the network
        assert!(addr.contains(0x20010db8_00000000_00000000_00000001));
        // 2001:db9::1 should NOT be in the network
        assert!(!addr.contains(0x20010db9_00000000_00000000_00000001));
    }
}
