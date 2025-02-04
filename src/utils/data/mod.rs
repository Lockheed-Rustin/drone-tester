use super::reverse_route;
use wg_2024::network::{NodeId, SourceRoutingHeader};
use wg_2024::packet::{Ack, FloodRequest, Fragment, Nack, NackType, NodeType, Packet, PacketType};

pub fn test_fragment(routing_header: SourceRoutingHeader) -> Packet {
    let fragment = Fragment::from_string(0, 1, String::from("test fragment"));
    Packet {
        routing_header,
        session_id: 0,
        pack_type: PacketType::MsgFragment(fragment),
    }
}

pub fn test_nack(
    routing_header: SourceRoutingHeader,
    drop_idx: usize,
    nack_type: NackType,
) -> Packet {
    let nack = Nack {
        fragment_index: 0,
        nack_type,
    };
    Packet {
        routing_header: reverse_route(routing_header, drop_idx),
        session_id: 0,
        pack_type: PacketType::Nack(nack),
    }
}

pub fn test_ack(routing_header: SourceRoutingHeader) -> Packet {
    let ack = Ack { fragment_index: 0 };
    Packet {
        routing_header,
        session_id: 0,
        pack_type: PacketType::Ack(ack),
    }
}

pub fn test_flood_request(flood_id: u64, initiator_id: NodeId, with_initiator: bool) -> Packet {
    let flood = FloodRequest {
        flood_id,
        initiator_id,
        path_trace: if with_initiator {
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
