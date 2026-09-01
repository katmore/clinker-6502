//! 6502 CPU model (NMOS).
//!
//! Instruction-stepped: [`Cpu::step`] executes one whole instruction and returns
//! its cycle count so the caller can advance peripherals by the same amount.
//! This is deliberately *not* a per-cycle bus-accurate core — nothing in the
//! Clinker 6502 spec needs cycle-exact bus behaviour.
//!
//! **Phase 1 status:** skeleton. Every opcode in [`decode`] is decoded to an
//! addressing mode and a base cycle count, and `step` walks the real
//! fetch → decode → execute → advance-PC loop, but only a handful of operations
//! actually touch the datapath. The rest are recognised and cycle-charged with
//! their logic left for phase 2.

pub mod decode;
pub mod interrupt;

use crate::bus::Bus;
use decode::{decode, AddrMode, Decoded, Mnemonic};

/// The processor status flags that are real state.
///
/// Bits 4 (B) and 5 are not stored — they exist only in the byte pushed to the
/// stack. Use [`Status::to_byte`] / [`Status::from_byte`] at those boundaries.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Status {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal: bool,
    pub overflow: bool,
    pub negative: bool,
}

impl Status {
    /// Pack into the on-stack byte layout. Bit 5 always reads as 1; bit 4
    /// reflects `break_flag` (set for `PHP`/`BRK`, clear for an IRQ/NMI push).
    pub const fn to_byte(self, break_flag: bool) -> u8 {
        (self.carry as u8)
            | (self.zero as u8) << 1
            | (self.interrupt_disable as u8) << 2
            | (self.decimal as u8) << 3
            | (break_flag as u8) << 4
            | 1 << 5
            | (self.overflow as u8) << 6
            | (self.negative as u8) << 7
    }

    /// Unpack from the on-stack byte layout, ignoring bits 4 and 5.
    pub const fn from_byte(b: u8) -> Self {
        Self {
            carry: b & 1 != 0,
            zero: b & 1 << 1 != 0,
            interrupt_disable: b & 1 << 2 != 0,
            decimal: b & 1 << 3 != 0,
            overflow: b & 1 << 6 != 0,
            negative: b & 1 << 7 != 0,
        }
    }
}

/// A 6502 core.
///
/// Interrupt inputs are plain fields the surrounding machine drives:
/// * `irq` — level-sensitive. Set it while any device is asserting; the core
///   takes the interrupt whenever `interrupt_disable` is clear.
/// * `nmi` — treated as a line level (`true` = asserted). The core services it
///   on the `false → true` transition only.
/// * `reset_pending` — request reset processing on the next [`step`](Cpu::step).
#[derive(Debug, Clone)]
pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    /// Stack pointer (offset into page 1).
    pub s: u8,
    pub pc: u16,
    pub status: Status,
    /// Total elapsed CPU cycles since power-on.
    pub cycles: u64,

    pub irq: bool,
    pub nmi: bool,
    nmi_prev: bool,
    pub reset_pending: bool,

    /// Skeleton diagnostics — removed once phase 2 implements the real ops.
    pub brk_trapped: bool,
    pub unknown_opcodes: u64,
}

impl Cpu {
    /// Power-on state. Registers are zeroed and the stack pointer parked at
    /// `$FD`; call [`reset`](Cpu::reset) to load `PC` from the reset vector.
    pub fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: 0xFD,
            pc: 0,
            status: Status::default(),
            cycles: 0,
            irq: false,
            nmi: false,
            nmi_prev: false,
            reset_pending: false,
            brk_trapped: false,
            unknown_opcodes: 0,
        }
    }

    /// Run the reset sequence now: `SP = $FD`, `I = 1`, `PC = [$FFFC]`, 7 cycles.
    pub fn reset(&mut self, bus: &mut impl Bus) {
        self.service_reset(bus);
    }

    /// Execute one instruction (or take a pending reset/NMI/IRQ) and return the
    /// number of cycles it consumed.
    pub fn step(&mut self, bus: &mut impl Bus) -> u8 {
        if self.reset_pending {
            self.reset_pending = false;
            return self.service_reset(bus);
        }

        let nmi_edge = self.nmi && !self.nmi_prev;
        self.nmi_prev = self.nmi;
        if nmi_edge {
            return self.service_nmi(bus);
        }
        if self.irq && !self.status.interrupt_disable {
            return self.service_irq(bus);
        }

        let opcode = self.fetch_u8(bus);
        match decode(opcode) {
            Some(ins) => self.execute(ins, bus),
            None => {
                self.unknown_opcodes += 1;
                self.cycles += 2;
                2
            }
        }
    }

    fn fetch_u8(&mut self, bus: &mut impl Bus) -> u8 {
        let v = bus.read_u8(self.pc);
        self.pc = self.pc.wrapping_add(1);
        v
    }

    fn execute(&mut self, ins: Decoded, bus: &mut impl Bus) -> u8 {
        // Skeleton: consume the operand bytes so PC lands after the instruction,
        // but effective-address resolution and most datapath logic are phase 2.
        let mut operand = [0u8; 2];
        for slot in operand.iter_mut().take(ins.mode.operand_len() as usize) {
            *slot = self.fetch_u8(bus);
        }
        let operand16 = u16::from_le_bytes(operand);

        match ins.mnemonic {
            Mnemonic::Nop => {}
            Mnemonic::Jmp if matches!(ins.mode, AddrMode::Absolute) => {
                self.pc = operand16;
            }
            Mnemonic::Sei => self.status.interrupt_disable = true,
            Mnemonic::Cli => self.status.interrupt_disable = false,
            Mnemonic::Sec => self.status.carry = true,
            Mnemonic::Clc => self.status.carry = false,
            Mnemonic::Brk => self.brk_trapped = true,
            _ => { /* recognised, not yet implemented in the skeleton */ }
        }

        self.cycles += ins.base_cycles as u64;
        ins.base_cycles
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
