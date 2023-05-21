use rand::random;
use serde::{Deserialize, Serialize};
use std::{net::UdpSocket, time::Duration};
use urlencoding::decode;

use actix::prelude::*;
use clap::Parser;
use magnet_url::Magnet;

use crate::frontend::FrontendMessage;

use super::cli::Args;

#[derive(Message)]
#[rtype(result = "()")]
pub enum BackendMessage {
    Quit,
    AddTorrent,
    DeleteTorrent,
}

pub struct Backend {
    recipient: Recipient<FrontendMessage>,
}

impl Actor for Backend {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        // let args = Args::parse();
        //
        // if let Ok(m) = magnet {
        // }
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        Running::Stop
    }
}

impl Backend {
    pub fn new(recipient: Recipient<FrontendMessage>) -> Self {
        Self { recipient }
    }
}

impl Handler<BackendMessage> for Backend {
    type Result = ();

    fn handle(&mut self, msg: BackendMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            BackendMessage::Quit => {
                ctx.stop();
                System::current().stop();
            }
            _ => {}
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ConnectReq {
    connection_id: u64,
    action: u32,
    transaction_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConnectRes {
    action: u32,
    transaction_id: u32,
    connection_id: u64,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn udp() {
        let magnet = Magnet::new("magnet:?xt=urn:btih:56BC861F42972DEA863AE853362A20E15C7BA07E&dn=Rust%20for%20Rustaceans%3A%20Idiomatic%20Programming&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.openbittorrent.com%3A6969%2Fannounce&tr=udp%3A%2F%2F9.rarbg.to%3A2710%2Fannounce&tr=udp%3A%2F%2F9.rarbg.me%3A2780%2Fannounce&tr=udp%3A%2F%2F9.rarbg.to%3A2730%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=http%3A%2F%2Fp4p.arenabg.com%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.torrent.eu.org%3A451%2Fannounce&tr=udp%3A%2F%2Ftracker.tiny-vps.com%3A6969%2Fannounce&tr=udp%3A%2F%2Fopen.stealth.si%3A80%2Fannounce");
        if let Ok(m) = magnet {
            // println!("name {:?}", decode(m.dn.unwrap().as_str()));
            // println!("xs {:?}", m.xs);
            // println!("hash_type {:?}", m.hash_type);
            // println!("tracker {:?}", decode(m.tr[0].as_str()));
            println!("");

            // literal magic number used for handshake
            const MAGIC: u64 = 0x0417_2710_1980;

            let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

            // let mut buf = [0; 512];

            let mut req = ConnectReq {
                connection_id: u64::to_be(MAGIC),
                action: 0,
                transaction_id: random::<u32>(),
            };

            let mut res = ConnectRes {
                action: 0,
                transaction_id: 0,
                connection_id: 0,
            };

            let mut buf_req = bincode::serialize(&req).unwrap();
            let mut buf_res = bincode::serialize(&res).unwrap();

            // let tracker = decode(&m.tr[0]).unwrap();

            socket
                .send_to(&buf_req, "184.105.151.166:6969")
                .expect("to send_to");

            socket.recv_from(&mut buf_res).expect("to receive_from");

            res = bincode::deserialize(&buf_res).unwrap();

            // let (amt, src) = socket.recv_from(&mut buf).expect("to recv_from");
            //
            // let buf = &mut buf[..amt];
            // let r = String::from_utf8_lossy(&buf);

            println!("got something? {:#?}", res);
            // println!("from someone? {:?}", src);
            // assert_eq!(false, true);
        }
    }
}
