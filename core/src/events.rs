use crate::{Duration, Dynamic, Note};

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// A musical note with pitch, duration, and dynamics
    Note(Note, Duration, Dynamic),
    /// A rest (silence) with duration
    Rest(Duration),
}

pub trait Generator {
    /// Generates a sequence of musical events
    fn generate(&self) -> Vec<Event>;
}

#[cfg(test)]
mod tests {
    use super::{
        super::{Octave, PitchClass},
        *,
    };

    #[test]
    fn event_creation() {
        match Event::Note(
            Note {
                pitch_cls: PitchClass::C,
                octave: Octave::new(4),
            },
            Duration::Quarter,
            Dynamic::MezzoForte,
        ) {
            Event::Note(note, duration, dynamic) => {
                assert_eq!(note.pitch_cls, PitchClass::C);
                assert_eq!(note.octave, Octave::new(4));
                assert!(matches!(duration, Duration::Quarter));
                assert_eq!(dynamic, Dynamic::MezzoForte);
            }
            _ => panic!("Expected Note event"),
        }

        match Event::Rest(Duration::Half) {
            Event::Rest(duration) => {
                assert!(matches!(duration, Duration::Half));
            }
            _ => panic!("Expected Rest event"),
        }
    }
}
