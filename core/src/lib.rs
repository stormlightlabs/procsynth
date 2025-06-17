//! `procsynth_core` contains core music theory types for melody construction

// NOTE: Is there a convention for as_* vs to_* methods in Rust?
// NOTE: What does `Box` do?
// NOTE: '& static vs Box
// NOTE: How do we go from enum members to a byte/integer type?

pub mod events;
pub mod melody;

/// A pitch class is a set of all pitches that are a whole number
/// of octaves apart. For example, all C notes (C0, C1, C2, C4, etc.)
/// belong to the same pitch class.
///
/// The pitch classes follow the chromatic scale with sharps (♯) notation:
/// C, C♯, D, D♯, E, F, F♯, G, G♯, A, A♯, B
///
/// Each pitch class maps can be mapped to (MIDI number % 12):
/// - C = 0, C♯ = 1, D = 2, etc.
/// TODO: Handle enharmonic equivalents (e.g., C♯ vs D♭)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PitchClass {
    /// C natural (0 semitones from C)
    C,
    /// C sharp / D flat (1 semitone from C)
    Cs,
    /// D natural (2 semitones from C)
    D,
    /// D sharp / E flat (3 semitones from C)
    Ds,
    /// E natural (4 semitones from C)
    E,
    /// F natural (5 semitones from C)
    F,
    /// F sharp / G flat (6 semitones from C)
    Fs,
    /// G natural (7 semitones from C)
    G,
    /// G sharp / A flat (8 semitones from C)
    Gs,
    /// A natural (9 semitones from C)
    A,
    /// A sharp / B flat (10 semitones from C)
    As,
    /// B natural (11 semitones from C)
    B,
}

impl Into<u8> for &PitchClass {
    fn into(self) -> u8 {
        use PitchClass::*;
        match self {
            C => 0,
            Cs => 1,
            D => 2,
            Ds => 3,
            E => 4,
            F => 5,
            Fs => 6,
            G => 7,
            Gs => 8,
            A => 9,
            As => 10,
            B => 11,
        }
    }
}

impl From<u8> for PitchClass {
    fn from(value: u8) -> Self {
        match value {
            1 => PitchClass::Cs,
            2 => PitchClass::D,
            3 => PitchClass::Ds,
            4 => PitchClass::E,
            5 => PitchClass::F,
            6 => PitchClass::Fs,
            7 => PitchClass::G,
            8 => PitchClass::Gs,
            9 => PitchClass::A,
            10 => PitchClass::As,
            11 => PitchClass::B,
            0 | _ => PitchClass::C,
        }
    }
}

impl PitchClass {
    pub fn midi_base(&self) -> u8 {
        self.into()
    }
}

/// Represents an octave in the musical pitch system.
///
/// An octave is the interval between one musical pitch and another
/// with double its frequency. The octave numbering system typically
/// used in MIDI places middle C (C4) at MIDI note 60.
///
/// Common octave ranges:
/// - C-1 to B-1: MIDI 0-11 (very low, often sub-bass)
/// - C0 to B0: MIDI 12-23 (bass range)
/// - C4 to B4: MIDI 60-71 (middle octave, contains middle C)
/// - C8 to G8: MIDI 108-127 (highest MIDI range)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Octave(i8);

impl Octave {
    pub fn new(octave_num: i8) -> Self {
        Octave(octave_num)
    }

    /// Returns the octave number as a signed 8-bit integer.
    ///
    /// Negative octaves are valid in MIDI (e.g., C-1 = MIDI 0).
    pub fn num(self) -> i8 {
        self.0
    }
}

/// Represents a specific musical note with both pitch class and octave.
///
/// A note combines a [`PitchClass`] (which note: C, D, E, etc.) with
/// an [`Octave`]  to create a specific pitch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    /// The pitch class (C, D, E, F, G, A, B, and their sharps)
    pub pitch_cls: PitchClass,
    /// The octave number
    pub octave: Octave,
}

impl Into<u8> for Note {
    fn into(self) -> u8 {
        let midi_base: i16 = self.pitch_cls.midi_base().into();
        let octave: i16 = ((self.octave.num() + 1) * 12).into();

        (midi_base + octave) as u8
    }
}

impl Note {
    pub fn as_midi_number(self) -> u8 {
        self.into()
    }
}

/// Represents the duration of a musical note or rest.
///
/// Duration determines how long a note sounds or how long a silence lasts.
/// Standard durations follow Western musical notation:
///
/// - **Whole note**: 4 beats in 4/4 time
/// - **Half note**: 2 beats in 4/4 time
/// - **Quarter note**: 1 beat in 4/4 time (common unit)
/// - **Eighth note**: 0.5 beats in 4/4 time
/// - **Sixteenth note**: 0.25 beats in 4/4 time
///
/// Duration modifiers:
/// - **Dotted**: Adds half the duration (e.g., dotted quarter = 1.5 beats)
/// - **Triplet**: Divides duration by 3 (e.g., quarter triplet = 1/3 beat)
/// - **Custom**: Arbitrary duration in beats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Duration {
    /// Whole note (4 beats in 4/4 time)
    Whole,
    /// Half note (2 beats in 4/4 time)
    Half,
    /// Quarter note (1 beat in 4/4 time)
    Quarter,
    /// Eighth note (0.5 beats in 4/4 time)
    Eighth,
    /// Sixteenth note (0.25 beats in 4/4 time)
    Sixteenth,
    /// Dotted duration (adds 50% to the base duration)
    Dotted(&'static Duration),
    /// Triplet duration (divides duration by 3)
    Triplet(&'static Duration),
    /// Custom duration in beats (floating point)
    Custom(f32),
}

/// Represents dynamic markings that indicate the loudness/intensity of musical notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dynamic {
    /// pp - Very soft (MIDI velocity ~16)
    Pianissimo,
    /// p - Soft (MIDI velocity ~32)
    Piano,
    /// mp - Medium soft (MIDI velocity ~48)
    MezzoPiano,
    /// mf - Medium loud (MIDI velocity ~64)
    MezzoForte,
    /// f - Loud (MIDI velocity ~80)
    Forte,
    /// ff - Very loud (MIDI velocity ~112)
    Fortissimo,
    /// Custom velocity value (0-127)
    Custom(u8),
}

impl Into<u8> for Dynamic {
    fn into(self) -> u8 {
        match self {
            Dynamic::Pianissimo => 16,
            Dynamic::Piano => 32,
            Dynamic::MezzoPiano => 48,
            Dynamic::MezzoForte => 64,
            Dynamic::Forte => 80,
            Dynamic::Fortissimo => 112,
            Dynamic::Custom(velocity) => velocity,
        }
    }
}

impl Dynamic {
    pub fn as_midi_velocity(self) -> u8 {
        self.into()
    }
}

/// Represents musical modes, which define the interval patterns for scales.
///
/// Intervals are measured in semitones (half steps):
/// - 1 = half step (e.g., C to C♯)
/// - 2 = whole step (e.g., C to D)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// Ionian mode - same as Major (bright, happy)
    Ionian,
    /// Major scale - most common Western scale (bright, happy)
    Major,
    /// Dorian mode - minor scale with raised 6th (bittersweet)
    Dorian,
    /// Phrygian mode - minor scale with lowered 2nd (Spanish, exotic)
    Phrygian,
    /// Lydian mode - major scale with raised 4th (dreamy, floating)
    Lydian,
    /// Mixolydian mode - major scale with lowered 7th (bluesy, folk)
    Mixolydian,
    /// Aeolian mode - same as Natural Minor (sad, melancholic)
    Aeolian,
    /// Natural minor scale - most common minor scale (sad, melancholic)
    NaturalMinor,
    /// Locrian mode - diminished scale (dissonant, unstable)
    Locrian,
    /// Custom mode with user-defined intervals
    Custom(&'static [u8; 7]),
}

impl Mode {
    /// Returns the interval pattern for this mode as an array of semitones.
    /// The pattern always contains 7 intervals that sum to 12 (one octave).
    pub fn intervals(self) -> &'static [u8] {
        match self {
            Mode::Ionian | Mode::Major => &[2, 2, 1, 2, 2, 2, 1],
            Mode::Dorian => &[2, 1, 2, 2, 2, 1, 2],
            Mode::Phrygian => &[1, 2, 2, 2, 1, 2, 2],
            Mode::Lydian => &[2, 2, 2, 1, 2, 2, 1],
            Mode::Mixolydian => &[2, 2, 1, 2, 2, 1, 2],
            Mode::Aeolian | Mode::NaturalMinor => &[2, 1, 2, 2, 1, 2, 2],
            Mode::Locrian => &[1, 2, 2, 1, 2, 2, 2],
            Mode::Custom(intervals) => intervals,
        }
    }
}

/// # Common Scales
///
/// - **C Major**: C, D, E, F, G, A, B (all white keys on piano)
/// - **A Minor**: A, B, C, D, E, F, G (relative minor of C Major)
/// - **G Major**: G, A, B, C, D, E, F♯ (one sharp)
/// - **D Dorian**: D, E, F, G, A, B, C (minor scale with raised 6th)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale {
    /// The root note (tonic) of the scale
    pub root: PitchClass,
    /// The mode that defines the interval pattern
    pub mode: Mode,
}

impl Scale {
    /// Returns a vector containing the seven scale degrees (notes)
    /// starting from the root note and following the mode's interval
    /// pattern.
    pub fn notes(self, octave: Octave) -> Vec<Note> {
        let intervals = self.mode.intervals();

        let mut current_note = self.root.midi_base();
        let mut notes = Vec::with_capacity(7);

        notes.push(Note {
            pitch_cls: self.root,
            octave,
        });

        for &step in &intervals[..6] {
            current_note = (current_note + step) % 12;

            notes.push(Note {
                pitch_cls: PitchClass::from(current_note),
                octave,
            })
        }

        notes
    }
}

/// Represents the quality/type of a chord, defining its harmonic character.
///
/// Chord kinds determine which notes are included in a chord by specifying
/// the intervals from the root note. Different chord kinds create different
/// harmonic colors and emotional qualities.
///
/// # Basic Triads (3-note chords)
///
/// - **Major**: Bright, happy sound - intervals [0, 4, 7] (root, major 3rd, perfect 5th)
/// - **Minor**: Sad, somber sound - intervals [0, 3, 7] (root, minor 3rd, perfect 5th)
/// - **Diminished**: Tense, unstable - intervals [0, 3, 6] (root, minor 3rd, diminished 5th)
/// - **Augmented**: Mysterious, floating - intervals [0, 4, 8] (root, major 3rd, augmented 5th)
///
/// # Seventh Chords (4-note chords)
///
/// - **Major7**: Jazz, sophisticated - intervals [0, 4, 7, 11] (major triad + major 7th)
/// - **Minor7**: Smooth, mellow - intervals [0, 3, 7, 10] (minor triad + minor 7th)
/// - **Dominant7**: Bluesy, tension - intervals [0, 4, 7, 10] (major triad + minor 7th)
///
/// # Intervals
///
/// All intervals are measured in semitones from the root:
/// - 0 = root (unison)
/// - 3 = minor third, 4 = major third
/// - 7 = perfect fifth
/// - 10 = minor seventh, 11 = major seventh
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordKind {
    /// Major triad - bright, happy (root, major 3rd, perfect 5th)
    Major,
    /// Minor triad - sad, somber (root, minor 3rd, perfect 5th)
    Minor,
    /// Diminished triad - tense, unstable (root, minor 3rd, diminished 5th)
    Diminished,
    /// Augmented triad - mysterious, floating (root, major 3rd, augmented 5th)
    Augmented,
    /// Major seventh chord - sophisticated, jazzy (major triad + major 7th)
    Major7,
    /// Minor seventh chord - smooth, mellow (minor triad + minor 7th)
    Minor7,
    /// Dominant seventh chord - bluesy, creates tension (major triad + minor 7th)
    Dominant7,
    /// Custom chord with user-defined intervals
    Custom(&'static [u8]),
}

impl ChordKind {
    /// Returns the interval pattern for this chord kind as semitones from the root.
    ///
    /// Each number represents the number of semitones above the root note:
    /// - 0 = root note (unison)
    /// - 3 = minor third, 4 = major third
    /// - 7 = perfect fifth
    /// - 10 = minor seventh, 11 = major seventh
    pub fn intervals(self) -> &'static [u8] {
        use ChordKind::*;
        match self {
            Major => &[0, 4, 7],
            Minor => &[0, 3, 7],
            Diminished => &[0, 3, 6],
            Augmented => &[0, 4, 8],
            Major7 => &[0, 4, 7, 11],
            Minor7 => &[0, 3, 7, 10],
            Dominant7 => &[0, 4, 7, 10],
            Custom(intervals) => intervals,
        }
    }
}

/// Represents a musical chord with a specific root note and chord quality.
///
/// A chord is a combination of three or more different notes played simultaneously.
/// It consists of:
/// - A **root note**: The foundational note that gives the chord its name
/// - A **chord kind**: The quality that determines which other notes are included
///
/// Chords are fundamental to harmony and provide the harmonic foundation for
/// melodies. They create different emotional qualities and musical tensions
/// that drive musical progressions.
///
/// # Common Chords
///
/// - **C Major**: C + E + G (happy, stable)
/// - **A Minor**: A + C + E (sad, melancholic)
/// - **G7**: G + B + D + F (creates tension, wants to resolve to C)
/// - **Dm7**: D + F + A + C (jazzy, smooth)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    /// The root note that forms the foundation of the chord
    pub root: Note,
    /// The chord quality that determines the harmonic intervals
    pub kind: ChordKind,
}

impl Chord {
    /// Generates all notes in this chord, calculated by adding the
    /// chord kind's intervals to the root note in ascending pitch order.
    /// TODO: Scale Direction
    pub fn notes(&self) -> Vec<Note> {
        let root_midi = self.root.as_midi_number();

        self.kind
            .intervals()
            .iter()
            .map(|&interval| {
                let new_midi = root_midi + interval;
                let octave_change = (new_midi / 12) as i8 - (root_midi / 12) as i8;
                let new_octave = Octave(self.root.octave.0 + octave_change);

                Note {
                    pitch_cls: PitchClass::from(new_midi % 12),
                    octave: new_octave,
                }
            })
            .collect()
    }
}

pub type Key = Scale;

/// A time signature consists of two numbers:
/// - **Numerator**: How many beats per measure
/// - **Denominator**: What note value gets one beat
///
/// # Common Time Signatures
///
/// - **4/4**: Four quarter-note beats per measure (most common, "common time")
/// - **3/4**: Three quarter-note beats per measure (waltz time)
/// - **2/4**: Two quarter-note beats per measure (march time)
/// - **6/8**: Six eighth-note beats per measure (compound time)
/// - **5/4**: Five quarter-note beats per measure (irregular meter)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeSignature(pub u8, pub u8);

/// Represents the tempo (speed) of music in beats per minute (BPM).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tempo(pub u16);

/// Represents common tempo markings used in music notation.
pub enum TempoMarkings {
    /// Very slow tempo (40-60 BPM) - solemn, stately
    Largo,
    /// Slow tempo (66-76 BPM) - leisurely, relaxed
    Adagio,
    /// Moderate tempo (76-108 BPM) - walking pace, flowing
    Andante,
    /// Moderate tempo (108-120 BPM) - comfortable, steady
    Moderato,
    /// Fast tempo (120-168 BPM) - lively, cheerful
    Allegro,
    /// Very fast tempo (168-200 BPM) - urgent, exciting
    Presto,
}

impl TempoMarkings {
    /// Returns the typical BPM range for this tempo marking.
    pub fn bpm_range(&self) -> (u16, u16) {
        match self {
            TempoMarkings::Largo => (40, 60),
            TempoMarkings::Adagio => (66, 76),
            TempoMarkings::Andante => (76, 108),
            TempoMarkings::Moderato => (108, 120),
            TempoMarkings::Allegro => (120, 168),
            TempoMarkings::Presto => (168, 200),
        }
    }

    pub fn to_tempo(&self) -> Tempo {
        let (min_bpm, max_bpm) = self.bpm_range();
        // Use the average BPM for the tempo marking
        Tempo((min_bpm + max_bpm) / 2)
    }
}

impl Into<Tempo> for TempoMarkings {
    fn into(self) -> Tempo {
        let (min_bpm, max_bpm) = self.bpm_range();
        // Use the average BPM for the tempo marking
        Tempo((min_bpm + max_bpm) / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_class_conversion() {
        let c_val: u8 = (&PitchClass::C).into();
        assert_eq!(c_val, 0);
        let a_val: u8 = (&PitchClass::A).into();
        assert_eq!(a_val, 9);

        assert_eq!(PitchClass::from(0), PitchClass::C);
        // Test default case
        assert_eq!(PitchClass::from(255), PitchClass::C);
    }

    #[test]
    fn test_octave_num() {
        let octave = Octave::new(4);
        assert_eq!(octave.num(), 4);

        let negative_octave = Octave::new(-1);
        assert_eq!(negative_octave.num(), -1);
    }

    #[test]
    fn test_note_to_midi_number() {
        // Middle C (C4)
        let middle_c = Note {
            pitch_cls: PitchClass::C,
            octave: Octave::new(4),
        };
        assert_eq!(middle_c.as_midi_number(), 60);

        // A4 (440 Hz)
        let a4 = Note {
            pitch_cls: PitchClass::A,
            octave: Octave::new(4),
        };
        assert_eq!(a4.as_midi_number(), 69);

        // C0 (lowest C)
        let c0 = Note {
            pitch_cls: PitchClass::C,
            octave: Octave::new(-1),
        };
        assert_eq!(c0.as_midi_number(), 0);

        // Test B in octave 9 (close to MIDI max)
        let b9 = Note {
            pitch_cls: PitchClass::B,
            octave: Octave::new(9),
        };

        // This would be > 127, but we test the calculation
        assert_eq!(b9.as_midi_number(), 131);
    }

    #[test]
    fn test_dynamic_to_midi_velocity() {
        assert_eq!(Dynamic::Pianissimo.as_midi_velocity(), 16);
        assert_eq!(Dynamic::Forte.as_midi_velocity(), 80);
        assert_eq!(Dynamic::Custom(100).as_midi_velocity(), 100);
    }

    #[test]
    fn test_mode_intervals() {
        assert_eq!(Mode::Major.intervals(), &[2, 2, 1, 2, 2, 2, 1]);
        let custom_intervals: &'static [u8; 7] = &[1, 1, 2, 2, 1, 3, 2];
        assert_eq!(Mode::Custom(custom_intervals).intervals(), custom_intervals);
    }

    #[test]
    fn test_scale_generation() {
        // C Major scale in octave 4
        let c_major = Scale {
            root: PitchClass::C,
            mode: Mode::Major,
        };

        let notes = c_major.notes(Octave::new(4));
        assert_eq!(notes.len(), 7);

        let expected_pitch_classes = [
            PitchClass::C,
            PitchClass::D,
            PitchClass::E,
            PitchClass::F,
            PitchClass::G,
            PitchClass::A,
            PitchClass::B,
        ];

        for (i, note) in notes.iter().enumerate() {
            assert_eq!(note.pitch_cls, expected_pitch_classes[i]);
            assert_eq!(note.octave, Octave::new(4));
        }
    }

    #[test]
    fn test_scale_generation_a_minor() {
        let a_minor = Scale {
            root: PitchClass::A,
            mode: Mode::NaturalMinor,
        };

        let notes = a_minor.notes(Octave::new(4));

        assert_eq!(notes.len(), 7);

        let expected_pitch_classes = [
            PitchClass::A,
            PitchClass::B,
            PitchClass::C,
            PitchClass::D,
            PitchClass::E,
            PitchClass::F,
            PitchClass::G,
        ];

        for (i, note) in notes.iter().enumerate() {
            assert_eq!(note.pitch_cls, expected_pitch_classes[i]);
        }
    }

    #[test]
    fn test_chord_notes_major() {
        let c_major = Chord {
            root: Note {
                pitch_cls: PitchClass::C,
                octave: Octave::new(4),
            },
            kind: ChordKind::Major,
        };

        let notes = c_major.notes();

        assert_eq!(notes.len(), 3);

        assert_eq!(notes[0].pitch_cls, PitchClass::C);
        assert_eq!(notes[0].octave, Octave::new(4));

        assert_eq!(notes[1].pitch_cls, PitchClass::E);
        assert_eq!(notes[1].octave, Octave::new(4));

        assert_eq!(notes[2].pitch_cls, PitchClass::G);
        assert_eq!(notes[2].octave, Octave::new(4));
    }

    #[test]
    fn test_chord_notes_major7() {
        // C Major7 chord
        let c_major7 = Chord {
            root: Note {
                pitch_cls: PitchClass::C,
                octave: Octave::new(4),
            },
            kind: ChordKind::Major7,
        };

        let notes = c_major7.notes();

        assert_eq!(notes.len(), 4);

        assert_eq!(notes[0].pitch_cls, PitchClass::C);
        assert_eq!(notes[0].octave, Octave::new(4));

        assert_eq!(notes[1].pitch_cls, PitchClass::E);
        assert_eq!(notes[1].octave, Octave::new(4));

        assert_eq!(notes[2].pitch_cls, PitchClass::G);
        assert_eq!(notes[2].octave, Octave::new(4));

        assert_eq!(notes[3].pitch_cls, PitchClass::B);
        assert_eq!(notes[3].octave, Octave::new(4));
    }

    #[test]
    fn test_chord_notes_with_octave_wrapping() {
        let a_major = Chord {
            root: Note {
                pitch_cls: PitchClass::A,
                octave: Octave::new(4),
            },
            kind: ChordKind::Major,
        };

        let notes = a_major.notes();

        // A Major chord: A, C#, E
        // A4 = MIDI 69, C#4 = MIDI 61, E4 = MIDI 64
        assert_eq!(notes.len(), 3);
        assert_eq!(notes[0].pitch_cls, PitchClass::A);
        assert_eq!(notes[0].octave, Octave::new(4)); // A4 = MIDI 69

        // A + 4 semitones = MIDI 73 = C#5
        // Since A4 = 69, and we add 4 semitones = 73, which is in octave 5
        assert_eq!(notes[1].pitch_cls, PitchClass::Cs);
        assert_eq!(notes[1].octave, Octave::new(5));

        // A + 7 semitones = MIDI 76 = E5
        assert_eq!(notes[2].pitch_cls, PitchClass::E);
        assert_eq!(notes[2].octave, Octave::new(5));

        let c_major = Chord {
            root: Note {
                pitch_cls: PitchClass::C,
                octave: Octave::new(4),
            },
            kind: ChordKind::Major,
        };

        let c_notes = c_major.notes();
        // C Major: C4, E4, G4 - all should stay in octave 4
        assert_eq!(c_notes[0].octave, Octave::new(4)); // C4 = MIDI 60
        assert_eq!(c_notes[1].octave, Octave::new(4)); // E4 = MIDI 64
        assert_eq!(c_notes[2].octave, Octave::new(4)); // G4 = MIDI 67
    }

    #[test]
    fn test_tempo_marking_conversion() {
        let marking = TempoMarkings::Allegro;
        let tempo: Tempo = marking.into();
        assert_eq!(tempo.0, 144); // Average of Allegro range (120, 168)
    }
}
