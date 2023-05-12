extern crate pcap;

pub fn subscribe(interface: &str) {
    let mut cap = pcap::Capture::from_device(interface) // use the provided interface
        .unwrap()
        .promisc(true)
        .snaplen(5000)  // Configure the capture length
        .tstamp_type(pcap::TimestampType::Adapter)
        .open()
        .unwrap();

    cap.filter("ether proto 0x88ba", true).unwrap(); // Add a filter for IEC 61850 Sampled Values

    while let Ok(packet) = cap.next_packet() {
        println!("received packet! {:?}", packet);
    }
}
