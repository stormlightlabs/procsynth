//! `procsynth_core` contains core music theory types for melody construction

// NOTE: Is there a convention for as_* vs to_* methods in Rust?
// NOTE: What does `Box` do?
// NOTE: '& static vs Box
// NOTE: How do we go from enum members to a byte/integer type?

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PitchClass {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Octave(i8);

impl Octave {
    pub fn num(self) -> i8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Note {
    pub pitch_cls: PitchClass,
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
    /// Converts the [Note] to a MIDI number, which can be 0 - 127
    pub fn as_midi_number(self) -> u8 {
        self.into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Duration {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    Dotted(&'static Duration),
    Triplet(&'static Duration),
    Custom(f32),
}

/// Named dynamic markings to map to MIDI velocity.
/// MIDI Velocity indicates the intensity of a loudness of a note (0 - 127)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dynamic {
    Pianissimo,
    Piano,
    MezzoPiano,
    MezzoForte,
    Forte,
    Fortissimo,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Note(Note, Duration, Dynamic),
    Rest(Duration),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Ionian,
    Major,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    NaturalMinor,
    Locrian,
    Custom(&'static [u8; 7]),
}

impl Mode {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale {
    pub root: PitchClass,
    pub mode: Mode,
}

impl Scale {
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

/// Represents chord quality
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordKind {
    Major,
    Minor,
    Diminished,
    Augmented,
    Major7,
    Minor7,
    Dominant7,
    Custom(&'static [u8]),
}

impl ChordKind {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chord {
    pub root: Note,
    pub kind: ChordKind,
}

impl Chord {
    pub fn notes(&self) -> Vec<Note> {
        self.kind
            .intervals()
            .iter()
            .map(|i| {
                let midi_num = self.root.as_midi_number() + i;
                let octave = Octave(self.root.octave.0 + (midi_num / 12) as i8);

                Note {
                    pitch_cls: PitchClass::from(midi_num % 12),
                    octave,
                }
            })
            .collect()
    }
}

pub type Key = Scale;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimeSignature(pub u8, pub u8);

/// Represents BPM
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tempo(u16);

// FIXME: Write all of these tests
#[cfg(test)]
mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn note_to_midi_number() {
        todo!()
    }

    #[test]
    fn duration_as_beats() {
        todo!()
    }

    // TODO: Use C Major
    #[test]
    fn scale_generation() {
        todo!()
    }

    // TODO: Use Major7
    #[test]
    fn chord_notes() {
        todo!()
    }
}
