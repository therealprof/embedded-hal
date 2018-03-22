//! Interrupt handling support interface

/// Abstraction for an interrupt
#[cfg(feature = "unproven")]
pub trait IRQ {
    /// Enable interrupt event generation
    fn enable(&mut self);

    /// Disable interrupt event generation
    fn disable(&mut self);

    /// Clear this interrupt after it was triggered
    fn clear(&mut self);
}
