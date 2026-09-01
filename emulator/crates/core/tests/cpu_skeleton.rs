//! Phase-1 gate: the CPU skeleton compiles and the fetch/decode/execute loop,
//! reset, and interrupt-line plumbing behave.

use clinker6502_core::bus::Bus;
use clinker6502_core::cpu::Cpu;

/// A flat 64 KiB address space for exercising the CPU in isolation.
struct TestBus {
    mem: [u8; 0x10000],
}

impl TestBus {
    fn new() -> Self {
        Self { mem: [0; 0x10000] }
    }

    fn load(&mut self, addr: u16, bytes: &[u8]) {
        let start = addr as usize;
        self.mem[start..start + bytes.len()].copy_from_slice(bytes);
    }
}

impl Bus for TestBus {
    fn read_u8(&mut self, addr: u16) -> u8 {
        self.mem[addr as usize]
    }
    fn write_u8(&mut self, addr: u16, val: u8) {
        self.mem[addr as usize] = val;
    }
}

fn booted(bus: &mut TestBus) -> Cpu {
    let mut cpu = Cpu::new();
    cpu.reset(bus);
    cpu
}

#[test]
fn reset_loads_pc_and_sane_state() {
    let mut bus = TestBus::new();
    bus.load(0xFFFC, &[0x00, 0xD0]); // reset vector -> $D000
    let cpu = booted(&mut bus);

    assert_eq!(cpu.pc, 0xD000);
    assert_eq!(cpu.s, 0xFD);
    assert!(cpu.status.interrupt_disable);
    assert_eq!(cpu.cycles, 7);
}

#[test]
fn step_runs_one_instruction_and_counts_cycles() {
    let mut bus = TestBus::new();
    bus.load(0xFFFC, &[0x00, 0xD0]);
    bus.load(0xD000, &[0xEA, 0xEA]); // NOP; NOP
    let mut cpu = booted(&mut bus);

    let c = cpu.step(&mut bus);
    assert_eq!(c, 2);
    assert_eq!(cpu.pc, 0xD001);
    assert_eq!(cpu.cycles, 9);

    cpu.step(&mut bus);
    assert_eq!(cpu.pc, 0xD002);
    assert_eq!(cpu.cycles, 11);
}

#[test]
fn jmp_absolute_updates_pc() {
    let mut bus = TestBus::new();
    bus.load(0xFFFC, &[0x00, 0xD0]);
    bus.load(0xD000, &[0x4C, 0x34, 0x12]); // JMP $1234
    let mut cpu = booted(&mut bus);

    cpu.step(&mut bus);
    assert_eq!(cpu.pc, 0x1234);
    assert_eq!(cpu.cycles, 7 + 3);
}

#[test]
fn sei_cli_toggle_the_interrupt_flag() {
    let mut bus = TestBus::new();
    bus.load(0xFFFC, &[0x00, 0xD0]);
    bus.load(0xD000, &[0x58, 0x78]); // CLI; SEI
    let mut cpu = booted(&mut bus);

    cpu.step(&mut bus);
    assert!(!cpu.status.interrupt_disable);
    cpu.step(&mut bus);
    assert!(cpu.status.interrupt_disable);
}

#[test]
fn unknown_opcode_is_counted_not_panicked() {
    let mut bus = TestBus::new();
    bus.load(0xFFFC, &[0x00, 0xD0]);
    bus.load(0xD000, &[0xFF]); // not wired in the skeleton
    let mut cpu = booted(&mut bus);

    cpu.step(&mut bus);
    assert_eq!(cpu.unknown_opcodes, 1);
}

#[test]
fn nmi_rising_edge_vectors_through_fffa() {
    let mut bus = TestBus::new();
    bus.load(0xFFFC, &[0x00, 0xD0]);
    bus.load(0xFFFA, &[0x00, 0xE0]); // NMI vector -> $E000
    bus.load(0xD000, &[0xEA]);
    bus.load(0xE000, &[0xEA]);
    let mut cpu = booted(&mut bus);

    cpu.nmi = true;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc, 0xE000);

    // Line still high but no fresh edge — next step runs a normal instruction.
    cpu.step(&mut bus);
    assert_eq!(cpu.pc, 0xE001);
}

#[test]
fn irq_respects_the_interrupt_disable_flag() {
    let mut bus = TestBus::new();
    bus.load(0xFFFC, &[0x00, 0xD0]);
    bus.load(0xFFFE, &[0x00, 0xF0]); // IRQ vector -> $F000
    bus.load(0xD000, &[0xEA, 0xEA]);
    let mut cpu = booted(&mut bus);

    // I is set coming out of reset — a pending IRQ must be ignored.
    cpu.irq = true;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc, 0xD001);

    // Clear I; the still-asserted line is now taken.
    cpu.status.interrupt_disable = false;
    cpu.step(&mut bus);
    assert_eq!(cpu.pc, 0xF000);
}
