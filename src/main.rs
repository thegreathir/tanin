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
        mirror(&mut cap1, &mut cap2);
        mirror(&mut cap2, &mut cap1);
    }
}

fn mirror(src_cap: &mut Capture<pcap::Active>, dst_cap: &mut Capture<pcap::Active>) {
    if let Ok(packet) = src_cap.next_packet() {
        if let Err(e) = dst_cap.sendpacket(packet.data) {
            println!("{}", e);
        }
    }
}
