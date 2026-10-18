//! Multi-tier succession and challenge grace period dead man's switch implementation.
//!
//! Provides documentation and module-level re-exports for the tiered succession model:
//! - Tier 1: Primary Backup with exclusive claim window.
//! - Tier 2: Multi-Sig Council of M-of-N signers with activation delay.
//! - Challenge Windows: 72-hour grace period during which the primary admin can re-arm the switch.

pub use crate::storage::DMS_GRACE_PERIOD;
pub use crate::types::DmsClaimStatus;
