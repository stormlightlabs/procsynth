# Procsynth

This project is an algorithmic music generator with an artwork generator.

## Overview

```text
[Motif + Rhythm + Random Algorithm]
         ↓
[List of Timed Note structs]
         ↓
[Track = Vec<Note>]
         ↓
[Write to MIDI sequence]
```

```sh
MUSIC
├── Drum Pattern Engine
├── Melody Engine
├── Chord Progressions
├── Song Section Assembler
└── Multi-Track MIDI Export

ART
├── Noise/Flow Texture Generator
├── Color Palettes
├── Frame Renderer (1 frame = 1 image)
└── Static Artwork

PIPELINE
├── Load KDL input
├── Generate music + artwork
└── Export final MIDI + Video + PNG
```

_NOTE_: KDL isn't required
