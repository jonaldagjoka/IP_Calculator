import unittest
from main import IPCalc

class TestIPCalc(unittest.TestCase):
    def test_getNetID(self):
        calc = IPCalc('192.168.1.10','255.255.255.0')
        self.assertEqual(calc.getNetID(),'192.168.1.0')
        self.assertEqual(calc.getBroadcast(),'192.168.1.255')
        self.assertEqual(calc.getHostCount(),254)

    def test_subnet_2(self):
        calc = IPCalc('10.0.0.1', '255.255.255.0')
        self.assertEqual(calc.getNetID(), '10.0.0.0')
        self.assertEqual(calc.getBroadcast(),'10.0.0.255' )
        self.assertEqual(calc.getHostCount(), 254)

if __name__ == '__main__':
   with open("results.txt", "w") as f:
        runner = unittest.TextTestRunner(stream=f, verbosity=2)  # stream=f dërgon output në file
        unittest.main(testRunner=runner, exit=False)
        print("Test results written to results.txt")
              
        