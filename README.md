# snmpfetch

## About
snmpfetch is a simple command-line program that uses the SNMP protocol to retrieve system information.  
It sends GET requests to a host where the SNMP daemon is running, parses the data, and prints them.  

[![asciicast](https://asciinema.org/a/LjwGWGZPLLV98vQmmpPPBgczt.png)](https://asciinema.org/a/LjwGWGZPLLV98vQmmpPPBgczt)
---

## Installation
### Ubuntu
1. ```apt-get install snmp snmpd snmp-mibs-downloader```
2. ```sed -i 's/mibs :/# mibs :/g' /etc/snmp/snmp.conf```
3. [Enable MIBs](#how-to-enable-host-resourcesucd-mibs)

---

## How to enable HOST-RESOURCES/UCD MIBs
Add to `/etc/snmp/snmpd.conf`  
```
view systemonly included .1.3.6.1.4.1.2021
view systemonly included .1.3.6.1.2.1.25
```
and restart snmpd with
```
service snmpd restart
```





