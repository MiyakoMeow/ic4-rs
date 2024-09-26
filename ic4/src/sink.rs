#![allow(clippy::redundant_closure)]

use crate::*;

/*
 * Sink
 */

bind_ptr_type!(
    Sink,
    ic4_sys::IC4_SINK,
    ic4_sys::ic4_sink_ref,
    ic4_sys::ic4_sink_unref
);

pub type SinkTypeOri = ic4_sys::IC4_SINK_TYPE;
bind_type!(SinkType, SinkTypeOri);

pub type SinkModeOri = ic4_sys::IC4_SINK_MODE;
bind_type!(SinkMode, SinkModeOri);
