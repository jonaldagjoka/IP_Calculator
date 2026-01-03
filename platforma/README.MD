# IP/Subnet Calculators Testing

Ky projekt përmban testime manuale dhe automatike të disa platformave dhe skripteve për llogaritje IP dhe subnet. Qëllimi është të krahasohen rezultatet për NetID, Broadcast Address, Host Count dhe informacione të tjera për IP të ndryshme.

---

## Platforma Online për Testime Manuale (me IP të përdorura në testet automatike)

| Emri | Lloji | Input | Output | Link |
| --- | --- | --- | --- | --- |
| Calculator.net | Web | Network Class (Any) <br> IP Address: 192.168.1.10 <br> Subnet 255.255.255.0 | IP Address: 192.168.1.10 <br> Network Address: 192.168.1.0 <br> Usable Host IP Range: 192.168.1.1 - 192.168.1.254 <br> Broadcast Address: 192.168.1.255 <br> Total Number of Hosts: 256 <br> Number of Usable Hosts: 254 <br> Subnet Mask: 255.255.255.0 <br> CIDR Notation: /24 <br> IP Class: C | [Link](https://www.calculator.net/ip-subnet-calculator.html) |
| IPCalc – Jodie’s Online IP Calculator | Web | Address: 192.168.1.10/24 | Network: 192.168.1.0/24 <br> Broadcast: 192.168.1.255 <br> HostMin: 192.168.1.1 <br> HostMax: 192.168.1.254 <br> Hosts/Net: 254 | [Link](https://jodies.de/ipcalc) |
| SolarWinds Subnet Calculator | Web | IP Address: 192.168.1.10 <br> Subnet Mask: 255.255.255.0 | Network ID: 192.168.1.0 <br> Broadcast: 192.168.1.255 <br> First Host: 192.168.1.1 <br> Last Host: 192.168.1.254 <br> Total Hosts: 254 | [Link](https://www.solarwinds.com/free-tools/advanced-subnet-calculator) |
| Calculator.net | Web | Network Class (Any) <br> IP Address: 10.0.0.1 <br> Subnet 255.255.255.0 | IP Address: 10.0.0.1 <br> Network Address: 10.0.0.0 <br> Usable Host IP Range: 10.0.0.1 - 10.0.0.254 <br> Broadcast Address: 10.0.0.255 <br> Total Number of Hosts: 256 <br> Number of Usable Hosts: 254 <br> Subnet Mask: 255.255.255.0 <br> CIDR Notation: /24 <br> IP Class: A | [Link](https://www.calculator.net/ip-subnet-calculator.html) |
| IPCalc – Jodie’s Online IP Calculator | Web | Address: 10.0.0.1/24 | Network: 10.0.0.0/24 <br> Broadcast: 10.0.0.255 <br> HostMin: 10.0.0.1 <br> HostMax: 10.0.0.254 <br> Hosts/Net: 254 | [Link](https://jodies.de/ipcalc) |
| SolarWinds Subnet Calculator | Web | IP Address: 10.0.0.1 <br> Subnet Mask: 255.255.255.0 | Network ID: 10.0.0.0 <br> Broadcast: 10.0.0.255 <br> First Host: 10.0.0.1 <br> Last Host: 10.0.0.254 <br> Total Hosts: 254 | [Link](https://www.solarwinds.com/free-tools/advanced-subnet-calculator) |

---

##  Përmbledhje e Testimeve Automatike

### **Python Tests**
- **Skriptet:** `main.py`, `test.py`
- **Metodologjia:** `unittest`
- **Raste testimi:**
  - `192.168.1.10/24`
  - `10.0.0.1/24`
- **Rezultat:** 2/2 teste kaluan, NetID, Broadcast, Host Count të sakta.
- **Përfitim:** Shpejtësi dhe mund të integrohet me raportim automatik (mund të ruhet `results.txt`).

---

### **PHP Tests**
- **Skriptet:** `main.php`, `test.php`, `run_tests.php`
- **Metodologjia:** `PHPUnit`
- **Raste testimi:**
  - `192.168.1.10/24`
  - `10.0.0.1/24`
- **Rezultat:** 2/2 teste kaluan, disa deprecations nga PHPUnit 10.  
- **Përfitim:** Lehtë për t’u integruar me projekte PHP dhe mund të ruhet në skedar `results_php.txt`.

---

### **JavaScript Tests**
- **Skriptet:** `main.js`, `IPCalc.test.js`
- **Metodologjia:** `Jest`
- **Raste testimi:**
  - `192.168.1.10/24`
  - `10.0.0.1/24`
- **Rezultat:** 2/2 teste kaluan 
- **Përfitim:** Mund të integrohet lehtësisht në projekte frontend ose NodeJS; testimi është i shpejtë dhe i lehtë për raportim.

---

## Përmbledhje e Metodologjisë

1. **Testime Manuale:**
   - Përdorim të platformave web si Calculator.net, Jodie’s IPCalc, SolarWinds.
   - Input: IP dhe subnet maska të ndryshme.
   - Output: NetID, Broadcast, Host Count, CIDR, IP Class.
   - Krahasim me rezultatet e skripteve.

2. **Testime Automatike:**
   - Python (`unittest`), PHP (`PHPUnit`), JS (`Jest`).
   - Të gjitha rastet testohen në mënyrë automatike dhe rezultatet ruhen në skedarë `results.txt`.

3. **Krahasimi i rezultateve:**
   - Sigurohet që NetID, Broadcast dhe numri i hosteve për çdo IP/subnet të jenë identik midis skripteve dhe platformave online.

---


