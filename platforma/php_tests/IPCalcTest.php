<?php
use PHPUnit\Framework\TestCase;

require_once __DIR__ . '/main.php';

class IPCalcTest extends TestCase {

    public function testSubnet1() {
        $calc = new IPCalc('192.168.1.10','255.255.255.0');
        $this->assertEquals('192.168.1.0', $calc->getNetID());
        $this->assertEquals('192.168.1.255', $calc->getBroadcast());
        $this->assertEquals(254, $calc->getHostCount());
    }

    public function testSubnet2() {
        $calc = new IPCalc('10.0.0.1','255.255.255.0');
        $this->assertEquals('10.0.0.0', $calc->getNetID());
        $this->assertEquals('10.0.0.255', $calc->getBroadcast());
        $this->assertEquals(254, $calc->getHostCount());
    }
}
