<?php

class IPCalc {
    private $ip;
    private $mask;

    public function __construct($ip, $mask){
        $this->ip = $ip;
        $this->mask = $mask;
    }

    private function maskToCIDR(){
        $parts = explode('.', $this->mask);
        $binary = '';
        foreach ($parts as $p){
            $binary .= str_pad(decbin($p), 8, '0', STR_PAD_LEFT);
        }
        return substr_count($binary, '1');
    }

    public function getNetID(){
        return long2ip(ip2long($this->ip) & ip2long($this->mask));
    }

    public function getBroadcast(){
        return long2ip(
            ip2long($this->getNetID()) |
            (~ip2long($this->mask) & 0xFFFFFFFF)
        );
    }

    public function getHostCount(){
        $cidr = $this->maskToCIDR();
        return pow(2, 32 - $cidr) - 2;
    }
}
