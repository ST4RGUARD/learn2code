#! /usr/bin/python

import sys
from scapy.all import *
# help

print './script ip port'

dstip = sys.argv[1]
src = RandShort()
dstport=sys.argv[2]
print "args - " , sys.argv
res = sr1(IP(dst=dstip)/TCP(dport=int(dstport),flags="FPU"),timeout=10)

# res.show() - need to catch for NoneType
# port open - no resp
if (str(type(res))=="<type 'NoneType'>"):
    print "Open"
# rst pkt port is closed
elif(res.haslayer(TCP)):
    if(res.getlayer(TCP).flags == 0x14):
        print "Closed"
# icmp filtered maybe open or closed
elif(res.haslayer(ICMP)):
    if(int(res.getlayer(ICMP).type)==3 and int(res.getlayer(ICMP).code) in [1,2,3,9,10,13]):
        print "Filtered"
