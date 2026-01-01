## 🧮  Terms

- Network ID
- Broadcast
- Host Range (first/last)
- Number of hosts
- CIDR → Subnet mask
- Wildcard mask
- IP belong check
- Next/previous network
- Supernet / IP summarization
- VLSM / Supernetting (Variable Length Subnet Mask)
- Binary/Decimal conversion
- IP arithmetic
- DHCP pool calculation
- Host arithmetic
- Maximum number of hosts
- Longest Prefix Match (LPM) & routing optimization
- CIDR aggregation
- Subnet efficiency / wastage
- IPv6
- Class / Range check

---

## 📌 Përmbledhje e shkurtër (ta mbash mend)

👉 IP = kush jam unë

👉 Subnet mask = kush është rrjeti im

👉 Gateway = si dal jashtë rrjetit

---

## 🧠 Koncepte kyçe (IPv4 + Subnetting)

**IP address (IPv4)** = një numër 32-bit që identifikon një pajisje në rrjet.

**Subnet mask** tregon cilët bitë janë **NETWORK** dhe cilët bitë janë **HOST**.

Subnet mask është “harta” që i tregon IP-së kufijtë e territorit të saj.

## Network / Broadcast / Host range

- **Network ID** = adresa e rrjetit (nuk i përket asnjë pajisjeje)
    - `Network = IP AND SubnetMask`
- **Broadcast** = adresa për t’i dërguar paketë të gjitha host-eve në subnet
    - `Broadcast = Network OR (NOT SubnetMask)`
- **Host-et** janë adresat midis:
    - `First host = Network + 1`
    - `Last host  = Broadcast - 1`
- **Hosts** = `2^(host_bits) - 2`
- **Network** dhe **Broadcast** nuk përdoren për host.

## CIDR Notation (/24, /16, ...)

CIDR tregon sa bitë janë network.

| CIDR | Subnet mask |
| --- | --- |
| /8 | 255.0.0.0 |
| /16 | 255.255.0.0 |
| /24 | 255.255.255.0 |
| /30 | 255.255.255.252 |

/n = n bitë 1 nga e majta.

**Subnetting** = ndarja e një rrjeti të madh në rrjete më të vogla.

---

## 🧱 KLASAT IP

| Class | First Octet Range | Default Subnet Mask | Network Bits | Host Bits | Network ID | First Host | Last Host | Broadcast | Max Hosts | Përdorimi |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| A | 1 – 126 | 255.0.0.0 (/8) | 8 | 24 | 10.0.0.0 | 10.0.0.1 | 10.255.255.254 | 10.255.255.255 | 16,777,214 | Rrjete shumë të mëdha |
| B | 128 – 191 | 255.255.0.0 (/16) | 16 | 16 | 172.16.0.0 | 172.16.0.1 | 172.16.255.254 | 172.16.255.255 | 65,534 | Rrjete mesatare, ISP, universitete |
| C | 192 – 223 | 255.255.255.0 (/24) | 24 | 8 | 192.168.1.0 | 192.168.1.1 | 192.168.1.254 | 192.168.1.255 | 254 | Rrjete të vogla, LAN |
| D | 224 – 239 | N/A | N/A | N/A | N/A | N/A | N/A | N/A | N/A | **Multicast** (grup host) |
| E | 240 – 255 | N/A | N/A | N/A | N/A | N/A | N/A | N/A | N/A | Rezervuar / Eksperimentale |

## Adresa speciale IPv4

| Adresa | Kuptimi |
| --- | --- |
| 127.0.0.1 | Loopback |
| 0.0.0.0 | Default |
| 255.255.255.255 | Broadcast global |

---

## 🧾 Llojet e subneteve

| Tip | CIDR |
| --- | --- |
| Point-to-Point | /30, /31 |
| LAN i vogël | /28 |
| LAN standard | /24 |
| ISP backbone | /16, /12 |

**Përmbledhje e artë:** Subnet mask ndan bitët → bitët japin network + host → bitwise operacione bëjnë magjinë.

---

## 🎯 Wildcard mask / Inverse mask

Shumë përdoret në firewall, routing dhe ACL.

- **Wildcard mask = NOT(subnet mask)**
- Subnet mask: 255.255.255.0
- Wildcard: 0.0.0.255

Llogaritja: thjesht inverton bitët e subnet mask.

---

## ✅ Check: IP belongs to subnet

Nëse do të kontrollosh nëse një IP i përket një subnet-i të caktuar:

`if (IP & subnet_mask) == network_ID → belongs`

---

## 🧩 Subnetting & VLSM (Variable Length Subnet Mask)

Përveç ndarjes klasike, mund të bëhet **VLSM**:

- Ndahen rrjete me madhësi të ndryshme në të njëjtin IP block.
- Përdoret kur departamente të ndryshme kanë nevoja të ndryshme.

---

## 📡 DHCP pool (range)

Përdoret për DHCP pools.

- DHCP start IP = first usable host
- DHCP end IP = last usable host

---

## 🔢 Binary / Decimal conversion

Shumë llogaritje përdorin binare për AND/OR.

Shembull:

- 192.168.1.0/24 → 11000000.10101000.00000001.00000000

---

## 🧮 Subnet mask shortening / CIDR optimization

Për planifikim rrjeti:

- gjej minimum subnet mask që mbulon një numër host-esh.

Shembull:

- Duam 50 hosts → 2^6 - 2 = 62 hosts → mask = /26

---

## 🧭 Class / Range check

Identifikon:

- Class A/B/C
- Private/Public
- Special IPs (loopback, multicast, etj.)

---

## ➕ IP arithmetic / Host arithmetic

- IP + x host → gjen IP e caktuar brenda subnet
- IP - x host → gjen host tjetër

Përdoret në DHCP, NAT dhe routing.

---

## 🧩 Supernetting / summarization

Marrim disa rrjete dhe i bashkojmë në një rrjet më të madh për të reduktuar tabelat e routing:

- 192.168.0.0/24 + 192.168.1.0/24 → 192.168.0.0/23

---

## 🧷 VLAN IP allocation

Secila VLAN ka rrjet të veçantë.

Përdoret bitwise subnetting + host counting.

---

## 📣 Broadcast vs Multicast

- Broadcast = çdo host në rrjet merr paketë
- Multicast = vetëm host-et e abonuar marrin paketë

---

## 🌍 IPv6 (opsionale, më komplekse)

IPv6 = 128-bit, por konceptet janë të ngjashme:

- Network prefix
- Host ID
- Subnetting
- Number of hosts

## Llojet e IPv6

| Tip | Shembull |
| --- | --- |
| Unicast | 2001:db8::1 (një host) |
| Multicast | ff00::/8 (për shumë host) |
| Anycast | 2001:db8::1 (rruga më e afërt) |
| Link-local | fe80::/10 (brenda segmentit lokal) |
| Unique local | fc00::/7 (private) |

## Prefix / Subnet

IPv6 përdor prefix notation si IPv4: /64, /48, /56.

Shembull /64 → 64-bit network + 64-bit host.

Nr. hosts = 2^(host_bits)

Për /64 → 2^64 host.

IPv6 zakonisht përdor /64 për LAN dhe nuk ka broadcast (përdor multicast).

---

## 🛣️ Routing & optimizim

- **Longest Prefix Match (LPM)** → router-i zgjedh rrjetin më të saktë për një IP.
- **CIDR aggregation** → kombinon rrjete në një rrjet më të madh për të ulur entries në routing table.
- **Subnet efficiency / wastage** → llogarit sa adresa mbeten “bosh” për të optimizuar përdorimin.

---

## 🧾 Private, Public dhe Special (IPv4)

## 1️⃣ IP Private (RFC 1918)

| Range | Për çfarë përdoret |
| --- | --- |
| 10.0.0.0 – 10.255.255.255 | Rrjete shumë të mëdha (enterprise, cloud) |
| 172.16.0.0 – 172.31.255.255 | Rrjete mesatare |
| 192.168.0.0 – 192.168.255.255 | LAN shtëpi / zyra |

🚫 Nuk routohen në Internet

✅ Përdoren me NAT

## 2️⃣ Loopback

| IP | Kuptimi |
| --- | --- |
| 127.0.0.1 | Vetë kompjuteri |
| 127.0.0.0/8 | I gjithë loopback range |

## 3️⃣ Link-local (APIPA)

| Range | Kur përdoret |
| --- | --- |
| 169.254.0.0 – 169.254.255.255 | Kur DHCP dështon |

📌 Kompjuterët flasin vetëm mes tyre, pa Internet

## 4️⃣ Broadcast

| IP | Kuptimi |
| --- | --- |
| 255.255.255.255 | Broadcast global |
| x.x.x.255 | Broadcast i subnet-it |

## 5️⃣ Network ID

| Shembull | Pse |
| --- | --- |
| 192.168.1.0/24 | Host bits = 0 |

🚫 Nuk caktohet host-it

## 6️⃣ Multicast (Class D)

| Range | Përdorimi |
| --- | --- |
| 224.0.0.0 – 239.255.255.255 | Streaming, routing protocols |

## 7️⃣ Reserved / Experimental

| Range | Status |
| --- | --- |
| 240.0.0.0 – 255.255.255.254 | Eksperimentale |
| 255.255.255.255 | Broadcast |

## 8️⃣ Documentation / TEST

| Range | Pse |
| --- | --- |
| 192.0.2.0/24 | Dokumentacion |
| 198.51.100.0/24 | Shembuj |
| 203.0.113.0/24 | Tutoriale |

---

## 📌 PËRMBLEDHJE QË TA MBAJSH MEND

👉 Private = LAN

👉 Public = Internet

👉 Subnet mask = kufiri i rrjetit

👉 Network & Broadcast nuk janë host

👉 192.168 është private sepse STANDARDI e thotë