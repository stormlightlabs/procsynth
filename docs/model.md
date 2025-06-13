---
title: Domain Model
date: 2025-06-12
---

In this context, a track is a time-sequenced list of musical events (usually just notes)
that get rendered into MIDI. Each instrument (Piano, Guitar, Drums, etc.) gets its own
track to produce a song.

| Layer                  | Description                                                       | Example                                            |
| ---------------------- | ----------------------------------------------------------------- | -------------------------------------------------- |
| Track                  | A sequence of timed notes                                         | [NoteEvent, NoteEvent, ...]                        |
| NoteEvent              | A pitch, duration, velocity, and start time                       | Note { pitch: 60, start: 0.0, dur: 0.25, vel: 90 } |
| Motif                  | A repeated rhythmic or melodic shape                              | [0, 2, 4, 5] over [0.25, 0.25, 0.5, 1.0]           |
| Phrase                 | A series of motifs                                                | Often generated procedurally                       |
| Instrument Role        | Bass plays root+fifth, Guitar arpeggiates, Drums hit kicks/snares | Affects pitch range, rhythm style                  |
| Randomization Strategy | Markov, noise, weights                                            | How the next note is picked                        |

### Music Generation

| Step                 | Primary Algorithms Involved                                  |
| -------------------- | ------------------------------------------------------------ |
| Set Parameters       | (manual)                                                     |
| Choose Scale         | Constraint Solving, Generative Grammars                      |
| Chord Progression    | Markov Chains, Random Walks, Genetic Algorithms              |
| Generate Melody/Riff | Markov Chains, Perlin Noise, L-Systems, Graph Traversal      |
| Drums                | Poisson Disk Sampling, Self-Similarity Matrices, Markov      |
| Arrange Sections     | Simulated Annealing, Genetic Algorithms, Multi-Agent Systems |
| Export MIDI          | Event-Based Systems                                          |
| Post-Processing      | Envelope Functions, Noise, Neural Networks                   |

#### Riff Creation

| Subtask            | Useful Algorithms                      |
| ------------------ | -------------------------------------- |
| Pick next note     | Markov Chains, Random Walks            |
| Modulate flow      | Perlin Noise, Brownian Motion          |
| Recursive motifs   | L-Systems                              |
| Melodic layout     | Graph Traversal, Minimum Spanning Tree |
| Smooth transitions | Bezier Curves, Spline Interpolation    |

### Art Generation

| Step                     | Primary Algorithms Involved                                           |
| ------------------------ | --------------------------------------------------------------------- |
| Set Canvas Parameters    | (manual)                                                              |
| Choose Style/Theme       | (manual) (optional: Random Forests for style suggestion)              |
| Create Structure         | Perlin Noise, Poisson Disk Sampling, Voronoi Diagrams, Flow Fields    |
| Generate Shapes/Forms    | L-Systems, Voronoi, Delaunay Triangulation, Fractal Subdivision       |
| Add Variation and Detail | Perlin Noise, Fractal Noise, Bezier Curves, Brownian Motion           |
| Apply Composition Rules  | Constraint Solving, Graph Traversal, Minimum Spanning Tree            |
| Finalize and Export      | Event-Based Systems (order of drawing), (manual or procedural export) |

### Algorithms

- **Markov Chains** (melody/rhythm generation)
- **Perlin Noise** (texture and flow)
- **Affine Transformations** (move notes/shapes around)
- **L-Systems** (recursive pattern growth)
- **Voronoi + Delaunay** (natural partitions and meshes)

## Concepts

| Concept                 | Use in Music                     | Use in Art                                     | Style                                         |
| ----------------------- | -------------------------------- | ---------------------------------------------- | --------------------------------------------- |
| Constraint Solving      | Keep notes within a scale/chord  | Keep shapes within boundaries                  | Structured Composition, Formalist Art         |
| Generative Grammars     | Define musical phrases           | Define visual shape-building rules             | Folk, Classical, Decorative Generative Art    |
| Event-Based Systems     | Notes triggered over time        | Visual elements triggered over space or time   | Electronic, Kinetic Art                       |
| Multi-Agent Systems     | Multiple "musicians" interacting | Multiple "painters" or "particles" interacting | Jam Bands, Swarm Art, Emergent Visual Art     |
| Tiling and Tessellation | Rhythmic loops                   | Pattern repetition                             | Minimal Techno, Islamic Art, Tessellation Art |

## Links

[Twitch](https://twitch.tv/functionalcomposer)

## Resources

- [MIDI Note Nums](https://studiocode.dev/resources/midi-middle-c/)
- [Librosa](https://librosa.org/doc/latest/index.html#)
- [Chordmark](https://github.com/no-chris/chord-mark)
- [JotChord](https://www.jotchord.com/)
- Fundamentals of Music Processing by Meinard Müller
- [Music Theory for Nerds](https://eev.ee/blog/2016/09/15/music-theory-for-nerds/)

### Learning

- [Open Music Theory](https://viva.pressbooks.pub/openmusictheory/)
- [Music Theory for the 21st Century Classroom](https://musictheory.pugetsound.edu/mt21c/MusicTheory.html)

## Tools

- [FluidSynth](https://www.fluidsynth.org/)
