const IPCalc = require('./main');

test('Subnet 192.168.1.10/255.255.255.0', () => {
    const calc = new IPCalc('192.168.1.10', '255.255.255.0');
    expect(calc.getNetID()).toBe('192.168.1.0');
    expect(calc.getBroadcast()).toBe('192.168.1.255');
    expect(calc.getHostCount()).toBe(254);
});

test('Subnet 10.0.0.1/255.255.255.0', () => {
    const calc = new IPCalc('10.0.0.1', '255.255.255.0');
    expect(calc.getNetID()).toBe('10.0.0.0');
    expect(calc.getBroadcast()).toBe('10.0.0.255');
    expect(calc.getHostCount()).toBe(254);
});
