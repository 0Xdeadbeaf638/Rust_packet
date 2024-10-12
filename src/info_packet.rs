use pnet::packet::ethernet::EthernetPacket;
use pnet::util::MacAddr;
use std::fmt;

pub struct PacketInfos {
    mac_address_source: MacAddr,
    mac_address_dest: MacAddr,
    interface_name: String, // Renamed for clarity
}

impl PacketInfos {
    pub fn new(interface_name: &str, ethernet_packet: &EthernetPacket<'_>) -> Self {
        PacketInfos {
            mac_address_source: ethernet_packet.get_source(),
            mac_address_dest: ethernet_packet.get_destination(),
            interface_name: interface_name.to_string(),
        }
    }
}

impl fmt::Display for PacketInfos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MAC Source: {}\n", self.mac_address_source)?;
        write!(f, "MAC Destination: {}\n", self.mac_address_dest)?;
        write!(f, "Interface: {}\n", self.interface_name)?;
        // Format other fields as needed

        Ok(())
    }
}
