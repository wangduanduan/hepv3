pub struct HepMessage {
    pub ip_protocol_family: u8,
    pub ip_protocol_id: u8,
    pub ip4_source_address: String,
    pub ip4_destination_address: String,
    pub ip6_source_address: String,
    pub ip6_destination_address: String,
    pub source_port: u16,
    pub destination_port: u16,
    pub timestamp: u32,
    pub timestamp_micro: u32,
    pub protocol_type: u8,
    pub capture_agent_id: u16,
    pub keep_alive_timer: u16,
    pub authenticate_key: String,
    pub body: String,
}

const HEP_MIN_PACKET_SIZE:u16 = 6;
const HEP_MAX_PACKET_SIZE:u16 = u16::MAX;

pub fn parse_packet(packet: &[u8]) -> Result<HepMessage, ()> {
    if packet.len() < HEP_MIN_PACKET_SIZE as usize || packet.len() > HEP_MAX_PACKET_SIZE as usize {
        return Err(());
    }

    let hep_message = HepMessage {
        ip_protocol_family: 0,
        ip_protocol_id: 0,
        ip4_source_address: String::new(),
        ip4_destination_address: String::new(),
        ip6_source_address: String::new(),
        ip6_destination_address: String::new(),
        source_port: 0,
        destination_port: 0,
        timestamp: 0,
        timestamp_micro: 0,
        protocol_type: 0,
        capture_agent_id: 0,
        keep_alive_timer: 0,
        authenticate_key: String::new(),
        body: String::new(),
    };

    // let version = HepVersion::from(&packet[..4]);

    // match version {
    //     HepVersion::HepV1 => {
    //         debug!("HEP Version 1");
    //         parse_hep_v1(packet)
    //     }
    //     HepVersion::HepV2 => {
    //         debug!("HEP Version 2");
    //         parse_hep_v2(packet)
    //     }
    //     HepVersion::HepV3 => {
    //         debug!("HEP version 3");
    //         parse_hep_v3(packet)
    //     }
    //     _ => {
    //         error!("Not matched HEP/EEP.");
    //         parse_hep_v1(packet)
    //     }
    // }
    return  Ok(hep_message);
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
