# Clinker 6502 — Simplified Plan

A fresh, smaller starting point. This does not replace
[CLINKER-6502-SPEC.md](CLINKER-6502-SPEC.md) yet; that document stays as-is
until this one has been lived with for a while.

## Scope

Three things, nothing else:

1. **CPU** — a 6502.
2. **RS-232** — one serial port. A terminal plugs in, so you type on its
   keyboard and the output shows on its screen.
3. **MIDI** — talk to an external synth. The era can stretch into the
   mid-80s.

RAM and ROM exist only because the CPU needs somewhere to run code. They are
not features.

## Parked

Not in this plan: floppy drives, the printer, extra serial ports, the MIDI
Catalog, bank switching, and clock-speed lore. They can be revisited later,
one at a time, if they still seem worth it.

## Roadmap

Each step ends with something that runs.

1. **CPU skeleton.** Done: registers, bus trait, opcode decode, reset.
2. **CPU complete.** All official opcodes, real IRQ/NMI/BRK. Passes the Klaus
   Dormann functional test.
3. **Minimal machine.** RAM + ROM behind the bus, a ROM image loads, and a
   headless runner steps the CPU.
4. **Serial terminal.** One UART model wired to stdin/stdout. A tiny ROM
   program echoes what you type. This is the "it's alive" milestone.
5. **MIDI out.** A second UART at 31.25 kbaud, with its bytes sent to a real
   or virtual MIDI device on the host. A ROM program plays a scale.
6. **MIDI in.** Notes from a host MIDI controller reach the 6502, so you can
   play through the machine.

After step 6, decide what to do next.

## Open questions

Decide these when the code reaches them, not before.

- **UART chip.** The 6551 ACIA (the 6502's companion part) or the 6850 ACIA
  (what most 80s MIDI interfaces used). Using one chip for both ports keeps
  things simple.
- **Memory map.** Pick the smallest layout that works when step 3 arrives.
- **Host MIDI.** Which Rust crate or OS API to use for step 5.
