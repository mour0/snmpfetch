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
//fn print_snmp(str: &str,oid: &[u32],session: &mut SyncSession)
//{
//    let mut response = session.get(oid).unwrap();
//    if let Some((_, Value::OctetString(sys_descr))) = response.varbinds.next() {
//        println!("{}: {}", str,String::from_utf8_lossy(sys_descr));
//    }
//}

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

fn main() {
    let _args = Args::parse();

    let sys_descr = &[1,3,6,1,2,1,1,1,0];
    let sys_name = &[1,3,6,1,2,1,1,5,0]; //sysName.0
    let sys_contact = &[1,3,6,1,2,1,1,4,0]; //sysName.0
    let sys_uptime = &[1,3,6,1,2,1,1,3,0]; //sysUpTime.0
    /*
    TODO
    CPU: .1.3.6.1.4.1.2021.11 (UCD-SNMP-MIB::systemStats)
    Memory: .1.3.6.1.4.1.2021.4 (UCD-SNMP-MIB::memory)
    */

    let agent_addr = "127.0.0.1:161";
    let community = b"mau22test";
    //let community = b"public";
    let timeout       = Duration::from_secs(2);

    let mut sess = SyncSession::new(agent_addr, community, Some(timeout), 0).unwrap();
    let mut response = sess.get(sys_name).unwrap();
    if let Some((_, Value::OctetString(descr))) = response.varbinds.next() {
        println!("name: {}", String::from_utf8_lossy(descr));
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
    //print_snmp("name",sys_name,&mut sess);
    //print_snmp("sysDescr",sys_descr,&mut sess);
}




