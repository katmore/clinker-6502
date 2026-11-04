# Clinker 6502 — Simplified Plan

A fresh, smaller starting point. This does not replace
[CLINKER-6502-SPEC.md](CLINKER-6502-SPEC.md) yet; that document stays as-is
until this one has been lived with for a while.

## The point

**Play any MIDI file found on the internet today, with the machine making the
sound itself.** Everything else exists to serve that.

## The fiction

The hardware is mid-80s. Its ROM was last updated in 1991–92, and that update
added a player for Standard MIDI Files with General MIDI instrument mapping.
This is why a mid-80s machine understands file formats from 1988 (SMF) and
1991 (General MIDI).

## Hardware

| Part | Choice |
|---|---|
| CPU | One 6502, 2 MHz |
| Sound | Yamaha OPL2 (YM3812), 1985: 9 FM voices, or 6 voices plus 5 drum sounds |
| Serial | One UART to a terminal, used to type commands and read messages |
| Memory | RAM and ROM, enough to run the player and hold a MIDI file |

**Why the OPL2:**

- It fits the era.
- It's the chip DOS games used to play General MIDI music, so ready-made
  General MIDI instrument tables already exist.
- Its built-in timers can raise an IRQ, so they keep the tempo and no separate
  timer chip is needed.
- At 2 MHz, the 6502 keeps up with its slow register writes.

## Out of scope

- MIDI in, and driving an external synth.
- Floppy drives, the printer, extra serial ports, the MIDI Catalog, bank
  switching, and clock-speed lore.

## Roadmap

Each step ends with something that runs.

1. **CPU skeleton.** Done: registers, bus trait, opcode decode, reset.
2. **CPU complete.** All official opcodes and real IRQ/NMI/BRK. Passes the
   Klaus Dormann functional test.
3. **Minimal machine.** RAM + ROM behind the bus, a ROM image loads, and a
   headless runner steps the CPU.
4. **Serial terminal.** One UART wired to stdin/stdout, plus a small ROM
   program that echoes what you type.
5. **OPL2.** Model the chip, including its timers and IRQ, and send its output
   to the host's audio. A ROM program plays a hard-coded scale.
6. **MIDI file in memory.** Get a `.mid` file's bytes into RAM.
7. **Player.** 6502 code that:
   - reads Type 0 and Type 1 files, merging the tracks
   - keeps the tempo from the OPL2's timer IRQ
   - maps General MIDI instruments and drums onto OPL2 sounds
   - steals voices when more than 9 notes sound at once

   You type `play` at the terminal and hear the song.

After step 7, decide what to do next.

## Open questions

Decide these when the code reaches them, not before.

- **Getting the file in.** Options:
  - the emulator loads it straight into RAM (simplest)
  - send it over the serial port (XMODEM, the period-authentic way)
  - add storage later
- **Big files.** Most MIDI files are 5–60 KB. Decide whether to refuse files
  that don't fit in RAM, or to read them in pieces.
- **UART chip.** The 6551 (the 6502's companion part) or something else.
- **Instrument table.** Which existing General MIDI instrument set for the OPL2
  to base ours on.
- **Memory map.** Pick the smallest layout that works when step 3 arrives.
