use std::io::{self, Write};

pub fn check_ip_classes() {
 println!("  ┌────────────────────────────────────────────────────┐");
 println!("  │               IPv4 Address Classifier              │");
 println!("  └────────────────────────────────────────────────────┘\n");
    print!("Vendos nje adrese IPv4 (x.x.x.x): ");
    io::stdout().flush().ok();
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    let input = input.trim();
    
    let octets: Vec<&str> = input.split('.').collect();
    if octets.len() != 4 {
        println!("IPv4 ne format te pavlefshëm");
        return;
    }
    
    // Parse all octets
    let parsed_octets: Result<Vec<u8>, _> = octets.iter()
        .map(|o| o.parse::<u8>())
        .collect();
    
    if let Ok(octs) = parsed_octets {
        classify_ip(octs[0], octs[1], octs[2], octs[3]);
    } else {
        println!("IP adrese e pavlefshme - oktetet duhet te jene 0-255");
    }
}

fn display_class_a() {
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│                      PUBLIC IP CLASS A                          │");
    println!("├─────────────────────────────────────────────────────────────────┤");
    println!("│ Class A: 1.0.0.0 - 126.255.255.255                              │");
    println!("│   → Default Mask: 255.0.0.0 (/8)                                │");
    println!("│   → Networks: 126 | Hosts per network: 16,777,214              │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│                  PRIVATE IP RANGE - CLASS A                     │");
    println!("├─────────────────────────────────────────────────────────────────┤");
    println!("│ 10.0.0.0 - 10.255.255.255                                       │");
    println!("│   → Class A private range | Mask: 255.0.0.0 (/8)               │");
    println!("│   → 16,777,216 addresses                                        │");
    println!("└─────────────────────────────────────────────────────────────────┘");
}

fn display_class_b() {
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│                      PUBLIC IP CLASS B                          │");
    println!("├─────────────────────────────────────────────────────────────────┤");
    println!("│ Class B: 128.0.0.0 - 191.255.255.255                            │");
    println!("│   → Default Mask: 255.255.0.0 (/16)                             │");
    println!("│   → Networks: 16,384 | Hosts per network: 65,534               │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│                  PRIVATE IP RANGE - CLASS B                     │");
    println!("├─────────────────────────────────────────────────────────────────┤");
    println!("│ 172.16.0.0 - 172.31.255.255                                     │");
    println!("│   → Class B private range | Mask: 255.240.0.0 (/12)            │");
    println!("│   → 1,048,576 addresses                                         │");
    println!("└─────────────────────────────────────────────────────────────────┘");
}

fn display_class_c() {
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│                      PUBLIC IP CLASS C                          │");
    println!("├─────────────────────────────────────────────────────────────────┤");
    println!("│ Class C: 192.0.0.0 - 223.255.255.255                            │");
    println!("│   → Default Mask: 255.255.255.0 (/24)                           │");
    println!("│   → Networks: 2,097,152 | Hosts per network: 254               │");
    println!("└─────────────────────────────────────────────────────────────────┘");
    
    println!("\n┌─────────────────────────────────────────────────────────────────┐");
    println!("│                  PRIVATE IP RANGE - CLASS C                     │");
    println!("├─────────────────────────────────────────────────────────────────┤");
    println!("│ 192.168.0.0 - 192.168.255.255                                   │");
    println!("│   → Class C private range | Mask: 255.255.0.0 (/16)            │");
    println!("│   → 65,536 addresses                                            │");
    println!("└─────────────────────────────────────────────────────────────────┘");
}

fn classify_ip(first: u8, second: u8, third: u8, fourth: u8) {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║       Classification for {:<3}.{:<3}.{:<3}.{:<3}               ║", first, second, third, fourth);
    println!("╚═══════════════════════════════════════════════════════════╝");
    
    // Special cases first
    if first == 0 {
        println!("\n Type: Current Network");
        println!(" Usage: Only valid as source address");
        println!(" Description: Represents 'this network'");
        println!("\n┌─────────────────────────────────────────────────────────────────┐");
        println!("│                    SPECIAL IP RANGE                             │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ 0.0.0.0 - 0.255.255.255                                         │");
        println!("│   → Current network (only valid as source)                      │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        return;
    }
    
    if first == 127 {
        println!("\n Class: A (Special)");
        println!(" Type: Loopback");
        println!(" Usage: Local host communication");
        println!(" Description: Packets never leave the device");
        println!("\n┌─────────────────────────────────────────────────────────────────┐");
        println!("│                    LOOPBACK ADDRESS RANGE                       │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ 127.0.0.0 - 127.255.255.255                                     │");
        println!("│   → Loopback addresses (localhost)                              │");
        println!("│   → Most common: 127.0.0.1                                      │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        return;
    }
    
    if first == 169 && second == 254 {
        println!("\n Type: APIPA (Automatic Private IP Addressing)");
        println!(" Usage: Link-Local address");
        println!(" Description: Auto-assigned when DHCP fails");
        println!("\n┌─────────────────────────────────────────────────────────────────┐");
        println!("│                    APIPA / LINK-LOCAL RANGE                     │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ 169.254.0.0 - 169.254.255.255                                   │");
        println!("│   → APIPA - Automatic Private IP Addressing (Link-Local)        │");
        println!("│   → Used when DHCP fails                                        │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        return;
    }
    
    if first >= 224 && first <= 239 {
        println!("\n Class: D");
        println!(" Type: Multicast");
        println!(" Usage: Group communication, streaming");
        println!(" Description: One-to-many communication");
        println!("\n┌─────────────────────────────────────────────────────────────────┐");
        println!("│                    MULTICAST ADDRESS RANGE                      │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ Class D: 224.0.0.0 - 239.255.255.255                            │");
        println!("│   → Used for group communication                                │");
        println!("│   → No subnet mask (not for host addressing)                    │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        return;
    }
    
    if first >= 240 {
        if first == 255 && second == 255 && third == 255 && fourth == 255 {
            println!("\n Type: Limited Broadcast");
            println!(" Usage: Broadcast to all hosts on local network");
            println!("\n┌─────────────────────────────────────────────────────────────────┐");
            println!("│                  LIMITED BROADCAST ADDRESS                      │");
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ 255.255.255.255                                                 │");
            println!("│   → Limited broadcast address                                   │");
            println!("└─────────────────────────────────────────────────────────────────┘");
        } else {
            println!("\n Class: E");
            println!(" Type: Reserved");
            println!(" Usage: Experimental / Future use");
            println!(" Description: Not available for general use");
            println!("\n┌─────────────────────────────────────────────────────────────────┐");
            println!("│                  RESERVED ADDRESS RANGE - CLASS E               │");
            println!("├─────────────────────────────────────────────────────────────────┤");
            println!("│ Class E: 240.0.0.0 - 255.255.255.255                            │");
            println!("│   → Reserved for future use                                     │");
            println!("└─────────────────────────────────────────────────────────────────┘");
        }
        return;
    }
    
    if first == 100 && second >= 64 && second <= 127 {
        println!("\n Type: Shared Address Space (Carrier-Grade NAT)");
        println!(" Usage: ISP internal networks (RFC 6598)");
        println!("\n┌─────────────────────────────────────────────────────────────────┐");
        println!("│            SHARED ADDRESS SPACE (CARRIER-GRADE NAT)            │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ 100.64.0.0 - 100.127.255.255                                    │");
        println!("│   → Shared Address Space (Carrier-Grade NAT / RFC 6598)        │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        return;
    }
    
    if first == 192 && second == 0 && third == 0 {
        println!("\n Type: IETF Protocol Assignments");
        println!(" Usage: Special protocols");
        println!("\n┌─────────────────────────────────────────────────────────────────┐");
        println!("│                 IETF PROTOCOL ASSIGNMENTS                       │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ 192.0.0.0 - 192.0.0.255                                         │");
        println!("│   → IETF Protocol Assignments                                   │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        return;
    }
    
    if first == 192 && second == 0 && third == 2 {
        println!("\n Type: TEST-NET-1");
        println!(" Usage: Documentation and examples only");
        println!("\n┌─────────────────────────────────────────────────────────────────┐");
        println!("│               DOCUMENTATION RANGE - TEST-NET-1                  │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ 192.0.2.0 - 192.0.2.255                                         │");
        println!("│   → TEST-NET-1 - Documentation and examples                     │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        return;
    }
    
    if first == 198 && second >= 18 && second <= 19 {
        println!("\n Type: Network Benchmark Testing");
        println!(" Usage: Performance testing between networks");
        println!("\n┌─────────────────────────────────────────────────────────────────┐");
        println!("│                 NETWORK BENCHMARK TESTING                       │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ 198.18.0.0 - 198.19.255.255                                     │");
        println!("│   → Network benchmark testing                                   │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        return;
    }
    
    if first == 198 && second == 51 && third == 100 {
        println!("\n Type: TEST-NET-2");
        println!(" Usage: Documentation and examples only");
        println!("\n┌─────────────────────────────────────────────────────────────────┐");
        println!("│               DOCUMENTATION RANGE - TEST-NET-2                  │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ 198.51.100.0 - 198.51.100.255                                   │");
        println!("│   → TEST-NET-2 - Documentation and examples                     │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        return;
    }
    
    if first == 203 && second == 0 && third == 113 {
        println!("\n Type: TEST-NET-3");
        println!(" Usage: Documentation and examples only");
        println!("\n┌─────────────────────────────────────────────────────────────────┐");
        println!("│               DOCUMENTATION RANGE - TEST-NET-3                  │");
        println!("├─────────────────────────────────────────────────────────────────┤");
        println!("│ 203.0.113.0 - 203.0.113.255                                     │");
        println!("│   → TEST-NET-3 - Documentation and examples                     │");
        println!("└─────────────────────────────────────────────────────────────────┘");
        return;
    }

    let class = match first {
        1..=126 => "A",
        128..=191 => "B",
        192..=223 => "C",
        _ => "Unknown",
    };
    
    let default_mask = match class {
        "A" => "255.0.0.0 (/8)",
        "B" => "255.255.0.0 (/16)",
        "C" => "255.255.255.0 (/24)",
        _ => "N/A",
    };
    
    let is_private = match first {
        10 => true,
        172 => second >= 16 && second <= 31,
        192 => second == 168,
        _ => false,
    };

    println!("\n Class: {}", class);
    println!(" Default Subnet Mask: {}", default_mask);

    if is_private {
        println!(" Type: Private (RFC 1918)");
        println!(" Usage: Internal networks (not routable on Internet)");
        
        match first {
            10 => display_class_a(),
            172 => display_class_b(),
            192 => display_class_c(),
            _ => {}
        }
    } else {
        println!(" Type: Public");
        println!(" Usage: Routable on the Internet");
        
        match class {
            "A" => display_class_a(),
            "B" => display_class_b(),
            "C" => display_class_c(),
            _ => {}
        }
    }
}