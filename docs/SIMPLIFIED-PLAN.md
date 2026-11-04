# Clinker 6502 — Simplified Plan

A fresh, smaller starting point. This does not replace
[CLINKER-6502-SPEC.md](CLINKER-6502-SPEC.md) yet; that document stays as-is
until this one has been lived with for a while.

## The point

**Play any MIDI file found on the internet today, with the machine making the
sound itself.** Everything else exists to serve that.

## The fiction

- **1985:** the machine ships with a 6502, an expansion bus, a serial card, and an
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
| Bus | Slotted expansion bus, 4 slots. The motherboard decodes addresses |
| Slot card: serial | One UART to a terminal. Used for commands and for uploading files |
| Slot card: file memory | 1 MB, outside the CPU's address space. Holds the song |
| Slot card: sound | Yamaha OPL3 (YMF262), 1990: 18 FM voices, stereo |

### Expansion bus

Every slot carries the same raw 6502 signals: address lines, data lines,
read/write, clock, and IRQ. The approach is the Apple II's (1977), whose slots
were filled with cards from Apple, third-party makers, and clone makers alike.

- **The motherboard decodes addresses.** One decoder chip (a 74HC138) watches
  the address lines. When the CPU touches a slot's addresses, the decoder
  switches on that slot's **select wire**.
- **Each slot gets its own 16 addresses** in the I/O area. Exact addresses get
  decided along with the memory map.
- **Cards stay simple.** A card connects the select wire straight to its main
  chip's chip-select pin, and the lowest 4 address lines pick the register.
  Cards need no decoding chips of their own.
- **Fixed slot assignments.** Every card always goes in the same slot, so the
  ROM simply knows where each card is and nothing gets detected at boot. Code
  writes to a card with a plain fixed address (for example `STA $C0C0`).
- **One shared IRQ line.** Any card can pull it to ask for attention. The CPU
  then checks each card's status register to see which card it was. Nothing
  needs jumpering.

| Slot | Card |
|---|---|
| 1 | Serial |
| 2 | File memory |
| 3 | Sound |
| 4 | Empty |

**Why the motherboard decodes:** it's one decoder chip for the whole machine,
rather than one or two on every card. That means less hardware in total,
simpler cards, and all the address logic in one place when something needs
debugging. In the emulator, each card is one module, and its address range
comes from its slot number.

**For a real build:** the breadboard 6502 walkthroughs already decode RAM, ROM,
and I/O with a few logic gates. Adding the '138 to the I/O part of that logic
and running its outputs to slot connectors is the step from there to this bus.
Several cards on the bus may also need a buffer chip (such as a 74HC245) on
the data lines.

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
- **For a real build:** the chip is out of production, but used and pulled
  chips are easy to find. It needs a companion chip (the YAC512) to turn its
  output into audio.

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
3. **Minimal machine.** Main RAM + ROM, the expansion bus with its slot
   decoding, and a headless runner that steps the CPU.
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
