use utuntap::tun;
use etherparse::{IpHeader, PacketBuilder, PacketHeaders, TransportHeader};
use serial_test::serial;
use std::io::Read;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};

#[cfg(target_os = "linux")]
#[test]
#[serial]
fn tun_sents_packets() {
    let (mut file, filename) = tun::OpenOptions::new()
        .packet_info(false)
        .number(10)
        .open()
        .expect("failed to open device");
    assert_eq!(filename, "tun10");
    let data = [1; 10];
    let socket = UdpSocket::bind("10.10.10.1:2424").expect("failed to bind to address");
    socket
        .send_to(&data, "10.10.10.2:4242")
        .expect("failed to send data");
    let mut buffer = [0; 50];
    let number = file.read(&mut buffer).expect("failed to receive data");
    assert_eq!(number, 38);
    let packet = &buffer[..number];
    if let PacketHeaders {
        ip: Some(IpHeader::Version4(ip_header)),
        transport: Some(TransportHeader::Udp(udp_header)),
        payload,
        ..
    } = PacketHeaders::from_ip_slice(&packet).expect("failed to parse packet")
    {
        assert_eq!(ip_header.source, [10, 10, 10, 1]);
        assert_eq!(ip_header.destination, [10, 10, 10, 2]);
        assert_eq!(udp_header.source_port, 2424);
        assert_eq!(udp_header.destination_port, 4242);
        assert_eq!(payload, data);
    } else {
        assert!(false, "incorrect packet");
    }
}

#[cfg(target_os = "linux")]
#[test]
#[serial]
fn tun_receives_packets() {
    let (mut file, filename) = tun::OpenOptions::new()
        .packet_info(false)
        .number(10)
        .open()
        .expect("failed to open device");
    assert_eq!(filename, "tun10");
    let data = [1; 10];
    let socket = UdpSocket::bind("10.10.10.1:2424").expect("failed to bind to address");
    let builder = PacketBuilder::ipv4([10, 10, 10, 2], [10, 10, 10, 1], 20).udp(4242, 2424);
    let packet = {
        let mut packet = Vec::<u8>::with_capacity(builder.size(data.len()));
        builder
            .write(&mut packet, &data)
            .expect("failed to build packet");
        packet
    };
    file.write(&packet).expect("failed to send packet");
    let mut buffer = [0; 50];
    let (number, source) = socket
        .recv_from(&mut buffer)
        .expect("failed to receive packet");
    assert_eq!(number, 10);
    assert_eq!(source.ip(), IpAddr::V4(Ipv4Addr::new(10, 10, 10, 2)));
    assert_eq!(source.port(), 4242);
    assert_eq!(data, &buffer[..number]);
}
