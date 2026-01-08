# IP Calculator

> DETYRA NR.2

```
┌─────────────────────────────────────────────────────┐
│                                                     │
│                                                     │
│     ▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄                                │
│       ▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄                              │
│         ▄                                           │
│         ▄    ▄▄▄▄▄                                  │
│       ▄▄▄▄▄  ▄                                      │
│                                                     │
│          I P   C A L C U L A T O R                  │
│                                                     │
│        WHEN THE PROF ASKS 'EASY QUESTION'           │
│             BUT YOUR BRAIN SAYS 404                 │
│                                                     │
└─────────────────────────────────────────────────────┘
```

## Përmbajtja

- [Rreth Projektit](#rreth-projektit)
- [Veçoritë](#veçoritë)
- [Instalimi](#instalimi)
- [Përdorimi](#përdorimi)
- [Operacionet IPv4](#operacionet-ipv4)
- [Operacionet IPv6](#operacionet-ipv6)
- [Struktura e Projektit](#struktura-e-projektit)
- [Testimet](#testimet)
- [Teste Manuale IPv4](#teste-manuale-ipv4)
- [Teste Manuale IPv6](#teste-manuale-ipv6)

---

## Rreth Projektit

**IP Calculator** është një mjet CLI i ndërtuar në Rust bazuar mbi kërkesat e përmendura në DETYRE NR.2 . 

---

## Veçoritë

### Aftësitë IPv4
-  Llogaritja e Network ID dhe Broadcast Address
-  Përcaktimi i DHCP Range
-  Llogartija e IP range dhe numërimi i host-eve
-  Llogaritja e Subnet Mask nga kërkesat e host/subnet
-  Konvertimet Binary ↔ Decimal
-  Konvertimet CIDR ↔ Subnet Mask
-  Llogaritja e Wildcard Mask
-  VLSM (Variable Length Subnet Masking)
-  Supernetting / Route Aggregation
-  Identifikimi i klasës IP (A, B, C, D, E)
-  Detektimi i IP-ve speciale (Private, Loopback, Multicast, etj.)

### Aftësitë IPv6
-  Llogaritja e Network Prefix
-  Përcaktimi i IP Range
-  Llogaritja e numrit të host-eve (2^(128-prefix))
-  Zgjerimi i adresës IPv6 (forma e plotë)
-  Kompresimi i adresës IPv6 (forma e shkurtuar)
-  Konvertimet Hexadecimal ↔ Decimal
-  Ndarje në Subnete
-  Identifikimi i tipit të adresës (Multicast, Link-Local, Global Unicast, etj.)
-  Gjenerimi i adresës Link-Local
-  Gjenerimi i adresës EUI-64 nga MAC
-  Subnetting dhe Supernetting
-  Llogaritja e DHCP Range

---

## Instalimi

### Parakushtet
- **Rust** (1.70+ i rekomanduar)
- **Cargo** (vjen me Rust)

Kontrollo instalimin e Rust:
```bash
rustc --version
cargo --version
```

### Instalo Rust
Nëse nuk e ke Rust të instaluar:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Klono dhe Ndërto
```bash
# Klono repository-n
git clone https://github.com/jonaldagjoka/IP_Calculator.git
cd ipcalculator

# Ndërto në modalitetin release
cargo build --release

# Ekzekuto aplikacionin
cargo run --release
```

---

## Përdorimi

### Fillimi i Shpejtë
```bash
# Ekzekuto në modalitetin e zhvillimit
cargo run

# Ekzekuto me optimizime
cargo run --release

# Kontrollo kodin pa ndërtuar
cargo check

# Ekzekuto testet
cargo test
```

### Menuja krysrore


```
  ┌────────────────────────────────────────────────────┐
  │               Menu Kryesore                        │
  │           (choose your weapon)                     |
  └────────────────────────────────────────────────────┘

  (a) Llogaritje mbi IPv4
  (b) Llogaritje mbi IPv6
  (q) Quit
```
### Menuja IPv4


```
┌────────────────────────────────────────────────────┐
│               IPv4 Calculator                      │
│        (the classic that still works)              │
└────────────────────────────────────────────────────┘
  (*) Kontrollo Klasat dhe IP speciale
  (a) Network ID
  (b) Broadcast Address
  (c) DHCP Range
  (d) Kontrollo nëse IP i përket një rrjeti
  (e) Llogarit Subnet Mask nga Numri i Hosteve
  (f) Llogarit Subnet Mask nga Numri i Subneteve
  (g) Decimal -> Binary
  (h) Binary -> Decimal
  (i) CIDR -> Subnet Mask
  (j) Subnet Mask -> CIDR
  (k) Wildcard Mask
  (l) VLSM
  (m) Supernetting
  (q) Kthehu në Menu Kryesore
  ────────────────────────────────────────────────────
```
### Menuja IPv6


```
┌────────────────────────────────────────────────────┐
│               IPv6 Calculator                      │
│        (welcome to the future of networking)       │
└────────────────────────────────────────────────────┘
 (a) Llogarit Network Prefix (NetID)
 (b) Llogarit IP Range
 (c) IPv6 Expansion (shkruaj formën e plotë)
 (d) IPv6 Compression (shkurto adresën)
 (e) Hex to Decimal Conversion
 (f) Decimal to Hex Conversion
 (g) IPv6 Address Type Identifier
 (h) Generate EUI-64 Address
 (i) Subnetting
 (j) Supernetting
 (k) DHCP Range Calculation
 (q) Kthehu në Menu Kryesore
  ────────────────────────────────────────────────────
```
---

## Operacionet IPv4

### Operacionet e Disponueshme

| Operacioni | Përshkrimi | Shembull Input |
|-----------|-------------|---------------|
| **Klasat dhe IP speciale** |Kontrollo Klasat dhe IP speciale | `192.168.1.10/24`
| **Network ID** | Llogarit adresën e rrjetit | `192.168.1.10/24` |
| **Broadcast** | Gjen adresën broadcast | `10.0.0.0/8` |
| **DHCP Range** | Përcakton intervalet e host-eve | `172.16.0.0/16` |
| **Anëtarësia IP** | Kontrollon nëse IP i përket rrjetit | IP + Rrjet |
| **Subnet nga Host-et** | Llogarit maskën për N host | `100` host |
| **Subnet nga Subnet-et** | Llogarit maskën për N subnet | `4` subnet |
| **Decimal ↔ Binary** | Konverton sistemet numerike | `255` ose `11111111` |
| **Binary ↔ Decimal** | Konverton sistemet numerike | `11111111` ose `255` |
| **CIDR ↔ Mask** | Konverton formatet e notacionit | `/24` ose `255.255.255.0` |
| **Mask ↔ CIDR** | Konverton formatet e notacionit | `255.255.255.0` ose `/24` |
| **Wildcard Mask** | Llogarit wildcard | `255.255.255.0` |
| **VLSM** | Subnetting me gjatësi variabile | Kërkesa të shumta |
| **Supernetting** | Agreguesi i rrjeteve | Rrjete të shumta |


---

## Operacionet IPv6

### Operacionet e Disponueshme

| Operacioni | Përshkrimi | Shembull Input |
|-----------|-------------|---------------|
| **Network Prefix** | Llogarit network prefix | `2001:db8::1/64` |
| **IP Range** | Adresa e parë dhe e fundit | `fe80::/10` |
| **Zgjerimi** | Forma e plotë e adresës | `2001:db8::1` |
| **Kompresimi** | Forma e shkurtuar | `2001:0db8:0000:0000::0001` |
| **Hex ↔ Decimal** | Konverton segmentet | `2001` ose `8193` |
| **Decimal ↔ Hex** | Konverton segmentet | `8193` ose `2001` |
| **Tipi i Adresës** | Identifikon tipin | Çdo IPv6 |
| **Gjenerimi EUI-64** | Gjenero nga MAC | MAC + prefix |
| **Subnetting** | Krijo subnete | Rrjet + numër |
| **Supernetting** | Agreguesi i rrugëve | Dy rrjete |
| **DHCP Range Calculation** |Llogairt numrin e hosteve |  `2001:db8:2000::/64 `


---

## Struktura e Projektit

```
ipcalculator/
├── Cargo.toml              # Konfigurimi i projektit
├── Cargo.lock              # Skedari i varësive
├── README.md               # Ky skedar
├── src/
│   ├── main.rs             # Entry point i aplikacionit
│   ├── lib.rs              # Rrënja e librarisë
│   ├── menus/              # Modulet e menysë CLI
│   │   ├── mod.rs
│   │   ├── main_menu.rs    # Trajtimi i menysë kryesore
│   │   ├── ipv4_menu.rs    # Menya e operacioneve IPv4
│   │   └── ipv6_menu.rs    # Menya e operacioneve IPv6
│   ├── models/             # Strukturat e të dhënave
│   │   ├── mod.rs
│   │   ├── ipv4_address.rs # Modeli i adresës IPv4
│   │   └── ipv6_address.rs # Modeli i adresës IPv6
│   └── utils/              # Funksionet ndihmëse
│       ├── mod.rs
│       ├── conversions.rs  # Konvertimet e formateve
│       ├── ipv4.rs         # Llogaritjet IPv4
│       ├── ipv6.rs         # Llogaritjet IPv6
│       └── ip_classes.rs   # Klasifikimi i IP-ve
└── tests/                  # Testet e integrimit
    ├── ipv4_tests.rs
    └── ipv6_tests.rs
```

---

## Testimet

### Ekzekuto Të Gjitha Testet
```bash
cargo test
```

### Ekzekuto Test Suite Specifik
```bash
# Vetëm testet IPv4
cargo test ipv4

# Vetëm testet IPv6
cargo test ipv6

# Output me detaje
cargo test -- --nocapture
```

### Mbulimi i Testeve
Suite-i i testeve përfshin:
-  Parsing dhe validim të adresave
-  Llogaritje të rrjetit dhe broadcast
-  Algoritme të numërimit të host-eve
-  Derivim të subnet mask
-  Konvertime të notacionit CIDR
-  Llogaritje VLSM
-  Logjikë supernetting
-  Zgjerim/kompresim IPv6
-  Klasifikim të tipit të adresës

---

## Teste Manuale IPv4
```bash
### Kontrollo Klasat dhe IP speciale
Vendos nje adrese IPv4 (x.x.x.x): 192.168.2.1

╔═══════════════════════════════════════════════════════════╗    
║       Classification for 192.168.2.1                      ║       
╚═══════════════════════════════════════════════════════════╝    

 Class: C
 Default Subnet Mask: 255.255.255.0 (/24)
 Type: Private (RFC 1918)
 Usage: Internal networks (not routable on Internet)


### Network ID
Vendos IPv4 (format x.x.x.x/24): 192.168.2.1/24
Network: 192.168.2.0/24

### Broadcast Address
Vendos IPv4 (format x.x.x.x/24): 192.168.2.1/24
Broadcast: 192.168.2.255

### DHCP Range
Vendos rrjetin (x.x.x.x/24): 192.168.2.1/24
DHCP Pool:
  Hosti i pare: 192.168.2.1
  Hosti i fundit: 192.168.2.254
  Hoste Totale: 254

### Kontrollo nëse IP i përket një rrjeti
Vendos rrjetin (x.x.x.x/24): 192.168.2.1/24
Vendos IP-në për kontroll (x.x.x.x): 192.168.2.56/24
✓ IP 192.168.2.56 i perket rrjetit 192.168.2.0/24

### Llogarit Subnet Mask nga Numri i Hosteve
Vendos numrin e hosteve: 500
Prefix: /23  Mask: 255.255.254.0

### Llogarit Subnet Mask nga Numri i Subneteve
Vendos rrjetin kryesor (format x.x.x.x/24): 192.168.2.56/24
Vendos numrin e subnetave të kërkuara: 120
New prefix: /31  Mask: 255.255.255.254

### Decimal->Binary
Vendos IPv4 (x.x.x.x or x.x.x.x/xx): 192.168.2.56/24
Binary: 11000000.10101000.00000010.00111000

### Binay->Decimal
Vendos IPv4 në binar (aaaaaaaa.bbbbbbbb.cccccccc.dddddddd): 11000000.10101000.00000010.00111000
Decimal IPv4: 192.168.2.56

### CIDR -> Subnet Mask
Vendos prefiksin CIDR (0-32): 25
Subnet Mask: 255.255.255.128

### Subnet Mask -> CIDR
Vendos subnet mask (x.x.x.x): 255.255.255.240
CIDR: /28

### Wildcard Mask
Vendos CIDR (x.x.x.x/24) ose Subnet Mask (x.x.x.x): 255.255.255.240
Wildcard Mask: 0.0.0.15

### VLSM
VLSM: Variable Length Subnet Mask
Vendos rrjetin kryesor (x.x.x.x/24): 192.168.2.56/24
Vendos numrin e subneteve të kërkuara: 7              
Subnet 1: 192.168.2.0/27
Subnet 2: 192.168.2.32/27
Subnet 3: 192.168.2.64/27
Subnet 4: 192.168.2.96/27
Subnet 5: 192.168.2.128/27
Subnet 6: 192.168.2.160/27
Subnet 7: 192.168.2.192/27

### Supernetting
Supernetting
Kushtet:
- Rrjetet të jenë të njëpasnjëshme
- Numri i rrjeteve fuqi e dyshit
- Të kenë të njëjtin prefiks
______________________________________
Supernetting me x rrjete
Vendos numrin e rrjeteve: 4
Vendos rrjetin 1 (x.x.x.x/24): 192.168.2.0/24
Vendos rrjetin 2 (x.x.x.x/24): 192.168.3.0/24
Vendos rrjetin 3 (x.x.x.x/24): 192.168.4.0/24
Vendos rrjetin 4 (x.x.x.x/24): 192.168.5.0/24
 Supernet: 192.168.0.0/22
```
## Teste Manuale IPv6
```bash
### Llogarit Network Prefix (NetID)
Vendos IPv6 (format xxxx:...:/64): 2001:db8::10/64
Network Prefix: 2001:db8::/64

### Llogarit IP Range
Vendos IPv6 (format xxxx:...:/64): 2001:db8::10/64
First Address: 2001:db8::
Last Address: 2001:db8::ffff:ffff:ffff:ffff

### IPv6 Expansion (shkruaj formën e plotë)
Vendos IPv6 në formë të shkurtuar (p.sh. 2001:db8::1/64): 2001:db8::10/64
Forma e plotë: 2001:0db8:0000:0000:0000:0000:0000:0010/64

### IPv6 Compression (shkurto adresën)
Vendos IPv6 në formë të plotë (p.sh. 2001:0db8:0000:0000:0000:0000:0000:0001): 2001:0db8:0000:0000:0000:0000:0000:0010   
Forma e shkurtuar: 2001:db8::10/128

### Hex to Decimal Conversion
Vendos një segment IPv6 në hex (p.sh. 2001): abc
Decimal: 2748

### Decimal to Hex Conversion
Vendos një vlerë decimal (0-65535): 20
Hexadecimal: 14

### IPv6 Address Type Identifier
Vendos IPv6 address: 2001:db8::10/64
Address Type: Global Unicast

### Generate EUI-64 Address
Vendos MAC address (format: xx:xx:xx:xx:xx:xx): 00:1a:2b:3c:4d:5e
Vendos network prefix (p.sh. 2001:db8::/64): 2001:db8::/64
EUI-64 Address: 2001:db8::21a:2bff:fe3c:4d5e/64

### Subnetting
Vendos rrjetin (xxxx:...:/64): 2001:db8::/64
Vendos numrin e subneteve: 4 
Subnete (/66):
  2001:db8::/66
  2001:db8:0:0:4000::/66
  2001:db8:0:0:8000::/66
  2001:db8:0:0:c000::/66

### Supernetting
Kushtet:
- Rrjetet të jenë të njëpasnjëshme
- Numri i rrjeteve fuqi e dyshit
- Të kenë të njëjtin prefiks
______________________________________
Supernetting me x rrjete
Vendos numrin e rrjeteve: 4
Vendos rrjetin 1 (xxxx::/64): 2001:db8:0:0::/64
Vendos rrjetin 2 (xxxx::/64): 2001:db8:0:1::/64
Vendos rrjetin 3 (xxxx::/64): 2001:db8:0:2::/64
Vendos rrjetin 4 (xxxx::/64): 2001:db8:0:3::/64
 Supernet IPv6: 2001:db8::/62

### DHCP Range Calculation
Vendos rrjetin (xxxx:...:/64): 2001:db8:2000::/64
IPv6 Prefix Range:
  First: 2001:db8:2000::
  Last: 2001:db8:2000:0:ffff:ffff:ffff:ffff
  Total addresses: 2^64
```

---

## Autori

**Jonalda Gjoka**

---

## Referenca

- RFC 1918 - Address Allocation for Private Internets
- RFC 4291 - IPv6 Addressing Architecture
- RFC 3513 - IPv6 Addressing Architecture (zëvendësuar nga 4291)

---

**Ndërtuar me Rust 🦀**