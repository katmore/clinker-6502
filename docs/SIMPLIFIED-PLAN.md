# Clinker 6502 — Simplified Plan

A fresh, smaller starting point. This does not replace
[CLINKER-6502-SPEC.md](CLINKER-6502-SPEC.md) yet; that document stays as-is
until this one has been lived with for a while.

## The point

**Play any MIDI file found on the internet today, with the machine making the
sound itself.** Everything else exists to serve that.

## The fiction

- **1985:** the machine ships with a 6502, a slot bus, a serial card, and an
  AY-3-8910 sound card (3 voices).
- **1992:** a hardware and ROM upgrade. The AY card is pulled and an OPL3 card
  goes in its slot. A file-memory card is added. The new ROM plays Standard
  MIDI Files with General MIDI instrument mapping.

This explains how a mid-80s design plays file formats from 1988 (SMF) and 1991
(General MIDI).

**Only the 1992 machine gets built.** The 1985 configuration is backstory.

## Hardware

| Part | Choice |
|---|---|
| CPU | One 6502, 2 MHz |
| Main memory | RAM plus the 1992 ROM |
| Bus | Apple II-style slot bus, 4 slots |
| Slot card: serial | One UART to a terminal. Used for commands and for uploading files |
| Slot card: file memory | 1 MB, outside the CPU's address space. Holds the song |
| Slot card: sound | Yamaha OPL3 (YMF262), 1990: 18 FM voices, stereo |

### Slot bus

Modelled on the Apple II's 6502-native slot bus (1977), where cards were
routinely swapped, sound cards included.

- Each slot gets its own small I/O window.
- Each card has a fixed **ID byte**. At boot, the ROM checks every slot and
  uses the right driver for each card it finds.

This also keeps the emulator code tidy, with one module per card.

### File memory card

Modelled on mid-80s Apple II memory cards such as Apple's Memory Expansion
Card and Applied Engineering's RamFactor. The 6502 reaches it through I/O
registers, not through its memory map:

| Register | What it does |
|---|---|
| Address low / mid / high | Set where to read or write, as a 24-bit address |
| Data | Read or write one byte. The address moves forward by one automatically after each access |

**Why 1 MB:** most MIDI files are under 100 KB. Dense ones reach a few hundred
KB, and files over 1 MB are almost entirely "black MIDI", a novelty genre no
real synth can play. The 24-bit address reaches 16 MB, so the size is easy to
change later.

### Sound card

**The OPL3 (YMF262):**

- It's the Sound Blaster 16 chip, so ready-made General MIDI instrument tables
  for it already exist.
- At power-up it behaves exactly like an OPL2, with 9 voices. The full 18
  voices and stereo can come later.
- Its built-in timers can raise an IRQ, so they keep the song's tempo and no
  separate timer chip is needed.

### Getting songs in

Over the serial port. Your terminal program sends the `.mid` with XMODEM or
YMODEM (period file-transfer protocols that modern terminal programs still
support), and the machine writes it into file memory as it arrives. YMODEM
also sends the filename and size.

Speeds for a 100 KB song:

| Speed (baud) | Time |
|---|---|
| 9600 | ~1.7 min |
| 19200 | ~50 s |
| 38400 | ~25 s |

Nothing persists across power-off, so you send the song again after each
power-up.

## Contemporary file support

These are requirements for the player, so that files from today's internet
just work:

- **Multi-track files.** Type 0 and Type 1, all 16 channels, drums on channel
  10.
- **Running status** and **tempo changes partway through a song**.
- **General MIDI instruments.** The GS and XG extensions (Roland's and
  Yamaha's) fall back to the nearest standard General MIDI instrument rather
  than going silent.
- **Skipping what it doesn't understand.** System messages (SysEx), unknown
  sections of the file, and odd headers are skipped safely rather than
  crashing.
- **Expressive controls.** Velocity, volume, expression, sustain pedal, and
  pitch bend.
- **Voice-stealing.** When more notes sound at once than there are voices, the
  melody and drums are protected.

## Out of scope

- Floppy drives and any other storage.
- MIDI in, and driving an external synth.
- The printer, the MIDI Catalog, bank switching, and clock-speed lore.
- Emulating the 1985 AY-3-8910 configuration.

## Roadmap

Each step ends with something that runs.

1. **CPU skeleton.** Done: registers, bus trait, opcode decode, reset.
2. **CPU complete.** All official opcodes and real IRQ/NMI/BRK. Passes the
   Klaus Dormann functional test.
3. **Minimal machine.** Main RAM + ROM, the slot bus with its card ID bytes,
   and a headless runner that steps the CPU.
4. **Serial card.** Wired to stdin/stdout, plus a small ROM program that
   echoes what you type.
5. **Sound card.** Model the OPL3, starting in its 9-voice OPL2 mode with its
   timers and IRQ, and send its output to the host's audio. A ROM program
   plays a hard-coded scale.
6. **File memory card.** 1 MB, with the address and data registers. At first,
   the emulator loads a `.mid` straight into it.
7. **Player.** 6502 code that reads the song from file memory and plays it,
   meeting the contemporary file support list. You type `play` at the
   terminal and hear the song.
8. **XMODEM/YMODEM upload.** Send a song from the terminal into file memory.
9. **Full OPL3.** 18 voices and stereo.

After step 9, decide what to do next.

## Open questions

Decide these when the code reaches them, not before.

- **UART chip** on the serial card: the 6551 (the 6502's companion part) or
  something else.
- **Instrument table:** which existing General MIDI instrument set for the
  OPL3 to base ours on.
- **Memory map:** pick the smallest layout that works when step 3 arrives.
