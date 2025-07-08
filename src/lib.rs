pub struct HepMessage {
    pub ipprotocol_family: u8,
    pub ipprotocol_id: u8,
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
