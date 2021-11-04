//! DMA Drivers.





/// Trait for all Drivers that can digest (receive) Data from a DMA.
pub trait DMADigest {}

/// Trait for all Drivers that can feed (send) Data through a DMA.
pub trait DMAFeed {}
