extern crate pcap;
extern crate clap;
use clap::{Arg, App};

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
        .get_matches();

    // Gets a value for interface if supplied by user
    let interface = matches.value_of("interface").unwrap_or("default");
    sv_pcap_subscriber::subscribe(interface);
}
