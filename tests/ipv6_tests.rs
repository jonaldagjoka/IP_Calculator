use ipcalculator::models::ipv6_addr::IPv6Address;

#[test]
fn test_ipv6_creation() {
    let addr = IPv6Address::new(0x20010db8000000000000000000000001, 64);
    assert_eq!(addr.prefix, 64);
}

#[test]
fn test_ipv6_mask_64() {
    let addr = IPv6Address::new(0, 64);
    assert_eq!(addr.mask(), 0xffffffffffffffff0000000000000000);
}

#[test]
fn test_ipv6_network() {
    let addr = IPv6Address::new(0x20010db8000000000000000000000001, 64);
    assert_eq!(addr.network(), 0x20010db8000000000000000000000000);
}

#[test]
fn test_ipv6_first_last_address() {
    let addr = IPv6Address::new(0x20010db800000000000000000000abcd, 64);

    assert_eq!(
        addr.first_address(),
        0x20010db8000000000000000000000000
    );

    assert_eq!(
        addr.last_address(),
        0x20010db800000000ffffffffffffffff
    );
}

#[test]
fn test_ipv6_num_addresses() {
    let addr = IPv6Address::new(0, 64);
    assert_eq!(addr.num_addresses(), 1u128 << 64);
}

#[test]
fn test_ipv6_contains() {
    let net = IPv6Address::new(0x20010db8000000000000000000000000, 64);
    assert!(net.contains(0x20010db8000000000000000000000001));
    assert!(!net.contains(0x20010db9000000000000000000000001));
}

#[test]
fn test_multicast() {
    let addr = IPv6Address::new(0xff020000000000000000000000000001, 128);
    assert!(addr.is_multicast());
}

#[test]
fn test_link_local() {
    let addr = IPv6Address::new(0xfe800000000000000000000000000001, 128);
    assert!(addr.is_link_local());
}

#[test]
fn test_unique_local() {
    let addr = IPv6Address::new(0xfd000000000000000000000000000001, 128);
    assert!(addr.is_private());
}

#[test]
fn test_address_type() {
    let g = IPv6Address::new(0x20010db8000000000000000000000001, 128);
    assert_eq!(g.address_type(), "Global Unicast");
}

#[test]
fn test_display_and_cidr() {
    let addr = IPv6Address::new(0x20010db8000000000000000000000001, 64);
    let s = addr.to_string();
    assert!(s.contains("2001"));
    assert!(addr.to_cidr_string().contains("/64"));
}
