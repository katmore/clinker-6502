//! Opcode → operation / addressing mode / base cycle count.
//!
//! **Phase 1:** the table *structure* is final and all 56 official mnemonics are
//! defined, but only a representative subset of opcodes — at least one per
//! addressing mode — is wired up. Unwired opcodes return `None`; phase 2 fills
//! the full 151-entry official table (and decides what to do with the NMOS
//! undocumented opcodes).

/// The 6502 addressing modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddrMode {
    Implied,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

impl AddrMode {
    /// Number of operand bytes that follow the opcode byte.
    pub const fn operand_len(self) -> u8 {
        match self {
            AddrMode::Implied | AddrMode::Accumulator => 0,
            AddrMode::Immediate
            | AddrMode::ZeroPage
            | AddrMode::ZeroPageX
            | AddrMode::ZeroPageY
            | AddrMode::Relative
            | AddrMode::IndirectX
            | AddrMode::IndirectY => 1,
            AddrMode::Absolute
            | AddrMode::AbsoluteX
            | AddrMode::AbsoluteY
            | AddrMode::Indirect => 2,
        }
    }
}

/// Every official 6502 operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[rustfmt::skip]
pub enum Mnemonic {
    Adc, And, Asl, Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Brk, Bvc, Bvs,
    Clc, Cld, Cli, Clv, Cmp, Cpx, Cpy, Dec, Dex, Dey, Eor, Inc, Inx,
    Iny, Jmp, Jsr, Lda, Ldx, Ldy, Lsr, Nop, Ora, Pha, Php, Pla, Plp,
    Rol, Ror, Rti, Rts, Sbc, Sec, Sed, Sei, Sta, Stx, Sty, Tax, Tay,
    Tsx, Txa, Txs, Tya,
}

/// A decoded opcode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decoded {
    pub mnemonic: Mnemonic,
    pub mode: AddrMode,
    /// Cycles before page-cross / branch-taken penalties (added in phase 2).
    pub base_cycles: u8,
}

const fn d(mnemonic: Mnemonic, mode: AddrMode, base_cycles: u8) -> Decoded {
    Decoded { mnemonic, mode, base_cycles }
}

/// Decode one opcode byte. `None` means "not wired in the phase-1 skeleton".
#[rustfmt::skip]
pub fn decode(opcode: u8) -> Option<Decoded> {
    use AddrMode::*;
    use Mnemonic::*;
    Some(match opcode {
        0x00 => d(Brk, Implied, 7),
        0xEA => d(Nop, Implied, 2),

        // Control flow
        0x4C => d(Jmp, Absolute, 3),
        0x6C => d(Jmp, Indirect,  5),
        0x20 => d(Jsr, Absolute,  6),
        0x60 => d(Rts, Implied,   6),
        0x40 => d(Rti, Implied,   6),

        // Flag ops
        0x18 => d(Clc, Implied, 2),
        0x38 => d(Sec, Implied, 2),
        0x58 => d(Cli, Implied, 2),
        0x78 => d(Sei, Implied, 2),

        // LDA — one entry per addressing mode it supports
        0xA9 => d(Lda, Immediate, 2),
        0xA5 => d(Lda, ZeroPage,  3),
        0xB5 => d(Lda, ZeroPageX, 4),
        0xAD => d(Lda, Absolute,  4),
        0xBD => d(Lda, AbsoluteX, 4),
        0xB9 => d(Lda, AbsoluteY, 4),
        0xA1 => d(Lda, IndirectX, 6),
        0xB1 => d(Lda, IndirectY, 5),
        0xA2 => d(Ldx, Immediate, 2),
        0xA0 => d(Ldy, Immediate, 2),

        // Stores
        0x85 => d(Sta, ZeroPage, 3),
        0x8D => d(Sta, Absolute, 4),

        // Register ops
        0xAA => d(Tax, Implied, 2),
        0xA8 => d(Tay, Implied, 2),
        0xE8 => d(Inx, Implied, 2),
        0xC8 => d(Iny, Implied, 2),
        0xCA => d(Dex, Implied, 2),
        0x88 => d(Dey, Implied, 2),

        // Branches
        0x10 => d(Bpl, Relative, 2),
        0x30 => d(Bmi, Relative, 2),
        0xD0 => d(Bne, Relative, 2),
        0xF0 => d(Beq, Relative, 2),

        _ => return None,
    })
}
