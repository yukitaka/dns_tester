use dns_lookup::lookup_host;
use clap::Parser;

/// DNS Tester
#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    author = env!("CARGO_PKG_AUTHORS"),
)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Select checking type name
    #[arg(
        value_name = "type name",
        short = 't',
        long = "type",
    )]
    exec_type: String,
    /// Target filename
    filename: String,
}

fn main() {
    let args = Args::parse();

    println!("{:#?}", args);

    let hostname = "localhost";
    let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
    assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
}
