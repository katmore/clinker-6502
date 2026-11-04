# Clinker 6502 — Simplified Plan

A fresh, smaller starting point. This does not replace
[CLINKER-6502-SPEC.md](CLINKER-6502-SPEC.md) yet; that document stays as-is
until this one has been lived with for a while.

## The point

**The machine reads a MIDI file and plays it, making the sound itself.**
Everything else exists to serve that.

## Scope

1. **CPU** — a 6502. It runs the player program that reads the MIDI file and
   drives the sound hardware.
2. **Sound** — an onboard synth chip of the early-to-mid-80s kind. This is what
   turns notes into audio.
3. **RS-232** — one serial port to a terminal, so you can type commands
   ("play") and read messages.

RAM and ROM exist only because the CPU needs somewhere to run code and hold the
file. They are not features.

## Out of scope

- MIDI in, and driving an external synth: the machine makes its own sound.
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
5. **Sound chip.** Model the chip and send its output to the host's audio. A
   ROM program plays a hard-coded scale.
6. **MIDI file in memory.** Get a `.mid` file's bytes into the machine's RAM.
7. **Player.** 6502 code parses the file, keeps the tempo with a timer, and
   turns note-on and note-off events into sound-chip writes. You type `play`
   at the terminal and hear the song.

After step 7, decide what to do next.

## Open questions

Decide these when the code reaches them, not before.

- **Sound chip.** Candidates:
  - SID (1982): 3 voices, rich sound.
  - AY-3-8910 / YM2149: 3 square-wave voices, simple.
  - Yamaha FM (YM2151, or the later YM2612): 8 or 6 voices, the mid-80s
    synth sound.

  More voices make MIDI playback easier, because songs often have more than
  3 notes sounding at once.
- **Getting the file in.** Options:
  - the emulator loads it straight into RAM (simplest)
  - send it over the serial port (XMODEM, the period-authentic way)
  - add storage later
- **MIDI file types.** Type 0 (one track) only to start, or Type 1 as well
  (multiple tracks, which need merging at play time).
- **Timer.** Where the tempo clock comes from: the sound chip's own timer, the
  UART, or a separate timer chip.
- **Memory map.** Pick the smallest layout that works when step 3 arrives.
