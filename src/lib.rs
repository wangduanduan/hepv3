pub enum ChunkType {
    IPProtocolFamily, // 1
	IPProtocolID, // 2
	IP4SourceAddress, // 3
	IP4DestinationAddress, // 4
	IP6SourceAddress, // 5
	IP6DestinationAddress, // 6
	SourcePort, // 7
	DestinationPort, // 8
	Timestamp, // 9
	TimestampMicro, // 10
	ProtocolType, // 11
	CaptureAgentID, // 12
	KeepAliveTimer, // 13
	AuthenticationKey, // 14
	PacketPayload, // 15
	CompressedPayload, // 16
	InternalC, // 17
    Unsupported //
}

impl std::convert::From<&[u8]> for ChunkType {
    fn from(b: &[u8]) -> Self {
        match b {
            [0,1] => ChunkType::IPProtocolFamily,
            [0,2] => ChunkType::IPProtocolID,
            [0,3] => ChunkType::IP4SourceAddress,
            [0,4] => ChunkType::IP4DestinationAddress,
            [0,5] => ChunkType::IP6SourceAddress,
            [0,6] => ChunkType::IP6DestinationAddress,
            [0,7] => ChunkType::SourcePort,
            [0,8] => ChunkType::DestinationPort,
            [0,9] => ChunkType::Timestamp,
            [0,10] => ChunkType::TimestampMicro,
            [0,11] => ChunkType::ProtocolType,
            [0,12] => ChunkType::CaptureAgentID,
            [0,13] => ChunkType::KeepAliveTimer,
            [0,14] => ChunkType::AuthenticationKey,
            [0,15] => ChunkType::PacketPayload,
            [0,16] => ChunkType::CompressedPayload,
            [0,17] => ChunkType::InternalC,
            _ => ChunkType::Unsupported,
        }
    }
}

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
