import ipaddress

class IPCalc:
    def __init__(self, ip, mask):
        self.network = ipaddress.IPv4Network(f"{ip}/{mask}", strict=False)
    
    def getNetID(self):
        return str(self.network.network_address)
    
    def getBroadcast(self):
        return str(self.network.broadcast_address)
    
    def getHostCount(self):
        return self.network.num_addresses - 2 
