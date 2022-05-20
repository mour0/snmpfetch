    // https://campus.barracuda.com/product/websecuritygateway/doc/77401430/snmp-oid-s-for-cpu-memory-and-disk-statistics-on-linux/
    // 1.3.6.1.2.1.25.3.3.1.2	hrProcessorLoad (The average, over the last minute, of the percentage of time that this processor was not idle. Implementations may approximate this one minute smoothing period if necessary.)
    // 1.3.6.1.2.1.25.3.6.1.4	hrDiskStorageCapacity
    // 1.3.6.1.2.1.25.2.3.1.3	hrStorageDescr
    // 1.3.6.1.2.1.25.2.3.1.5	hrStorageSize
    // 1.3.6.1.2.1.25.2.3.1.6	hrStorageUsed

    // 1.3.6.1.2.1.25.4.2.1.1	hrSWRunIndex (pid?)
    // 1.3.6.1.2.1.25.4.2.1.2	hrSWRunName (names)
    // 1.3.6.1.2.1.25.6.3.1.2	hrSWInstalledName

    // mem usata dal processo hrSWRunPerfMem	1.3.6.1.2.1.25.5.1.1.2
    
    // &[1,3,6,1,2,1,25,7,5] probs

    // 1.3.6.1.2.1.2.2.1.2	ifDescr (interface desc)
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

    #[clap(short, long, default_value="public")]
    community: String,

}

//const SYS_DESCR: &[i32] = &[1,3,6,1,2,1,1,1,0]; //sysDesc.0
/*
fn print_snmp(str: &str,oid: &[u32],session: &mut SyncSession) 
{
    let mut response = session.get(oid).unwrap();
    if let Some((_, Value::OctetString(sys_descr))) = response.varbinds.next() {
        println!("{}: {}", str,String::from_utf8_lossy(sys_descr));
    }
}
*/

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

const SYS_DESCR: &[u32; 9] = &[1,3,6,1,2,1,1,1,0]; //sysDesc.0
const SYS_UPTIME: &[u32; 9] = &[1,3,6,1,2,1,1,3,0]; //sysUpTime.0 -  time (in hundredths of a second) since the network daemon of the system was last re-initialized
const HR_SYSTEM_UPTIME: &[u32; 10] = &[1,3,6,1,2,1,25,1,1,0]; //hrSystemUptime.0 - measures the amount of time since the host was last initialized
const HR_SYSTEM_PROCESSES: &[u32; 10] = &[1,3,6,1,2,1,25,1,6,0]; //hrSystemProcesses.0 - number of processes
const HR_MEMORY_SIZE: &[u32; 10] = &[1,3,6,1,2,1,25,2,2,0]; //hrMemorySize.0 - RAM contained by the host. (KBytes)
const HR_SW_RUN_PERF_MEM: &[u32; 12] = &[1,3,6,1,2,1,25,5,1,1,2,0]; //hrSWRunPerfMem - The total amount of real system memory allocated to a process.

fn main() {
    /*
    -name 
    ---
    os
    -Uptime
    -Process(?)
    CPU
    GPU
    Memory_tot
    
    */
    let _args = Args::parse();

    let sys_name = &[1,3,6,1,2,1,1,5,0]; //sysName.0
    let sys_contact = &[1,3,6,1,2,1,1,4,0]; //sysName.0
    let hr_device_table = &[1,3,6,1,2,1,25,3,2]; //hrDeviceTable.

    let agent_addr = _args.ip + ":161";
    let community = b"mau22test";
    //let community = b"public";
    let timeout       = Duration::from_secs(2);

    let mut sess = SyncSession::new(agent_addr, community, Some(timeout), 0).unwrap();
    let mut response = sess.get(sys_name).unwrap();
    if let Some((_, Value::OctetString(descr))) = response.varbinds.next() {
        println!("sysName: {}", String::from_utf8_lossy(descr));
        println!("{:-<1$}", "", descr.len() + 9);
    }
    

    
    response = sess.get(SYS_DESCR).unwrap();
    if let Some((_, Value::OctetString(descr))) = response.varbinds.next() {
        println!("sysDescr: {}",String::from_utf8_lossy(descr));
    }

    response = sess.get(SYS_UPTIME).unwrap();
    if let Some((_, Value::Timeticks(descr))) = response.varbinds.next() {
        println!("sysUptime: {} ({})",sec_to_date((descr/100).into()),descr);
    }

    response = sess.get(HR_SYSTEM_UPTIME).unwrap();
    if let Some((_, Value::Timeticks(descr))) = response.varbinds.next() {
        println!("hrSystemUptime: {} ({})",sec_to_date((descr/100).into()),descr);
    }

    let mut process_number = 0;
    response = sess.get(HR_SYSTEM_PROCESSES).unwrap();
    if let Some((_, Value::Unsigned32(descr))) = response.varbinds.next() {
        println!("hrSystemProcesses: {}",descr);
        process_number = descr; 
    }

    let mut memory_size:u32 = 0;
    response = sess.get(HR_MEMORY_SIZE).unwrap();
    if let Some((_, Value::Integer(descr))) = response.varbinds.next() {
        println!("hrMemorySize: {:.2} GB ({} KB)",descr as f64/(1024.0 * 1024.0),descr);
        memory_size = descr as u32;
    }


    let mut temp = HR_SW_RUN_PERF_MEM.clone(); 
    let mut sum: usize =0;
    for n in 0..=process_number {
        response = sess.getnext(&temp).unwrap();
        if let Some((_oid, Value::Integer(sys_descr))) = response.varbinds.next() {
            //println!("{} => {}: {}", n,_oid,sys_descr);
            temp[11] = _oid.to_string().split('.').last().unwrap().parse::<u32>().unwrap();
            sum += sys_descr as usize;
        }
    }
    println!("hrSWRunPerfMem: {} ({:.0}%)",sum,(sum as f32 /memory_size as f32 ) * 100.0);
}

