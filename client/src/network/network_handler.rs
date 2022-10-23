use crate::network::RemoteInfo;

use core::events::{ClientEvent, ServerEvent};
use std::io;
use std::net::UdpSocket;

const MAX_MESSAGE_SIZE: usize = 65535;

pub struct NetworkHandler {
    socket: UdpSocket,
}

impl NetworkHandler {
    pub fn new(info: RemoteInfo) -> io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.set_nonblocking(true).unwrap();
        socket.connect(format!("{}:{}", info.ip, info.port))?;

        Ok(Self { socket })
    }

    pub fn send(&self, event: ClientEvent) {
        let buffer = bincode::serialize(&event).unwrap();

        let _ = self.socket.send(&buffer);
    }

    pub fn process(&self) -> io::Result<Vec<ServerEvent>> {
        let mut events = Vec::new();

        loop {
            let mut data = [0; MAX_MESSAGE_SIZE];
            if self.socket.recv(&mut data).is_err() {
                break;
            }

            match bincode::deserialize(&data) {
                Ok(event) => events.push(event),
                Err(err) => error!("{}", err),
            }
        }

        Ok(events)
    }
}
