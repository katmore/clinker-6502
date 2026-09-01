# Clinker 6502

Introducing the Clinker 6502, a serial-terminal microcomputer. The predecessor
to the [Clinker 68000](https://github.com/katmore/clinker-68000).

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

## License

Copyright 2026 D.B.

Licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for
the full text.

This program is free software: you can redistribute it and/or modify it under
the terms of the GNU General Public License as published by the Free Software
Foundation, either version 3 of the License, or (at your option) any later
version. It is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU General Public License for more details.
