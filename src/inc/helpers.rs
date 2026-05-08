use core::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

pub fn to_hex_string(bytes: &Vec<u8>) -> String {
    let strs: Vec<String> = bytes.iter().map(|b| format!("{:02X}", b)).collect();
    strs.join(" ")
}

pub fn convert_to_ip_array(addr: String) -> [u8; 4] {
    let mut octets = [0u8; 4];
    let parts: Vec<&str> = addr.trim().split('.').collect();
    if parts.len() != 4 {
        return octets;
    }
    octets[0] = parts[0].parse().unwrap_or(0);
    octets[1] = parts[1].parse().unwrap_or(0);
    octets[2] = parts[2].parse().unwrap_or(0);
    octets[3] = parts[3].parse().unwrap_or(0);
    octets
}

pub fn build_server_addr(addr: String, port: u16) -> SocketAddr {
    let octets = convert_to_ip_array(addr);
    SocketAddr::V4(SocketAddrV4::new(
        Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]),
        port as u16,
    ))
}
