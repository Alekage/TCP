use std::net::Ipv4Addr;
use std::{collections::HashMap, io};

use tcp::State;

mod tcp;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    let mut connections: HashMap<Quad, tcp::Connection> = Default::default();
    let mut nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
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
            }
            Ok(iph) => {
                let src = iph.source_addr();
                let dst = iph.destination_addr();
                if iph.protocol().0 != 0x06 {
                    // not TCP
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(&buf[4 + iph.slice().len()..]) {
                    Err(e) => println!("Ignoring TCP packet {:?}", e),
                    Ok(tcph) => {
                        let quad = Quad {
                            src: (src, tcph.source_port()),
                            dst: (dst, tcph.destination_port()),
                        };

                        let datai = 4 + iph.slice().len() + tcph.slice().len();

                        connections.entry(quad).or_default().on_packet(
                            &mut nic,
                            iph,
                            tcph,
                            &buf[datai..nbytes],
                        )?;
                    }
                }
            }
        }
    }
}
