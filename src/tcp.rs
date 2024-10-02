use std::usize;
use std::io;

pub enum State {
    Closed,
    //Estab,
    Listen,
    //SynRcvd,
}

impl Default for State {
    fn default() -> Self {
        // Assume all the ports are listening
        State::Listen
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize> {
        let mut buf = [0u8; 1500];
        match *self {
            Self::Closed => {
                return Ok(0);
            }
            Self::Listen => {
                if !tcph.syn() {
                    // Only syn for my bros
                    return Ok(0);
                }

                // i have been received a syn flag, lets start a new connection
                let mut syn_ack =
                    etherparse::TcpHeader::new(tcph.destination_port(), tcph.source_port(), unimplemented!(), unimplemented!());
                syn_ack.syn = true;
                syn_ack.ack = true;

                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.header_len().try_into().unwrap(),
                    69,
                    etherparse::IpNumber::TCP,
                    [
                        iph.destination()[0],                 
                        iph.destination()[1],
                        iph.destination()[2],
                        iph.destination()[3],
                    ],
                    [
                        iph.source()[0],                       
                        iph.source()[1],
                        iph.source()[2],
                        iph.source()[3],
                    ],
                );

                let unwritten = {
                    let mut unwritten = &mut buf[..];

                    match ip {
                        Ok(header) => header.write(&mut unwritten)?,
                        Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("Error: {:?}", e))),
                    }

                    syn_ack.write(&mut unwritten)?;
                    unwritten.len()
                };

                nic.send(&buf[..unwritten])?;
            }
        }
    }
}
