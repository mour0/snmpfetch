# snmpfetch

## About
snmpfetch is a simple command-line program that uses the SNMP protocol to retrieve system information.  
It sends GET requests to a host where the SNMP daemon is running, parses the data, and prints them.

**TODO asciinema**
---

## Installation
### Ubuntu
1. ```apt-get install snmp snmpd snmp-mibs-downloader```
2. ```sed -i 's/mibs :/# mibs :/g' /etc/snmp/snmp.conf```
3. [Enable UCD MIBs](#how-to-enable-ucd-mibs)

---

## How to enable UCD MIBs
Add to `/etc/snmp/snmpd.conf`  
```
view systemonly included .1.3.6.1.4.1.2021
```
and restart snmpd with
```
service snmpd restart
```





