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

pub enum HepVersion {
    HepV1,
    HepV2,
    HepV3,
    Unknown,
}

impl std::convert::From<&[u8]> for HepVersion {
    fn from(b: &[u8]) -> Self {
        match b {
            [1, _, _, _] => HepVersion::HepV1,
            [2, _, _, _] => HepVersion::HepV2,
            [72, 69, 80, 51] => HepVersion::HepV3,
            _ => HepVersion::Unknown,
        }
    }
}

const HEP_MIN_PACKET_SIZE:u16 = 6;
const HEP_MAX_PACKET_SIZE:u16 = u16::MAX;

pub enum HepError {
    InvalidPacketSize,
    UnsupportedVersion,
    UnknownVersion
}

pub fn parse_packet(packet: &[u8]) -> Result<HepMessage, HepError> {
    if packet.len() < HEP_MIN_PACKET_SIZE as usize || packet.len() > HEP_MAX_PACKET_SIZE as usize {
        return Err(HepError::InvalidPacketSize);
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

    let version = HepVersion::from(&packet[..4]);

    match version {
        HepVersion::HepV1 => {
            return Err(HepError::UnsupportedVersion);
        }
        HepVersion::HepV2 => {
            return Err(HepError::UnsupportedVersion);
        }
        HepVersion::HepV3 => {
            // parse packet
            // return Err(HepError::UnsupportedVersion);
        }
        HepVersion::Unknown => {
            return Err(HepError::UnknownVersion);
        }
    }
    return Ok(hep_message);
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
