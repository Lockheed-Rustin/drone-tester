use wg_2024::network::{NodeId, SourceRoutingHeader};
use wg_2024::packet::{FloodRequest, Fragment, Nack, NackType, NodeType, Packet, PacketType};

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

pub fn test_flood_request(flood_id: u64, initiator_id: NodeId, include_self: bool) -> Packet {
    let flood = FloodRequest {
        flood_id,
        initiator_id,
        path_trace: if include_self {
            vec![(initiator_id, NodeType::Client)]
        } else {
            vec![]
        },
    };
    Packet {
        routing_header: SourceRoutingHeader {
            hop_index: 0,
            hops: Vec::new(),
        },
        session_id: 0,
        pack_type: PacketType::FloodRequest(flood),
    }
}
