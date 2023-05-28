extern crate pcap;
extern crate clap;
extern crate ctrlc;
use clap::{Arg, App};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod sv_pcap_subscriber;

fn main() {
    let matches = App::new("sv_tools")
        .version("0.1.0")
        .author("Mathieu Dupré <mathieu.dupre@savoirfairelinux.com>")
        .about("Tools to measure IEEC 61850 Sample value network latency")
        .arg(Arg::with_name("interface")
            .short('i')
            .long("interface")
            .value_name("INTERFACE")
            .help("Sets the network interface to listen on")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Sets the output file to write the latency")
            .takes_value(true))
        .get_matches();

    // Gets a value for interface if supplied by user
    let interface = matches.value_of("interface").unwrap_or("default");
    // Gets a value for output file if supplied by user
    let output_file = matches.value_of("output").unwrap_or("latency.txt");

    // Setup a boolean to indicate when to stop the program
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    sv_pcap_subscriber::subscribe(interface, output_file, running);
}
