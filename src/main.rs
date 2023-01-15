use dns_lookup::lookup_host;

fn main() {
    let hostname = "localhost";
    let ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();
    assert!(ips.contains(&"127.0.0.1".parse().unwrap()));
}
