#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;

/// Network type for Arctic.
#[derive(Clone, Copy, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum NetworkType {
    Arctic,
}

impl NetworkType {
    /// Return ss58 address prefix from network type.
    pub fn ss58_addr_format_id(&self) -> u8 {
        match self {
            NetworkType::Arctic => ARCTIC_PREFIX,
        }
    }
}

pub const ARCTIC_PREFIX: u8 = 110;
