use ipcalculator::models::ipv4_addr::IPv4Address;

#[test]
fn test_ipv4_creation() {
    let addr = IPv4Address::new(0xc0a80101, 24); // 192.168.1.1/24
    assert_eq!(addr.ip, 0xc0a80101);
    assert_eq!(addr.prefix, 24);
}

#[test]
fn test_ipv4_mask_calculation() {
    let addr = IPv4Address::new(0xc0a80101, 24);
    let mask = addr.mask();
    assert_eq!(mask, 0xffffff00); // 255.255.255.0
}

#[test]
fn test_ipv4_network_address() {
    let addr = IPv4Address::new(0xc0a80150, 24); // 192.168.1.80/24
    let network = addr.network();
    assert_eq!(network, 0xc0a80100); // 192.168.1.0
}

#[test]
fn test_ipv4_broadcast_address() {
    let addr = IPv4Address::new(0xc0a80150, 24); // 192.168.1.80/24
    let broadcast = addr.broadcast();
    assert_eq!(broadcast, 0xc0a801ff); // 192.168.1.255
}

#[test]
fn test_ipv4_first_host() {
    let addr = IPv4Address::new(0xc0a80150, 24);
    let first = addr.first_host().unwrap();
    assert_eq!(first, 0xc0a80101); // 192.168.1.1
}

#[test]
fn test_ipv4_last_host() {
    let addr = IPv4Address::new(0xc0a80150, 24);
    let last = addr.last_host().unwrap();
    assert_eq!(last, 0xc0a801fe); // 192.168.1.254
}

#[test]
fn test_ipv4_num_hosts_24() {
    let addr = IPv4Address::new(0xc0a80150, 24);
    let hosts = addr.num_hosts();
    assert_eq!(hosts, 254); // 2^(32-24) - 2 = 254
}

#[test]
fn test_ipv4_num_hosts_32() {
    let addr = IPv4Address::new(0xc0a80101, 32);
    let hosts = addr.num_hosts();
    assert_eq!(hosts, 1);
}

#[test]
fn test_ipv4_num_hosts_31() {
    let addr = IPv4Address::new(0xc0a80100, 31);
    let hosts = addr.num_hosts();
    assert_eq!(hosts, 2); // Point-to-point /31
}

#[test]
fn test_ipv4_contains() {
    let network = IPv4Address::new(0xc0a80100, 24); // 192.168.1.0/24
    
    let in_network = 0xc0a80150; // 192.168.1.80
    let out_network = 0xc0a90100; // 192.168.144.0
    
    assert!(network.contains(in_network));
    assert!(!network.contains(out_network));
}

#[test]
fn test_ipv4_to_string() {
    let addr = IPv4Address::new(0xc0a80101, 24); // 192.168.1.1/24
    let ip_str = addr.to_string_ip();
    assert_eq!(ip_str, "192.168.1.1");
}

#[test]
fn test_ipv4_mask_to_string() {
    let addr = IPv4Address::new(0xc0a80101, 24);
    let mask_str = addr.mask_to_string();
    assert_eq!(mask_str, "255.255.255.0");
}

#[test]
fn test_ipv4_cidr_string() {
    let addr = IPv4Address::new(0xc0a80101, 24);
    let cidr = addr.to_cidr_string();
    assert_eq!(cidr, "192.168.1.1/24");
}

#[test]
fn test_ipv4_prefix_zero() {
    let addr = IPv4Address::new(0, 0);
    let mask = addr.mask();
    assert_eq!(mask, 0);
}

#[test]
fn test_ipv4_prefix_32() {
    let addr = IPv4Address::new(0xffffffff, 32);
    let mask = addr.mask();
    assert_eq!(mask, 0xffffffff);
}

#[test]
fn test_ipv4_class_a() {
    let addr = IPv4Address::new(0x01000001, 8); // 1.0.0.1/8
    let network = addr.network();
    assert_eq!(network, 0x01000000); // 1.0.0.0
}

#[test]
fn test_ipv4_class_b() {
    let addr = IPv4Address::new(0x80000001, 16); // 128.0.0.1/16
    let network = addr.network();
    assert_eq!(network, 0x80000000); // 128.0.0.0
}

#[test]
fn test_ipv4_class_c() {
    let addr = IPv4Address::new(0xc0000001, 24); // 192.0.0.1/24
    let network = addr.network();
    assert_eq!(network, 0xc0000000); // 192.0.0.0
}

#[test]
fn test_ipv4_private_range_class_a() {
    let network = IPv4Address::new(0x0a000000, 8); // 10.0.0.0/8
    let private_ip = 0x0a0a0a0a; // 10.10.10.10
    assert!(network.contains(private_ip));
}

#[test]
fn test_ipv4_private_range_class_b() {
    let network = IPv4Address::new(0xac100000, 12); // 172.16.0.0/12
    let private_ip = 0xac100001; // 172.16.0.1
    assert!(network.contains(private_ip));
}

#[test]
fn test_ipv4_private_range_class_c() {
    let network = IPv4Address::new(0xc0a80000, 16); // 192.168.0.0/16
    let private_ip = 0xc0a80001; // 192.168.0.1
    assert!(network.contains(private_ip));
}

#[test]
fn test_ipv4_large_subnet() {
    let addr = IPv4Address::new(0x08000000, 8); // 8.0.0.0/8
    let hosts = addr.num_hosts();
    assert_eq!(hosts, (1u32 << 24) - 2); // 2^24 - 2
}

#[test]
fn test_ipv4_small_subnet() {
    let addr = IPv4Address::new(0xc0a80100, 30); // 192.168.1.0/30
    let hosts = addr.num_hosts();
    assert_eq!(hosts, 2);
}

#[test]
fn test_ipv4_display_impl() {
    let addr = IPv4Address::new(0xc0a80101, 24);
    let display_str = format!("{}", addr);
    assert_eq!(display_str, "192.168.1.1/24");
}
