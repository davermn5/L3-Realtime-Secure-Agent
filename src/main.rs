use pnet::datalink::{self, Channel, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::Packet;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::task;
use log::{info, error};

#[derive(Debug, Deserialize, Clone)]
struct FirewallRule {
    src_ip: String,
    dst_ip: String,
    src_port: u16,
    dst_port: u16,
    action: Action
}

#[derive(Debug, Clone, Deserialize)]
enum Action {
    Allow,
    Drop
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    //Firewall rule-pooling allows Multiple Atomic reference-counted handles to be accessed across different threads while maintaining availability to the shared (parent) Mutex synchronization primitive.
    let rules:Arc<Mutex<Vec<FirewallRule>>> = Arc::new(Mutex::new(vec![]));

    //Listen on pre-defined interface that the edge device is listening on
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == "en0")
        .expect("Undiscoverable interface");

    println!("Hello, world!");
    Ok(())
}
