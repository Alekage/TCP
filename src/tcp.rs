pub struct State {}


impl State {
    pub fn on_packet(&mut self, iph: etherparse::Ipv4HeaderSlice, tcph: etherparse::TcpHeaderSlice, data: &[u8]) {
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
        State {}
    }
}

