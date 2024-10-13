use pnet::datalink;
use std::thread;
use pnet::packet::ipv4::{Ipv4Packet};
use pnet::packet::ethernet::{EthernetPacket, EtherTypes};
use pnet::packet::tcp::{TcpPacket};
use pnet::packet::udp::{UdpPacket};
use pnet::packet::Packet;
use std::env;
use pnet::packet::ethernet::EtherType;
use std::time::Duration;


fn captures_tcp_packets(ipv4_packet :Ipv4Packet){
        match ipv4_packet.get_next_level_protocol(){
                                    pnet::packet::ip::IpNextHeaderProtocols::Tcp=>{
                                         if let Some(tcp_packet) = TcpPacket::new(ipv4_packet.payload()){
                                                 let source_port = tcp_packet.get_source();
            let dest_port = tcp_packet.get_destination();

            // Assuming HTTP usually runs on port 80
            if source_port == 80 || dest_port == 80 {
                if let Ok(http_payload) = std::str::from_utf8(tcp_packet.payload()) {
                    if http_payload.contains("GET") || http_payload.contains("POST") {
                       let mut next = term::stdout().unwrap();
                       next.fg(term::color::RED).unwrap();
                        println!("HTTP Packet Detected:");
                        println!("Source Port: {}", source_port);
                        println!("Destination Port: {}", dest_port);
                        println!("Payload: {}", http_payload);
                        println!("--------------------------------------");
                        thread::sleep(Duration::from_secs(10));
                        next.reset().unwrap();
                    }
                }
            }
            let mut t = term::stdout().unwrap();
            t.fg(term::color::BLUE).unwrap();
            println!("Tcp Packet ");
            println!("Source port: {}", source_port);
            println!("Dest Port: {}", dest_port);
            println!("Tcp Payload {:?}" , tcp_packet.payload());
            println!("--------------------------------------");
            t.reset().unwrap();
                                         }
                                    }
                                    pnet::packet::ip::IpNextHeaderProtocols::Udp => {
        if let Some(udp_packet) = UdpPacket::new(ipv4_packet.payload()) {
            let mut t = term::stdout().unwrap();
            t.fg(term::color::MAGENTA).unwrap();
            println!("UDP Packet:");
            println!("Source Port: {}", udp_packet.get_source());
            println!("Destination Port: {}", udp_packet.get_destination());
            t.reset().unwrap();
        }
    }
    _ => {
        println!("Unhandled IPv4 Protocol: {:?}", ipv4_packet.get_next_level_protocol());
    }
                            }
}



fn captures_ip_packets(ether_type :EtherType , ethernet_packet:EthernetPacket){
        match ether_type{                                                                                                                                       
              EtherTypes::Ipv4=>{
                  if let Some(ipv4_packet) = Ipv4Packet::new(ethernet_packet.payload()){
                         let mut t = term::stdout().unwrap();
                         t.fg(term::color::CYAN).unwrap();
                          println!("IPv4 Packet.........");
                          println!("Source Address {}", ipv4_packet.get_source());
                          println!("Destination Address {}", ipv4_packet.get_destination());
                          println!("Protocol: {:?}", ipv4_packet.get_next_level_protocol());
                          t.reset().unwrap();
                          captures_tcp_packets(ipv4_packet);
                  }
              }
        
         _ => {
                        // Handle other EtherTypes if necessary
                        println!("Unhandled EtherType: {:?}", ether_type);
                    }
        }
}

fn captures_ether_packets(interface:&datalink::NetworkInterface , iface_name:String){
    if interface.name == iface_name {
            let (_ , mut rx) = match datalink::channel(&interface , Default::default()){
                      Ok(datalink::Channel::Ethernet(tx, rx))=>(tx, rx),
                      Ok(_)=>panic!("Unhandled type"),
                      Err(e)=>panic!("An error occured in {}: {}",iface_name,e),
            };

            let mut count = 0;

            loop {
                  match rx.next(){
                       Ok(packet)=> {
                          if let Some(ethernet_packet) = EthernetPacket::new(packet){
                                    println!("Packet Count :[{}]",count);
                                    count+=1;
                                    let mut t = term::stdout().unwrap();
                                    t.fg(term::color::CYAN).unwrap();
                                    println!("Ethernet Packet");
                                    println!("Mac Source : {}", ethernet_packet.get_source());
                                    println!("Mac Destination: {}" ,ethernet_packet.get_destination());
                                    println!("Ether Type : {}" ,ethernet_packet.get_ethertype());
                                    captures_ip_packets(ethernet_packet.get_ethertype() , ethernet_packet);
                                    println!("-----------------------------");
                                    t.reset().unwrap();
                       }
                   }
                       Err(_)=>panic!("No packets found")
                  }
            }
           
    }
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
             captures_ether_packets(&iface ,args[1].clone());
           }
    });handles.push(handle1);

    for handle in handles{
          handle.join().unwrap();
    }

}
