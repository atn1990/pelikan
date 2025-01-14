// Copyright 2021 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

//! Implements the wire protocol for the `Ping` protocol implementation.

mod request;
mod response;

pub use request::*;
pub use response::*;

use rustcommon_metrics::*;

#[cfg(feature = "server")]
counter!(PING);

#[cfg(feature = "client")]
counter!(PONG);
