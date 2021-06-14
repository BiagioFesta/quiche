// Copyright (C) 2021, Cloudflare, Inc.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS
// IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
// THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
// PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

//! Dummy Congestion Control

use std::time::Instant;

use crate::packet;

use crate::recovery::Acked;
use crate::recovery::CongestionControlOps;
use crate::recovery::Recovery;

pub static DUMMY: CongestionControlOps = CongestionControlOps {
    on_packet_sent,
    on_packet_acked,
    congestion_event,
    collapse_cwnd,
    checkpoint,
    rollback,
    has_custom_pacing,
};

fn on_packet_sent(_r: &mut Recovery, _sent_bytes: usize, _now: Instant) {}

fn on_packet_acked(
    _r: &mut Recovery, _packet: &Acked, _epoch: packet::Epoch, _now: Instant,
) {
}

fn congestion_event(
    _r: &mut Recovery, _time_sent: Instant, _epoch: packet::Epoch, _now: Instant,
) {
}

fn collapse_cwnd(_r: &mut Recovery) {}

fn checkpoint(_r: &mut Recovery) {}

fn rollback(_r: &mut Recovery) {}

fn has_custom_pacing() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::recovery;

    #[test]
    fn dummy_init() {
        let mut cfg = crate::Config::new(crate::PROTOCOL_VERSION).unwrap();
        cfg.set_cc_algorithm(recovery::CongestionControlAlgorithm::Dummy);

        let r = Recovery::new(&cfg);

        assert!(r.cwnd() > cfg.max_send_udp_payload_size);
        assert_eq!(r.bytes_in_flight, 0);
    }
}
