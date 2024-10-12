use pnet::datalink;
use std::thread;
use pnet::packet::ipv4::{Ipv4Packet};
use pnet::packet::ipv6::{Ipv6Packet};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::tcp::{TcpPacket};
use pnet::packet::udp::{UdpPacket};
use pnet::packet::Packet;
use std::env;
use std::io::prelude::*;
fn captures_packet(interface:&datalink::NetworkInterface , iface_name:String){
           
       let mut t = term::stdout().unwrap();
       t.fg(term::color::CYAN).unwrap();    
       if interface.name == iface_name{
             let (_, mut rx) = match datalink::channel(&interface , Default::default()){
            Ok(datalink::Channel::Ethernet(tx ,rx))=>(tx ,rx),
            Ok(_)=>panic!("Unhandled channel type"),
            Err(e)=>panic!("Failed to create the data link for  {}"
                      , e
             )
    };
    let mut count = 0;

    loop {
    match rx.next() {
         
        Ok(packet) => {
            if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                println!("[{:?}]",count);
                count +=1;
                let src_mac = ethernet_packet.get_source();
                let dst_mac = ethernet_packet.get_destination();
                let ether_type = ethernet_packet.get_ethertype();
                 
                println!("Source MAC: {:?}", src_mac);
                println!("Destination MAC: {:?}", dst_mac);
                println!("EtherType: {:?}", ether_type);

                match ether_type {
                    EtherTypes::Ipv4 => {
                        if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()) {
                            println!("IPv4 Packet:");
                            println!("Source IP: {:?}", ipv4_packet.get_source());
                            println!("Destination IP: {:?}", ipv4_packet.get_destination());
                            println!("Protocol: {:?}", ipv4_packet.get_next_level_protocol());
                           
                            // Handle additional fields as necessary

                            match ipv4_packet.get_next_level_protocol(){
                                    pnet::packet::ip::IpNextHeaderProtocols::Tcp=>{
                                         if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()){
                                                println!("Tcp Packet ");
                                                println!("Souce port: {}" , tcp_packet.get_source());
                                                println!("Dest Port: {}" , tcp_packet.get_destination());
                                                 println!("--------------------------------------");
                                         }
                                    }
                                    pnet::packet::ip::IpNextHeaderProtocols::Udp => {
        if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
            println!("UDP Packet:");
            println!("Source Port: {}", udp_packet.get_source());
            println!("Destination Port: {}", udp_packet.get_destination());
        }
    }
    _ => {
        println!("Unhandled IPv4 Protocol: {:?}", ipv4_packet.get_next_level_protocol());
    }
                            }
                        }
                    }
                    EtherTypes::Ipv6 => {
                        if let Some(ipv6_packet) = Ipv6Packet::new(ethernet_packet.payload()) {
                            println!("IPv6 Packet:");
                            println!("Source IP: {:?}", ipv6_packet.get_source());
                            println!("Destination IP: {:?}", ipv6_packet.get_destination());
                             println!("--------------------------------------");

                            // Handle additional fields as necessary
                        }
                    }
                    _ => {
                        // Handle other EtherTypes if necessary
                        println!("Unhandled EtherType: {:?}", ether_type);
                    }

                }
            }
        }

        Err(e) => {
            eprintln!("Error receiving packet: {}", e);
        }

    } 
}
       }
       t.reset().unwrap();
}

fn main() {

    let interface = datalink::interfaces();
    let mut handles = vec![];
    let args:Vec<String> = env::args().collect();
    if args.len() < 2 {
           panic!("[+]few agruments provided cargo r <iface_name>");
    }
    let handle1 = thread::spawn(move||{
           for iface in interface{
             captures_packet(&iface ,args[1].clone());
           }
    });handles.push(handle1);

    for handle in handles{
          handle.join().unwrap();
    }

}
