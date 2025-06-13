---
title: "Ideas"
date: 2025-06-12
---

## Music Theory

| Feature   | Desc                                                            |
| --------- | --------------------------------------------------------------- |
| Scale     | Random (or fixed) scale, like C major or A minor                |
| Key       | Range Instrument-appropriate pitch ranges (e.g., Piano: 48–72)  |
| Rhythm    | Weighted random choice of note durations: quarter, eighth, rest |
| Motifs    | Use short 3–5 note sequences, varied and repeated               |
| Structure | AABA form or looped patterns over bars                          |
| Velocity  | Slightly varied for natural feel                                |
| Timing    | 480 ticks per quarter note (PPQ), standard                      |

## Notes

- MIDI expresses tempo as: microseconds per quarter note
- Beat budgeting means we aim for an accurate timing target (e.g. 30s at 120bpm)

## Ideas

### Melody-Level (Microstructure)

| Feature      | Description                                                        | Impl                                                    |
| ------------ | ------------------------------------------------------------------ | ------------------------------------------------------- |
| Syncopation  | Off-beat emphasis, e.g., notes starting mid-beat or on weak beats  | Use sub-beat durations (e.g. 3/16), shifted start times |
| Accents      | Increased velocity or emphasis on certain beats                    | Add velocity shaping via curve or rule                  |
| Slurs / Ties | Extend notes across beats without retrigger                        | Combine durations, suppress NoteOn                      |
| Dotted       | rhythms 3:2 time patterns (e.g., dotted 8th + 16th)                | Add DottedEighth etc. to Duration                       |
| Swing        | feel Triplet-based timing: long-short pairs (e.g., 2/3 + 1/3 beat) | Quantize durations post-generation                      |

### Harmony-Level

| Feature           | Description                               | Impl                                               |
| ----------------- | ----------------------------------------- | -------------------------------------------------- |
| Chords            | Play multiple notes simultaneously        | Chord struct (Vec), mapped to MIDI on same tick    |
| Chord Progression | Functional harmonic movement (I–IV–V–I)   | Add ProgressionGenerator or mode-aware tonic shift |
| Arpeggios         | Broken chords played as rhythmic patterns | Generate from Chord and unroll over time           |
| Inversions        | Chord voicings with reordered intervals   | Arpeggio offset logic or chord shape mapping       |

### Instrument-Level

| Feature               | Description                                   | Impl                                         |
| --------------------- | --------------------------------------------- | -------------------------------------------- |
| Velocity shaping      | Vary loudness per note                        | Add velocity: u8 to TimedNote                |
| Range constraints     | Instruments have usable pitch/velocity ranges | Add per-instrument range filters             |
| Timbre mapping        | Map MIDI channels to realistic instruments    | TrackConfig per instrument                   |
| Track/channel routing | Polyphonic layering / orchestration           | Add multiple `Vec<TrackEvent>` or Part types |

### Form & Structure

| Feature           | Description                                    | Impl                                       |
| ----------------- | ---------------------------------------------- | ------------------------------------------ |
| Phrases           | Distinct musical units (e.g. 4-bar statements) |                                            |
| Themes/Variations | Altered motif reuses                           |                                            |
| AABA / Rondo form | High-level structural form                     |                                            |
| Modulation paths  | Planned key changes (not random)               | Use scale graph or functional harmony tree |

### Expression

| Feature      | Description                           | Impl                                             |
| ------------ | ------------------------------------- | ------------------------------------------------ |
| Humanization | Imperfect timing/velocity for realism | Add jitter/variance to timing & velocity         |
| Dynamic      | shaping Crescendo, decrescendo        | Map TimedNote velocity over time/curve           |
| Tempo shifts | Ritardando, accelerando               | Missing Vary BPM or tempo meta-event dynamically |

## Notes

### Theory

- Notes and Octaves (e.g., C4 = middle C)
- The 12 chromatic pitches (C, C#/Db, D, …, B)
- Enharmonic equivalents (C# = Db)
- Major and Minor scales
- Diatonic scale degrees (1 = tonic, 5 = dominant, etc.)
- Church modes (Ionian, Dorian, Phrygian, Lydian, etc.)
- Intervals (major 3rd = 4 semitones, perfect 5th = 7)
- Simple transposition (move melody by consistent interval)
- Motifs (short musical ideas, 3–6 notes)
- Rhythmic values (sixteenth, eighth, quarter, half notes)
- Rests (intentional silence of specific duration)
- Meter/time signatures (implied in 4/4 for now)
- Modulation (changing key or mode mid-phrase)
- Basic melodic contour (stepwise vs leaps)
- Phrase structure (motif repetition, variation, length control)

---

- Chord construction
- Triads: major, minor, diminished, augmented
- Seventh chords: dominant, major 7, minor 7, half-diminished
- Chord inversions (1st, 2nd inversion voicings)
- Diatonic chord functions (I, ii, iii, IV, V, vi, vii°)
- Common chord progressions (I–IV–V–I, ii–V–I, etc.)
- Voice leading principles
- Arpeggios (chord tones played in sequence)
- Functional harmony (tonic, subdominant, dominant roles)
- Harmonic rhythm (rate of chord changes)
- Formal structures (AABA, ABA’, 12-bar blues, rondo)
- Syncopation (accenting off-beats or weak beats)
- Swing feel and tuplets (triplet subdivisions, dotted rhythms)
- Accents and articulation (staccato, legato, marcato)
- Dynamics and expression (crescendo, decrescendo, forte, piano)
- Counterpoint basics (independent melodic lines interacting)
- Cadences (perfect, plagal, deceptive, half)

### MIDI & Code

- MIDI note numbers (0–127, C4 = 60)
- Basic MIDI messages (NoteOn, NoteOff)
- Delta time and PPQ (pulses per quarter note, e.g., 480)
- Writing .mid files with tempo and track events
- MIDI tempo event (MetaMessage::Tempo)
- MIDI channel usage (single-channel for now)
- Note velocity (loudness) – statically set, not varied
- Track events as sequential note instructions
- Tick-based timing (durations mapped to 480-based scale)
- Using midly for MIDI generation
- Seeded randomness (`StdRng`, `gen_range`, `choose`)
- Weighted random choice (`WeightedIndex`)
- Markov chain implementation (`HashMap<T, Vec<T>>`)
- Symbol-based generation (MotifSymbol enum)
- Modular generator structure (NoteSelector, RhythmSelector, etc.)
- Beat budgeting for precise duration control
- CLI parameter parsing with clap
- Writing to file with standard library I/O
- Logging generation metadata (seed, bpm, etc.)

---

- Multiple MIDI tracks (polyphony: e.g., chords + melody)
- MIDI channels for separate instruments
- Controlling note velocity dynamically (expression, accents)
- Humanization (micro-jitter for timing & velocity)
- Tempo curves (accelerando, ritardando)
- Program change events (instrument mapping in GM soundfonts)
- Control Change messages (expression, modulation wheel, volume)
- Track naming and metadata (MetaMessage::TrackName)
- Swing feel implementation via duration mapping
- Triplets and tuplets in tick math
- Slurs/ties (suppressing NoteOff between repeated notes)
- Using real-time audio engines (e.g., synth backend instead of just .mid)
- Parallel composition (generate and sync multiple voices)
- Dynamic struct dispatch with traits + boxed enums
- Formal testing of musical logic (e.g., motif properties, interval math)
