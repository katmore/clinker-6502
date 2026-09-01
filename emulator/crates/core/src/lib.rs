//! Clinker 6502 emulator core.
//!
//! **Phase 1 — CPU skeleton only.** This crate currently provides the CPU
//! register file, the [`bus::Bus`] trait, and a fetch/decode/execute stub. There
//! is no memory map, no peripheral models, and no [`Machine`]-level assembly yet;
//! those arrive in later phases (see `emulator/Cargo.toml`).
//!
//! Design commitments already locked (see `docs/CLINKER-6502-SPEC.md` and the
//! session notes):
//!
//! * Instruction-stepped timing: [`cpu::Cpu::step`] runs one whole instruction
//!   and returns its cycle count; callers advance peripherals by that amount.
//! * WD1793 FDC completion (INTRQ) will share the level-sensitive IRQ line with
//!   the four 6551 ACIAs. The NMI line is modelled but wired to nothing in v1.
//! * The MIDI clock divider is a plain register, not an interrupt source.

pub mod bus;
pub mod cpu;
