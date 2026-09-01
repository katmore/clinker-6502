# Clinker 6502

Introducing the Clinker 6502, a serial-terminal microcomputer. The predecessor
to the Clinker 68000.

## Primary Features
 - MOS 6502 (NMOS), 2 MHz
 - 64 KB RAM, 48 KB usable
 - Serial console over RS-232 — bring your own VT100-class terminal; no keyboard
   or display hardware onboard
 - 4× RS-232 serial ports, one MOS 6551 ACIA each, software-polled shared IRQ
 - Serial RS-232 printer, drawn from the same 4-port pool
 - Dual 5.25" floppy bay, WD1793-class controller, standard soft-sectored format
 - MIDI output to an external synthesiser — one serial port plus an external
   500 kHz ÷16 clock divider. No onboard synthesis (that is the 68000's job).

## Technical Specifications
See [docs/CLINKER-6502-SPEC.md](docs/CLINKER-6502-SPEC.md)

## Emulation
 - Emulator core (Rust)
   - Note: in development — CPU skeleton only
 - Supported Software
   - **Clinker MIDI Catalog**
     - Note: in development
     - Flat-file catalogue — Name / Date / Kind / Size, sorted printout via the
       serial printer port
 - Supported peripherals:
   - Any required for the *Primary Features* listed above
 - Front-end for full-stack emulation experience: not yet implemented
