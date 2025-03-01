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

// Send Sequence Space

// 1         2          3          4
// ----------|----------|----------|----------
//   SND.UNA    SND.NXT    SND.UNA
//                        +SND.WND

// 1 - old sequence numbers which have been acknowledged
// 2 - sequence numbers of unacknowledged data
// 3 - sequence numbers allowed for new data transmission
// 4 - future sequence numbers which are not yet allowed

//        Send Sequence Space

struct SendSequenceSpace {
    /// send acknowledgement
    una: usize,
    /// send next
    nxt: usize,
    /// send window
    wnd: usize,
    /// send urgent pointer
    up: bool,
    /// segment sequence number used for last window update
    wl1: usize,
    /// segment acknowledge number used for last window update
    wl2: usize,
    /// initial send sequence nunber
    iss: usize
}

// Receive Sequence Space

//                        1          2          3
//                    ----------|----------|----------
//                           RCV.NXT    RCV.NXT
//                                     +RCV.WND

//         1 - old sequence numbers which have been acknowledged
//         2 - sequence numbers allowed for new reception
//         3 - future sequence numbers which are not yet allowed

//                          Receive Sequence Space

struct RecvSequenceSpace {
    // receive next
    nxt: usize,
    /// receive window
    wnd: usize,
    /// receive urgent pointer
    up: bool,
    /// initial receive sequence number
    irs: usize,
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
