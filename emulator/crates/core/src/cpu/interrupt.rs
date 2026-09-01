//! Interrupt vectors and entry sequencing.
//!
//! **Phase 1:** the vectors are real and `service_reset` is real — enough to
//! bring the CPU up at its reset vector and run code. `service_nmi` and
//! `service_irq` are stubs: they jump through the correct vector and charge 7
//! cycles, but do not yet push PC/P to the stack. Phase 2 replaces the stubs
//! with the real sequence (push PCH, PCL, P with B clear; set I; load vector).

use super::Cpu;
use crate::bus::Bus;

/// `$FFFA`–`$FFFB` — non-maskable interrupt. Modelled but unused in Clinker v1.
pub const NMI_VECTOR: u16 = 0xFFFA;
/// `$FFFC`–`$FFFD` — reset.
pub const RESET_VECTOR: u16 = 0xFFFC;
/// `$FFFE`–`$FFFF` — maskable interrupt (shared with `BRK`). Carries the four
/// 6551 ACIAs and the WD1793 FDC completion line.
pub const IRQ_VECTOR: u16 = 0xFFFE;

impl Cpu {
    pub(super) fn service_reset(&mut self, bus: &mut impl Bus) -> u8 {
        self.s = 0xFD;
        self.status.interrupt_disable = true;
        self.pc = bus.read_u16(RESET_VECTOR);
        // Reset establishes the NMI edge-detector baseline.
        self.nmi_prev = self.nmi;
        self.cycles += 7;
        7
    }

    /// STUB — phase 2: push PC/P (B clear), set I, then load the vector.
    pub(super) fn service_nmi(&mut self, bus: &mut impl Bus) -> u8 {
        self.pc = bus.read_u16(NMI_VECTOR);
        self.cycles += 7;
        7
    }

    /// STUB — phase 2: push PC/P (B clear), set I, then load the vector.
    pub(super) fn service_irq(&mut self, bus: &mut impl Bus) -> u8 {
        self.status.interrupt_disable = true;
        self.pc = bus.read_u16(IRQ_VECTOR);
        self.cycles += 7;
        7
    }
}
