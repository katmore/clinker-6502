//! The CPU's view of the outside world: a flat 16-bit address space of bytes.
//!
//! Everything the CPU can reach — RAM, ROM, and the `$C000`–`$CFFF` I/O window —
//! is accessed through this trait. Phase 1 ships only the trait itself; the real
//! address-decoding implementation (the locked `$0000`/`$C000`/`$D000` split)
//! lands in phase 3.

/// A device the CPU can read and write by address.
///
/// `read_u8` takes `&mut self` on purpose: reads in the I/O window have side
/// effects (e.g. a 6551 status read clears its interrupt-source bits), so the
/// bus cannot promise read-only access.
pub trait Bus {
    fn read_u8(&mut self, addr: u16) -> u8;

    fn write_u8(&mut self, addr: u16, val: u8);

    /// Little-endian 16-bit read: `addr` low, `addr + 1` high.
    ///
    /// Used for interrupt-vector fetches. The 6502's `JMP ($xxxx)` page-wrap
    /// bug is *not* modelled here — vectors live at `$FFFA`–`$FFFF` where it
    /// never triggers. A device that needs the quirk should not route through
    /// this method.
    fn read_u16(&mut self, addr: u16) -> u16 {
        let lo = self.read_u8(addr) as u16;
        let hi = self.read_u8(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }
}
