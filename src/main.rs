use std::io;

fn main() -> io::Result<()> {
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    let mut buf = [0u8; 1504];
    
    loop {
        let nbytes = nic.recv(&mut buf[..])?;
        let _flags = u16::from_be_bytes([buf[0], buf[1]]);
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]);
        if eth_proto != 0x800 {
            //not IPv4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Err(e) => {
                eprintln!("Ignoring packet: {:?}", e);
            },
            Ok(p) => {
                let src = p.source();
                let dst = p.destination_addr();
                let proto = p.protocol();
                println!("{:?} -> {} {:?}b of protocol {:?}", src, dst, p.payload_len(), proto);
            }
        }

    };
}
