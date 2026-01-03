class IPCalc{
    constructor(ip, mask){
        this.ip=ip;
        this.mask=mask;
    }
    maskToCIDR() {
    const parts = this.mask.split('.').map(Number);
    let binary = '';
    parts.forEach(p => {
        binary += p.toString(2).padStart(8, '0');
    });
    return (binary.match(/1/g) || []).length;
}

    getNetID(){
        const ipLong=this.ipToLong(this.ip);
        const maskLong=this.ipToLong(this.mask);
        return this.longToIp(ipLong & maskLong);
    }
    getBroadcast() {
        const netID = this.ipToLong(this.getNetID());
        const maskLong = this.ipToLong(this.mask);
        return this.longToIp(netID | (~maskLong >>> 0));
    }

    getHostCount() {
        const cidr = this.maskToCIDR();
        return Math.pow(2, 32 - cidr) - 2;
    }

    ipToLong(ip) {
        return ip.split('.').reduce((acc, octet) => (acc << 8) + parseInt(octet), 0) >>> 0;
    }

    longToIp(long) {
        return [
            (long >>> 24) & 0xFF,
            (long >>> 16) & 0xFF,
            (long >>> 8) & 0xFF,
            long & 0xFF
        ].join('.');
    }
}

module.exports = IPCalc;
