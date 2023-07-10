use std::env;

use pcap::Capture;

fn main() {
    let device1 = env::args().nth(1).unwrap().as_str().to_owned();
    let device2 = env::args().nth(2).unwrap().as_str().to_owned();

    let mut cap1 = Capture::from_device(device1.as_str())
        .unwrap()
        .promisc(true)
        .immediate_mode(true)
        .open()
        .unwrap()
        .setnonblock()
        .unwrap();

    let mut cap2 = Capture::from_device(device2.as_str())
        .unwrap()
        .promisc(true)
        .immediate_mode(true)
        .open()
        .unwrap()
        .setnonblock()
        .unwrap();

    loop {
        if let Ok(packet) = cap1.next_packet() {
            cap2.sendpacket(packet.data).unwrap();
        }

        if let Ok(packet) = cap2.next_packet() {
            cap1.sendpacket(packet.data).unwrap();
        }
    }
}
