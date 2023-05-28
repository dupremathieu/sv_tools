extern crate pcap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn subscribe(interface: &str, output_file: &str, running: Arc<AtomicBool>) {
    let mut cap = pcap::Capture::from_device(interface) // use the provided interface
        .expect("Error opening device")
        .promisc(true)
        .snaplen(5000)  // Configure the capture length
        .tstamp_type(pcap::TimestampType::AdapterUnsynced) // Enable hardware timestamping
        .open()
        .expect("Error opening pcap");

    cap.filter("ether proto 0x88ba", true).expect("Error setting filter"); // Add a filter for IEC 61850 Sampled Values

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_file)
        .expect("Error opening file");


    loop {
        if running.load(Ordering::SeqCst) {
            match cap.next_packet() {
                Ok(packet) => {
                    // Get the current timestamp
                    let timestamp: u64;
                    match SystemTime::now().duration_since(UNIX_EPOCH) { 
                        Ok(n) => {
                            timestamp = n.as_secs() * 1_000_000 + n.subsec_micros() as u64;
                        }
                        Err(_) => panic!("Cannot get time!"),
                    }
                    // Get the packet timestamp
                    let packet_timestamp = packet.header.ts.tv_sec as u64 * 1000000 + packet.header.ts.tv_usec as u64;
                    // Calculate the latency
                    let latency = timestamp - packet_timestamp;
                    // write latency to file
                    if let Err(e) = writeln!(file, "latency {} {} {}", packet_timestamp, timestamp, latency) {
                        eprintln!("Couldn't write to file: {}", e);
                    }
                    else {
                        file.flush().expect("Could not flush file");
                    }
                    // Get the packet length
                    // let packet_length = packet.header.len;
                    // Parse IEC 61850 Sampled Values
                    //let sv = sv_parser(&packet.data[42..packet_length as usize]);
                },
                Err(pcap::Error::NoMorePackets) => {
                    break;
                },
                Err(e) => {
                    eprintln!("Error reading packet: {}", e);
                },
            }
        } else {
            break; // SIGINT or SIGTERM received, break the loop
        }
    }

}
