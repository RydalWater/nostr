// Copyright (c) 2022 Yuki Kishimoto
// Distributed under the MIT software license

#[cfg(feature = "blocking")]
use once_cell::sync::Lazy;
#[cfg(feature = "blocking")]
use tokio::runtime::Runtime;

pub mod client;
pub mod relay;
pub mod subscription;

pub use client::Client;
pub use relay::{Relay, RelayPool, RelayPoolNotifications, RelayStatus};

#[cfg(feature = "blocking")]
static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().expect("Can't start Tokio runtime"));
