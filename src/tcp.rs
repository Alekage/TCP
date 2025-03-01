#![allow(unused)]

use std::io;

pub enum State {
    Closed,
    Listen,
    //SynRcvd,
    //Estab,
}

pub struct Connection {
    state: State
}

impl State {
    pub fn on_packet(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice,
        tcph: etherparse::TcpHeaderSlice,
        data: &[u8],
    ) -> io::Result<usize> {
        let mut buf = [0u8; 1500];

        match self {
            State::Closed => {
                return Ok(0);
            }
            State::Listen => {
                if !tcph.syn() {
                    return Ok(0);
                }

                // establishing a connection
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcph.destination_port(),
                    tcph.source_port(),
                    todo!(),
                    todo!(),
                );

                syn_ack.syn = true;
                syn_ack.ack = true;

                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len_u16(),
                    64,
                    etherparse::IpNumber::TCP,
                    iph.destination_addr().octets(),
                    iph.source_addr().octets(),
                )
                .unwrap();

                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    ip.write(&mut unwritten);
                    syn_ack.write(&mut unwritten);
                    unwritten.len()
                };

                nic.send(&buf[..unwritten])
            }
        }
    }
}

impl Default for Connection {
    fn default() -> Self {
        //State::Closed
        Connection {
            state: State::Listen
        }
    }
}
