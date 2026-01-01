use std::fmt;

/// Represents an IPv4 address with CIDR notation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IPv4Address {
    pub ip: u32,
    pub prefix: u8,
}

impl IPv4Address {
    /// Create a new IPv4 address
    pub fn new(ip: u32, prefix: u8) -> Self {
        IPv4Address { ip, prefix }
    }

    /// Get the subnet mask
    pub fn mask(&self) -> u32 {
        if self.prefix == 0 { 0 }
        else if self.prefix >= 32 { u32::MAX }
        else { u32::MAX << (32 - self.prefix) }
    }

    /// Get the network address
    pub fn network(&self) -> u32 {
        self.ip & self.mask()
    }

    /// Get the broadcast address
    pub fn broadcast(&self) -> u32 {
        self.network() | !self.mask()
    }

    /// Get the first usable host IP
    pub fn first_host(&self) -> Option<u32> {
        match self.prefix {
            32 => None,
            31 => Some(self.network()),
            _ => Some(self.network() + 1),
        }
    }

    /// Get the last usable host IP
    pub fn last_host(&self) -> Option<u32> {
        match self.prefix {
            32 => None,
            31 => Some(self.broadcast()),
            _ => Some(self.broadcast() - 1),
        }
    }

    /// Get the number of usable hosts
    pub fn num_hosts(&self) -> u32 {
        let host_bits = 32 - self.prefix as u32;
        match self.prefix {
            32 => 1,
            31 => 2,
            _ => (1u64 << host_bits).saturating_sub(2) as u32,
        }
    }

    /// Check if an IP address belongs to this network
    pub fn contains(&self, other: u32) -> bool {
        (other & self.mask()) == self.network()
    }

    /// Format IPv4 address as string (x.x.x.x)
    pub fn to_string_ip(&self) -> String {
        format!("{}.{}.{}.{}",
            (self.ip >> 24) & 0xff,
            (self.ip >> 16) & 0xff,
            (self.ip >> 8) & 0xff,
            self.ip & 0xff)
    }

    /// Format subnet mask as string
    pub fn mask_to_string(&self) -> String {
        let m = self.mask();
        format!("{}.{}.{}.{}",
            (m >> 24) & 0xff,
            (m >> 16) & 0xff,
            (m >> 8) & 0xff,
            m & 0xff)
    }

    /// Format as CIDR notation (x.x.x.x/prefix)
    pub fn to_cidr_string(&self) -> String {
        format!("{}/{}", self.to_string_ip(), self.prefix)
    }
}

impl fmt::Display for IPv4Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.to_string_ip(), self.prefix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_network() {
        let addr = IPv4Address::new(0xc0a80101, 24); // 192.168.1.1/24
        assert_eq!(addr.network(), 0xc0a80100); // 192.168.1.0
        assert_eq!(addr.broadcast(), 0xc0a801ff); // 192.168.1.255
    }

    #[test]
    fn test_ipv4_contains() {
        let addr = IPv4Address::new(0xc0a80101, 24);
        assert!(addr.contains(0xc0a80178)); // 192.168.1.120 - yes
        assert!(!addr.contains(0xc0a80201)); // 192.168.2.1 - no
    }

    #[test]
    fn test_ipv4_hosts() {
        let addr = IPv4Address::new(0xc0a80101, 25); // /25 = 128 addresses
        assert_eq!(addr.num_hosts(), 126);
    }
}
