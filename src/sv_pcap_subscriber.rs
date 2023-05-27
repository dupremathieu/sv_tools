extern crate pcap;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub fn subscribe(interface: &str, output_file: &str) {
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

    while let Ok(packet) = cap.next_packet() {
        // Get the current timestamp
        let timestamp =  Instant::now();
        // Get the packet timestamp
        let packet_timestamp = packet.header.ts.tv_sec as u64 * 1000000 + packet.header.ts.tv_usec as u64;
        // Calculate the latency
        let latency = timestamp.elapsed().as_micros() as u64 - packet_timestamp;
        // write latency to file
        if let Err(e) = writeln!(file, "latency {} {}", timestamp.elapsed().as_micros(), latency) {
            eprintln!("Couldn't write to file: {}", e);
        }
        // Get the packet length
        // let packet_length = packet.header.len;
        // Parse IEC 61850 Sampled Values
        //let sv = sv_parser(&packet.data[42..packet_length as usize]);
    }
}
