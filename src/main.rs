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
    
    // 1.3.6.1.2.1.2.2.1.2	ifDescr (interface desc)
#![allow(unused_variables)]
#![allow(unused_mut)]
use clap::Parser;
use std::{time::Duration, fmt::Display};
use snmp::{SyncSession, Value};

#[derive(Parser)]
#[clap(name="snmpfetch",
    //author, 
    version="1.0", 
    about="SNMP", 
    long_about = None)]
struct Args {

    /// IP address
    ip: String,

    /// SNMP community
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
const SYS_NAME: &[u32; 9] = &[1,3,6,1,2,1,1,5,0]; //sysName.0
const SYS_UPTIME: &[u32; 9] = &[1,3,6,1,2,1,1,3,0]; //sysUpTime.0 -  time (in hundredths of a second) since the network daemon of the system was last re-initialized
const HR_SYSTEM_UPTIME: &[u32; 10] = &[1,3,6,1,2,1,25,1,1,0]; //hrSystemUptime.0 - measures the amount of time since the host was last initialized
const HR_SYSTEM_PROCESSES: &[u32; 10] = &[1,3,6,1,2,1,25,1,6,0]; //hrSystemProcesses.0 - number of processes
// Float wrapped
const LA_LOAD: &[u32;11] = &[1,3,6,1,4,1,2021,10,1,5,0];
//const LA_LOAD_1: &[u32;11] = &[1,3,6,1,4,1,2021,10,1,5,0]; //laLoad.1 - The load average for the last minute.
//const LA_LOAD_2: &[u32;11] = &[1,3,6,1,4,1,2021,10,1,5,2]; //laLoad.2 - The load average for the last 5 minutes.
//const LA_LOAD_3: &[u32;11] = &[1,3,6,1,4,1,2021,10,1,5,3]; //laLoad.3- The load average for the last 15 minutes.

const SS_CPU_NUM_CPUS: &[u32;10] = &[1,3,6,1,4,1,2021,11,67,0]; //ssCpu.0 - The number of CPUs in the system.
const SS_CPU_RAW: &[u32; 9] = &[1,3,6,1,4,1,2021,11,49]; 
//const SS_CPU_RAW_USER: &[u32; 10] = &[1,3,6,1,4,1,2021,11,50,0]; //The percentage of CPU time spent processing user-level code
//const SS_CPU_RAW_NICE: &[u32; 10] = &[1,3,6,1,4,1,2021,11,51,0]; //The percentage of CPU time spent processing low-priority code 
//const SS_CPU_RAW_SYSTEM: &[u32; 10] = &[1,3,6,1,4,1,2021,11,52,0]; //The percentage of CPU time spent processing sys-level code
//const SS_CPU_RAW_IDLE: &[u32; 10] = &[1,3,6,1,4,1,2021,11,53,0]; //Idle

const MEM_TOTAL_REAL: &[u32;10] = &[1,3,6,1,4,1,2021,4,5,0]; //memTotalReal.0 - Total RAM (KBytes)
const MEM_AVAIL_REAL: &[u32;10] = &[1,3,6,1,4,1,2021,4,6,0]; //memAvailReal.0 - Memory currently unused(KBytes)
const MEM_TOTAL_FREE: &[u32;10] = &[1,3,6,1,4,1,2021,4,11,0]; //memTotalFree.0 - Total amount of memory free or available for use on this host. (KBytes)


//const SYS_CONTACT: [u32; 9]= &[1,3,6,1,2,1,1,4,0]; //sysName.0
//const HR_DEVICE_TABLE: &[u32; 9] = &[1,3,6,1,2,1,25,3,2]; //hrDeviceTable - set of services that this entity may potentially offer (sum)

//const HR_MEMORY_SIZE: &[u32; 10] = &[1,3,6,1,2,1,25,2,2,0]; //hrMemorySize.0 - RAM contained by the host. (KBytes)
//const HR_SW_RUN_PERF_MEM: &[u32; 12] = &[1,3,6,1,2,1,25,5,1,1,2,0]; //hrSWRunPerfMem - The total amount of real system memory allocated to a process.
fn main() {
    let args = Args::parse();
    let agent_addr = args.ip + ":161";
    let community = args.community.as_bytes();

    let timeout       = Duration::from_secs(2);

    let mut sess = SyncSession::new(agent_addr, community, Some(timeout), 0).unwrap();
    let mut response = match sess.get(SYS_NAME) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("The IP or community string is incorrect");
            return;
        }
    };
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

    response = sess.get(HR_SYSTEM_PROCESSES).unwrap();
    if let Some((_, Value::Unsigned32(descr))) = response.varbinds.next() {
        println!("hrSystemProcesses: {}",descr);
    }

    response = sess.get(SS_CPU_NUM_CPUS).unwrap();
    if let Some((_, Value::Integer(descr))) = response.varbinds.next() {
        println!("ssNumCPUs: {}",descr);
    }

    response = sess.getbulk(&[SS_CPU_RAW],0,4).unwrap();
    let mut cpu_usage = vec![0;4]; 
    let mut sum_cpu = 0;
    for i in 0..4 {
        if let Some((_oid, Value::Counter32(descr))) = response.varbinds.next() {
            cpu_usage[i] = descr;
            sum_cpu += descr;
            println!("|-> {}: {}",_oid,descr);
        }
    }
    if sum_cpu != 0
    {
        println!("cpuUsage:");
        println!("|-> User: {:.2}%",cpu_usage[0] as f32/sum_cpu as f32*100.0);
        println!("|-> Nice: {:.2}%",cpu_usage[1] as f32/sum_cpu as f32*100.0);
        println!("|-> System: {:.2}%",cpu_usage[2] as f32/sum_cpu as f32*100.0);
        println!("|-> Idle: {:.2}%",cpu_usage[3] as f32/sum_cpu as f32*100.0);

    }

    let mut memory_size = 0;
    response = sess.get(MEM_TOTAL_REAL).unwrap();
    if let Some((_, Value::Integer(descr))) = response.varbinds.next() {
        println!("memTotal: {:.2} GB ({} KB)",descr as f64/(1024.0 * 1024.0),descr);
        memory_size = descr;
    }


    response = sess.get(MEM_AVAIL_REAL).unwrap();
    if let Some((_, Value::Integer(descr))) = response.varbinds.next() {
        let mem_used = memory_size - descr;
        println!("memUsed: {:.2} GB ({} KB) ({:.0}%)",mem_used as f64/(1024.0 * 1024.0),mem_used,(mem_used as f32/memory_size as f32)*100.0);
    }

    response = sess.getbulk(&[LA_LOAD],0,3).unwrap();
    println!("Loads:");
    for n in [1,5,15]
    {
        if let Some((_, Value::Integer(descr))) = response.varbinds.next() {
            println!("|-> {}m: {}",n,descr);
        }
    }

}

