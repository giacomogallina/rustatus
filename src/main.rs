extern crate systemstat;
extern crate i3ipc;

use std::thread;
use std::time::Duration;
use systemstat::{System, Platform};
use std::fs::File;
use std::io::prelude::*;
use i3ipc::I3Connection;

// reads the first line of /proc/stat, and creates a vector of integers from 
// the data that it reads
fn read_proc_stat() -> Vec<u64>
{
    let mut proc_stat_file = File::open("/proc/stat")
        .expect("cannot open /proc/stat file");
    
    let mut proc_stat = String::new();
    proc_stat_file.read_to_string(&mut proc_stat)
        .expect("cannot read /proc/stat");
    
    let proc_stat = proc_stat.lines().collect::<Vec<_>>();
    let mut cpu: Vec<u64> = Vec::new();
    
    for i in proc_stat[0].split_whitespace()
    {
        cpu.push(i.parse().unwrap_or(0)); //the first item of proc_stat[0] is "cpu"
    }
    cpu
}

// returns the average cpu usage (in percentage) in the interval between
// old_cpu misuration and new_cpu misuration
fn get_total_cpu_usage(old_cpu: &Vec<u64>, new_cpu: &Vec<u64>) -> u64
{
    let mut old_total: u64 = 0;
    let mut new_total: u64 = 0;

    for i in old_cpu.iter()
    {
        old_total += *i;
    }

    for i in new_cpu.iter()
    {
        new_total += *i;
    }

    let old_idle = old_cpu[4];
    let new_idle = new_cpu[4];
    
    let total = new_total - old_total;
    let idle = new_idle - old_idle;

    ((total as f64 - idle as f64)/total as f64 * 100.0).round() as u64
    
}


fn main() {
    let sys = System::new();
    let mut i3 = I3Connection::connect().unwrap();

    
    match sys.memory()
    {
        Ok(mem) => println!("RAM usage: {}", mem.total - mem.free),
        Err(error) => println!("Memory: Error: {}", error)
    }
    
    let old_cpu = read_proc_stat();
    thread::sleep(Duration::from_millis(1000));
    let new_cpu = read_proc_stat();

    println!("CPU usage: {}%", get_total_cpu_usage(&old_cpu, &new_cpu));
    

    let workspaces = i3.get_workspaces().unwrap().workspaces;
    for i in workspaces.iter()
    {
        println!("{}", i.num);
    }

}
