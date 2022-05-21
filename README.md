# snmpfetch

## About


## Install on Ubuntu
1. ```apt-get install snmp snmpd snmp-mibs-downloader```
2. ```sed -i 's/mibs :/# mibs :/g' /etc/snmp/snmp.conf```
3. Enable CPU and Memory Monitor

## How to enable CPU and Memory Monitoring
Add to ```/etc/snmp/snmpd.conf```<br>
```sh
view systemonly included .1.3.6.1.4.1.2021
```
and restart snmpd with
```sh
service snmpd restart
```



