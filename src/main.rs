use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];

    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);

        // IPV4 only
        if eth_proto != 0x800 {
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(pkt) => {
                let src = pkt.source_addr();
                let dest = pkt.destination_addr();
                let proto = pkt.protocol();

                eprintln!(
                    "{} -> {} {}b of protocol {:x}",
                    src,
                    dest,
                    pkt.payload_len(),
                    proto,
                );
            }
            Err(e) => {
                eprintln!("a vei affs {:?}", e);
            }
        }
    }
}
