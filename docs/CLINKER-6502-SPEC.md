# Clinker 6502 — Spec Sheet (v1.0 — Ready for repo, 1 item flagged for implementation-time decision)

Predecessor to the [[Clinker 68000]]. Late-70s/early-80s class machine — serial-terminal era, not the bitmapped/HIL-successor era the 68000 model belongs to.

## 1. Hard Constraints (must hold)

| Subsystem | Spec |
|---|---|
| CPU | MOS 6502 family |
| RAM | 64 KB (6502's full 16-bit address space) |
| Console/Keyboard | Serial terminal, RS-232 |
| Printer | Serial, RS-232 |
| Serial ports | Many, standard (not add-on cards) — see §4a |
| Storage | Dual 5.25" floppy bay |
| MIDI | Drives external synth via serial port — see §7 |
| MIDI Catalog | Flat-file DB, sorted printout by Name/Date/Kind/Size — see §8 |

## 2. CPU & Clock Speed

- **CPU:** 6502-family, NMOS.
- **Locked: 2 MHz stock.** Rockwell/Synertek "A"-suffix binned parts (R6502A, SY6502A) real-shipped at this speed — the honest ceiling for a documented commercial part circa 1979–81, not an overclock story. Framing device for the machine's own lore if you want it: binned parts, not every unit necessarily hits rated speed — more true to how this actually worked historically than a flat guaranteed number anyway.
- Faster ("exotic," 3 MHz hand-selected batch) was considered and dropped for v1 — reopen only if a later revision specifically wants that story.

## 3. RAM

- 64 KB, flat — the 6502's entire 16-bit address space, populated stock, no expansion needed (same "ships maxed" approach as the 68000).
- **Locked: 48 KB usable RAM** — see §3a for the concrete split. Matches the real Apple II/Commodore-class pattern of reserving the high end for ROM + I/O.
- **Bank-switched RAM beyond 64 KB: deferred, not in v1.** Real precedent exists (Apple IIe, C128) and stays available as a v2 extension, but it adds a bank-select register and mapping complexity that doesn't serve "finish the emulator" as the current priority. Cut from scope, not forgotten.

## 3a. Address Map (Locked)

| Range | Size | Contents |
|---|---|---|
| `$0000`–`$BFFF` | 48 KB | RAM |
| `$C000`–`$CFFF` | 4 KB | I/O window — 4× 6551 ACIA (§4a), WD1793 FDC (§6), MIDI clock-divider control (§7) |
| `$D000`–`$FFF9` | ~12 KB | ROM (boot/monitor) |
| `$FFFA`–`$FFFF` | 6 B | 6502 hardwired vectors (NMI/RESET/IRQ) — lands in ROM, non-negotiable per the 6502's own architecture |

Same shape as classic Apple II memory maps, chosen for exactly that reason — it's a known-good, well-understood layout rather than a novel one, which matters more here than anywhere else in the spec since this is the piece an emulator's address-decode logic is built directly against.

## 4. Console / Keyboard

- Serial terminal over RS-232 — no dedicated keyboard-scan hardware, no HIL, no PC/XT-style scan-code link. The "keyboard" in this era is just whatever ASCII terminal you plug into the serial port (DEC VT-52/VT100-class or similar), same pattern used by KIM-1/AIM 65-class 6502 systems.
- Console occupies one port out of the standard multi-port pool (§4a) — not a dedicated separate UART, since there's no functional reason to wall it off from the general pool.
- **Emulation:** Trivial — model as a byte stream in/out at whatever baud rate you pick (300–9600 bps was the real-world range then), no scan-code state machine needed at all since the terminal owns that complexity, not Clinker.

## 4a. Standard Multi-Port RS-232

You asked for "as many as feasible" built-in (not add-on-card) ports. Worked backward from real constraints of the era rather than picking a round number:

- **Chip:** MOS 6551 ACIA — the companion UART MOS Technology built specifically for the 6502 (used in the Commodore PET, Plus/4, and the Apple II Super Serial Card). One chip per port, 4 registers each (data, status, command, control) — cleanest possible decode.
- **Real precedent for port count:** Seattle Computer Products' 400B serial board (period S-100 product) shipped in 2- or 4-channel configurations using 8251A UARTs — 4 channels was the top real commercial config for a single board of that class.
- **What actually limits the count** isn't address space — even a small reserved I/O page easily fits a dozen+ 4-byte register blocks. It's two other things: (1) chip count/board real estate, and (2) the 6502's single shared IRQ line — every additional port means more devices sharing one interrupt, each requiring a status-register poll to find out who fired. That's fine at a handful of ports; it gets genuinely worse (interrupt latency, polling overhead) as you add more.
- **Locked: 4 ports standard**, matching the SCP 400B's top real-world config. Console + printer + 2 free general-purpose ports, all from the identical chip/pool — nothing dedicated or special-cased. 6–8 with a priority-encoder chip stays available as a v2 extension if you want it later, cut from v1 for the same "finish it" reason as bank-switched RAM.

## 5. Printer

- Serial RS-232 — draws from the same standard port pool as the console (§4a), not a separately dedicated UART.
- **Emulation:** Same as console — byte stream, fixed rate, no protocol beyond basic RS-232 framing.

## 6. Storage — Dual 5.25" Floppy Bay

- Two bays, standard 5.25" — no 10-bay complexity here, this is the predecessor machine.
- **Controller:** WD1793-class FDC (or the WD1770 if you want the slightly later/cleaner variant) — genuinely period-correct and widely used across 6502-family machines (Coco, various CP/M boxes), well-documented, straightforward register set (command/status/track/sector/data).
- **Format:** Standard soft-sectored 5.25" (single or double density, your call) — no need to invent anything here, unlike the 68000's proprietary-bus 10-bay expander. Two drives is exactly what these controllers were built for natively (2-bit drive select covers it with room to spare).

## 7. MIDI (External Synth Driver)

Not onboard synthesis (that's the 68000's job) — Clinker 6502 drives an external synth over one of the standard RS-232 ports (§4a).

- **Confirmed sufficient as-speced:** the real-world floor for this class of task is the Commodore 64 — 1 MHz 6510, 64 KB total RAM (usable RAM well under that after OS overhead) — which ran full commercial MIDI sequencer software in the era. Clinker 6502 at 2 MHz / clean 64 KB sits above that floor already; no CPU/RAM bump needed.
- **Why it's cheap:** sound synthesis happens on the external box. Clinker's job is just sequencing and transmitting MIDI bytes — a modest, interrupt-driven serial-transmit load, not compute-heavy.
- **Clock requirement (the one real gotcha):** MIDI's 31.25 kbaud isn't a standard RS-232 rate. Whichever port drives the synth needs an external clock source divided correctly — the period-standard trick was a 500 kHz crystal ÷ 16 = 31.25 kHz feeding the UART's external clock input, same approach real C64/Atari MIDI interfaces used.
- **Non-issue, flagged anyway:** most period C64 MIDI software specifically needed 6850-register-compatible UARTs; the 6551s in Clinker's standard port pool (§4a) aren't register-compatible with the 6850. Irrelevant since Clinker runs its own OS/drivers, not ported period software — noted only so it's a deliberate non-issue, not a silent one.

## 8. MIDI Catalog (Software)

Old-school flat-file DB, not an RDBMS — organizes MIDI files by Name, Date, Kind (genre), and Size; produces a sorted printout via the RS-232 printer port (§5). Doesn't launch/play files, catalog only.

- **Record layout:** Three of the four fields are metadata about the *music*, not the file, and no vintage filesystem tracks them — they live in the catalog tool's own index file, one small record per entry:
  - **Name** — full composition title (not the filename)
  - **Date** — publication date of the piece (not a file timestamp)
  - **Kind** — genre, software-assigned category, never filesystem metadata on any period system
- **Size** is the one field that isn't stored redundantly — pulled live from the disk directory at print time, since that's already accurate and free.
- **Precedent:** same performance/complexity class as PFS:File or AppleWorks' database module — field-based records, sort, printed report — both ran on 1 MHz/64 KB Apple II hardware, which Clinker 6502 exceeds. A few dozen bytes per record (title string, date, genre code, filename pointer) against 64 KB RAM and two floppy bays isn't a meaningful storage load even for a few hundred entries.
- **Card-catalog shape:** filename in the index record is just a pointer to the real file for Size lookup — the authentic old-school pattern, not filesystem introspection.

## 9. Emulation Notes (suggestions only)

- 6502 core, 6551 ACIA, and WD1793 all have mature open reference implementations to study — none of this is unexplored territory, similar to the 68000 spec.
- Console and printer both reduce to "byte queue at a fixed baud rate" in emulation — the actual complexity budget on this machine is almost entirely the CPU core and the FDC, not I/O.
- Since there's no bitmapped display and no HIL/scan-code keyboard to emulate, Clinker 6502's emulator core is meaningfully smaller in scope than the 68000's — worth deciding whether you want it as a fully separate project or a "mode" within the same Rust workspace, given the shared lineage.

## Deferred to v2 (deliberately cut, not forgotten)

- **Bank-switched RAM beyond 64 KB** — real precedent exists (IIe/C128-style), cut from v1 to keep scope tight.
- **6–8 serial ports with priority-encoder interrupt handling** — v1 ships 4 on pure software polling; the encoder-based version is a real extension path, not a hardware redesign, if you want it later.
- **Faster "exotic" clock story** — 2 MHz is locked for v1; a hand-selected 3 MHz batch stays available as a documented variant later.

## Flagged for Implementation (decide in Claude Code, not here)

- **FDC and MIDI-port interrupt wiring aren't assigned yet.** §3a puts the WD1793 and MIDI clock-divider control in the I/O window, but neither has a stated IRQ vs. NMI assignment relative to the 4 ACIA ports sharing IRQ (§4a). Leave unresolved on purpose — decide when the interrupt-handling code actually gets written, not speculatively here. One reasonable starting point worth knowing about: WD1793 on NMI (floppy completion is high-priority and NMI can't be masked, so a disk operation can't get silently missed), MIDI clock-divider control staying on IRQ alongside the 4 ACIAs since it's a config register, not time-critical. Not locked — just a starting point for that session.
