use std::str;

use nom;
use serde_json;

use types::{Packet, PushData, PushAck, ProtocolVersion};
use types::{Payload, Rxpk, Stat};

/// Parse protocol version
named!(protocol_version<&[u8], ProtocolVersion>,
    map!(
        take!(1), |b: &[u8]| match b[0] {
            1 => ProtocolVersion::V1,
            2 => ProtocolVersion::V2,
            n @ _ => ProtocolVersion::Other(n),
        }
    )
);

/// Parse random token
named!(random_token<&[u8], (u8, u8)>,
    map!(
        take!(2), |b: &[u8]| (b[0], b[1])
    )
);

/// Parse gateway UID
named!(gateway_uid<&[u8], &[u8]>,
    take!(8)
);

/// Parse JSON payload
named!(payload<&[u8], Payload>,
   map_res!(
        map_res!(
            nom::rest,
            str::from_utf8
        ),
        |txt: &str| serde_json::from_str(txt)
    )
);

named!(parse_push_data<&[u8], PushData>,
    do_parse!(
        v: protocol_version >>
        r: random_token >>
        tag!(&[0x00]) >>
        u: gateway_uid >>
        p: payload >>
        (PushData { version: v, random_token: r, gateway_uid: u, payload: p })
    )
);

named!(parse_push_ack<&[u8], PushAck>,
    do_parse!(
        v: protocol_version >>
        r: random_token >>
        tag!(&[0x01]) >>
        (PushAck { version: v, random_token: r })
    )
);

named!(pub parse_packet<&[u8], Packet>,
    alt!(
        map!(parse_push_data, |d| Packet::PushData(d)) |
        map!(parse_push_ack, |a| Packet::PushAck(a))
    )
);
