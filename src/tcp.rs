#![allow(unused)]

pub enum State {
    Closed,
    Listen,
    //SynRcvd,
    //Estab,
}

impl State {
    pub fn on_packet(
        &mut self,
        iph: etherparse::Ipv4HeaderSlice,
        tcph: etherparse::TcpHeaderSlice,
        data: &[u8],
    ) {
        match self {
            State::Closed => {
                return;
            }
            State::Listen => {
                if !tcph.syn() {
                    return;
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

                // Potential problem with payload_len, it should be a 
                // payload, not a headeru16
                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len_u16(),
                    64,
                    etherparse::IpNumber::TCP,
                    iph.destination_addr().octets(),
                    iph.source_addr().octets(),
                )
                .unwrap();
            }
        }

        println!(
            "{:?}:{:?} -> {:?}:{:?} {:?}b of TCP",
            iph.source_addr(),
            tcph.source_port(),
            iph.destination_addr(),
            tcph.destination_port(),
            data.len()
        );
    }
}

impl Default for State {
    fn default() -> Self {
        //State::Closed
        State::Listen
    }
}
