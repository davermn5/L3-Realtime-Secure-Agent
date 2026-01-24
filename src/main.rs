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

fn main() {
    println!("Hello, world!");
}
