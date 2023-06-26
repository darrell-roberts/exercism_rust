#[derive(Debug)]
pub enum Error {
    InvalidTonic(String),
}

pub struct Scale<'a> {
    pitch: Pitch,
    intervals: Option<&'a str>,
    tonic: String,
}

impl<'a> Scale<'a> {
    const SHARP: &'static [&'static str] = &[
        "A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#",
    ];
    const FLAT: &'static [&'static str] = &[
        "A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab",
    ];

    fn sharps(&'a self) -> impl Iterator<Item = String> + '_ {
        Self::SHARP
            .iter()
            .cycle()
            .copied()
            .skip_while(|&note| note != self.tonic.as_str())
            .map(|note| note.to_string())
    }

    fn flats(&'a self) -> impl Iterator<Item = String> + '_ {
        Self::FLAT
            .iter()
            .cycle()
            .copied()
            .skip_while(|&note| note != self.tonic.as_str())
            .map(|note| note.to_string())
    }

    pub fn new(tonic: &'a str, intervals: &'a str) -> Result<Scale<'a>, Error> {
        Ok(Scale {
            tonic: upper_tonic(tonic)?,
            pitch: Pitch::try_from(tonic)?,
            intervals: Some(intervals),
        })
    }

    pub fn chromatic(tonic: &'a str) -> Result<Scale<'a>, Error> {
        Ok(Self {
            tonic: upper_tonic(tonic)?,
            pitch: Pitch::try_from(tonic)?,
            intervals: None,
        })
    }

    pub fn enumerate(&self) -> Vec<String> {
        match (&self.pitch, self.intervals.as_ref()) {
            (Pitch::Sharp | Pitch::Natural, None) => self.sharps().take(13).collect(),
            (Pitch::Flat, None) => self.flats().take(13).collect(),
            (Pitch::Sharp | Pitch::Natural, Some(interval)) => {
                let mut result = vec![self.tonic.to_string()];
                let notes = self.sharps().skip(1);
                walk_intervals(notes, interval.chars(), &mut result);
                result
            }
            (Pitch::Flat, Some(interval)) => {
                let mut result = vec![self.tonic.to_string()];
                let notes = self.flats().skip(1);
                walk_intervals(notes, interval.chars(), &mut result);
                result
            }
        }
    }
}

enum Pitch {
    Sharp,
    Flat,
    Natural,
}

impl TryFrom<&str> for Pitch {
    type Error = Error;
    fn try_from(tonic: &str) -> Result<Self, Self::Error> {
        is_sharp(tonic)
            .then_some(Pitch::Sharp)
            .or_else(|| is_flat(tonic).then_some(Pitch::Flat))
            .or_else(|| is_natural(tonic).then_some(Pitch::Natural))
            .ok_or_else(|| Error::InvalidTonic(tonic.to_string()))
    }
}

fn is_sharp(tonic: &str) -> bool {
    const SHARP_TONICS: &[&str] = &[
        "G", "D", "A", "E", "B", "F#", "e", "b", "f#", "c#", "g#", "d#",
    ];

    SHARP_TONICS.iter().any(|&note| note == tonic)
}

fn is_flat(tonic: &str) -> bool {
    const FLAT_TONICS: &[&str] = &[
        "F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb",
    ];

    FLAT_TONICS.iter().any(|&note| note == tonic)
}

fn is_natural(tonic: &str) -> bool {
    tonic == "C" || tonic == "a"
}

fn walk_intervals(
    mut notes: impl Iterator<Item = String>,
    interval: impl Iterator<Item = char>,
    result: &mut Vec<String>,
) {
    for c in interval {
        match c {
            'M' => {
                notes.next();
                result.extend(notes.next().into_iter());
            }
            'A' => {
                notes.next();
                notes.next();
                result.extend(notes.next().into_iter());
            }
            'm' => {
                result.extend(notes.next().into_iter());
            }
            _ => (),
        }
    }
}

fn upper_tonic(s: &str) -> Result<String, Error> {
    let mut chars = s.chars();
    let first = chars.next().map(|c| c.to_uppercase().collect::<String>());
    let second = chars.next();
    match (first, second) {
        (Some(f), None) => Ok(f),
        (Some(mut f), Some(s)) => {
            f.push(s);
            Ok(f)
        }
        _ => Err(Error::InvalidTonic(s.to_string())),
    }
}
