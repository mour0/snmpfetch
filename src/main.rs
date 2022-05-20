#![allow(unused_variables)]
#![allow(unused_mut)]
use clap::Parser;

use std::time::Duration;
use snmp::{SyncSession, Value};

#[derive(Parser)]
#[clap(name="snmpfetch",
    author, 
    version="1.0", 
    about="SNMP", 
    long_about = None)]
struct Args {

    ip: String,

}

//const SYS_DESCR: &[i32] = &[1,3,6,1,2,1,1,1,0]; //sysDesc.0
fn print_snmp(str: &str,oid: &[u32],session: &mut SyncSession) 
{
    let mut response = session.get(oid).unwrap();
    if let Some((_, Value::OctetString(sys_descr))) = response.varbinds.next() {
        println!("{}: {}", str,String::from_utf8_lossy(sys_descr));
    }
}

// convert seconds to years, days, hours, minutes, seconds
#[allow(unused_assignments)]
fn sec_to_date(mut secs: u64) -> String {
    let mut years = 0;
    let mut days = 0;
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;
    let mut result = String::new();

    if secs >= 31536000 {
        years = secs / 31536000;
        secs = secs % 31536000;
        result.push_str(&format!("{}y ", years));
    }
    if secs >= 86400 {
        days = secs / 86400;
        secs = secs % 86400;
        result.push_str(&format!("{}d ", days));
    }
    if secs >= 3600 {
        hours = secs / 3600;
        secs = secs % 3600;
        result.push_str(&format!("{}h ", hours));
    }
    if secs >= 60 {
        minutes = secs / 60;
        secs = secs % 60;
        result.push_str(&format!("{}m ", minutes));
    }
    seconds = secs;
    result.push_str(&format!("{}s", seconds));

    result
}

/*
fn main() {
    /*
    -name 
    ---
    os
    kernel(?)
    -Uptime
    -Process(?)
    CPU
    GPU
    Memory_tot
    
    */
    let _args = Args::parse();

    let sys_descr = &[1,3,6,1,2,1,1,1,0];
    let sys_name = &[1,3,6,1,2,1,1,5,0]; //sysName.0
    let sys_contact = &[1,3,6,1,2,1,1,4,0]; //sysName.0
    let sys_uptime = &[1,3,6,1,2,1,1,3,0]; //sysUpTime.0
    let hr_device_table = &[1,3,6,1,2,1,25,3,2]; //hrDeviceTable.
    let hr_system_uptime = &[1,3,6,1,2,1,25,1,1,0]; //hrSystemUptime.0
    let hr_system_processes = &[1,3,6,1,2,1,25,1,6,0]; //hrSystemProcesses.0
    /*
    TODO
    CPU: .1.3.6.1.4.1.2021.11 (UCD-SNMP-MIB::systemStats)
    Memory: .1.3.6.1.4.1.2021.4 (UCD-SNMP-MIB::memory)
    */
    let agent_addr = _args.ip + ":161";
    let community = b"mau22test";
    //let community = b"public";
    let timeout       = Duration::from_secs(2);

    let mut sess = SyncSession::new(agent_addr, community, Some(timeout), 0).unwrap();
    let mut response = sess.get(sys_name).unwrap();
    if let Some((_, Value::OctetString(descr))) = response.varbinds.next() {
        println!("sysName: {}", String::from_utf8_lossy(descr));
    }
    

    println!("---");
    
    response = sess.get(sys_descr).unwrap();
    if let Some((_, Value::OctetString(descr))) = response.varbinds.next() {
        println!("sysDescr: {}",String::from_utf8_lossy(descr));
    }

    response = sess.get(sys_uptime).unwrap();
    if let Some((_, Value::Timeticks(descr))) = response.varbinds.next() {
        println!("sysUptime: {} ({})",sec_to_date((descr/100).into()),descr);
    }

    response = sess.get(hr_system_processes).unwrap();
    if let Some((_, Value::Unsigned32(descr))) = response.varbinds.next() {
        println!("hrSystemProcesses: {:?}",descr);
    }
    //response = sess.get(hr_system_uptime).unwrap();
    //if let Some((_, Value::Timeticks(descr))) = response.varbinds.next() {
    //    println!("sysUptime2:  ({})",descr);
    //}

    //response = sess.get(hr_device_table).unwrap();
    //if let Some((_, descr)) = response.varbinds.next() {
    //    println!("hrDeviceTable: {:?}",descr);
    //}
    //else {
    //    println!("ERROR!");
    //}
    //print_snmp("name",sys_name,&mut sess);
    //print_snmp("sysDescr",sys_descr,&mut sess);
}

*/

fn main()
{
    // 1.3.6.1.2.1.25.2.1.4
    //let system_oid      = &[1,3,6,1,2,1,25,3,2,1,3];
    // 1.3.6.1.2.1.25.3.3.1.2	hrProcessorLoad
    // 1.3.6.1.2.1.25.3.6.1.4	hrDiskStorageCapacity
    // 1.3.6.1.2.1.25.2.2.0	hrMemorySize
    // 1.3.6.1.2.1.25.2.3.1.3	hrStorageDescr
    // 1.3.6.1.2.1.25.2.3.1.5	hrStorageSize
    // 1.3.6.1.2.1.25.2.3.1.6	hrStorageUsed

    // 1.3.6.1.2.1.25.4.2.1.1	hrSWRunIndex (pid?)
    // 1.3.6.1.2.1.25.4.2.1.2	hrSWRunName (names)
    // 1.3.6.1.2.1.25.6.3.1.2	hrSWInstalledName
    
    // &[1,3,6,1,2,1,25,7,5] probs

    // 1.3.6.1.2.1.2.2.1.2	ifDescr (interface desc)
    let system_oid      = &[1,3,6,1,2,1,2,2,1,2];

let agent_addr      = "127.0.0.1:161";
let community       = b"mau22test";
let timeout         = Duration::from_secs(2);
let non_repeaters   = 0;
let max_repetitions = 7; // number of items in "system" OID

let mut sess = SyncSession::new(agent_addr, community, Some(timeout), 0).unwrap();
let response = sess.getbulk(&[system_oid], non_repeaters, max_repetitions).unwrap();

for (name, val) in response.varbinds {
    println!("{} => {:?}", name, val);
}

}
