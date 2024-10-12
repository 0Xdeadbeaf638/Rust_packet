mod info_packet;

use pnet::datalink::channel;
use pnet::datalink::Channel::Ethernet;
use pnet::datalink;
use pnet::packet::ethernet::EthernetPacket;


use std::thread;
fn packet_captures(interface:datalink::NetworkInterface){
                  println!("Attempting to create datalink channel for interface: {}", interface.name);

    if !interface.is_up() {
        panic!("The interface {} is down or not available", interface.name);
    }

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel for {}: {}",
            interface.name, e
        ),
    };

    println!("Started capturing packets on interface: {}", interface.name);
    println!("Start reading packets from interface {}",&interface );

    loop{
          match rx.next(){
                 Ok(packet) =>{
                       if let Some(ether_packet) = EthernetPacket::new(packet){
                               println!("__________");
                               let pakcet_info = info_packet::PacketInfos::new(&interface.name , &ether_packet);
                               println!("{}", pakcet_info);
                       }
                 }Err(e)=>{eprintln!("Error Occured {}",e );}
          }
    }
}

fn main(){
     let interfaces = datalink::interfaces();
   
      println!("Available interfaces:");
    for iface in &interfaces {
        println!("{} - {:?}", iface.name, iface.is_up());
    }
    let desired_interface_name = "enp0s3"; // Change this to your interface name

    // Find the desired interface
    let iface = interfaces
        .iter()
        .cloned()
        .find(|iface| iface.name == desired_interface_name)
        .expect("Desired interface not found");


    let handle = thread::spawn(move || {
        packet_captures(iface);
    });

    handle.join().unwrap();
}