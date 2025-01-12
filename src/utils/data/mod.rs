use wg_2024::network::{NodeId, SourceRoutingHeader};
use wg_2024::packet::{Fragment, Nack, NackType, Packet, PacketType};

pub fn test_fragment() -> Packet {
    let fragment = Fragment::from_string(0, 1, String::from("test fragment"));
    Packet {
        routing_header: SourceRoutingHeader {
            hop_index: 0,
            hops: Vec::new(),
        },
        session_id: 0,
        pack_type: PacketType::MsgFragment(fragment),
    }
}

pub fn test_nack(hops: Vec<NodeId>, nack_type: NackType) -> Packet {
    let nack = Nack {
        fragment_index: 0,
        nack_type,
    };
    Packet {
        routing_header: SourceRoutingHeader {
            hop_index: hops.len() - 1,
            hops,
        },
        session_id: 0,
        pack_type: PacketType::Nack(nack),
    }
}
