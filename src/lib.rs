use std::{fmt, ops::Deref};

/// Enum whith IPA vowels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Vowels {
    CloseBackRounded,
    CloseFrontRounded,
    CloseFrontUnrounded,
    CloseMidFrontRounded,
    MidCentral,
    NearOpenFrontUrounded,
    OpenBackUnrounded,
    OpenFrontUnrounded,
    OpenMidBackUnrounded
}

/// Enum whith IPA consonants.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Consonants {
    VoicedAlveolarNasal,
    VoicedBilabialNasal,
    VoicedPalatalApproximant,
    VoicelessBilabialPlosive
}

/// Enum for IPA sounds.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Sound {
    Vowel { phoneme: Vowels, is_long: bool },
    Consonant { phoneme: Consonants, is_long: bool, is_palatalized: bool },
    Space
}

/// Struct containing a sequence of IPA sounds.
///
/// # Examples
///
/// ```
/// assert_eq!(
///     format!("{}", ipa_sounds::Ipa::from("nʲæ nʲæn")),
///     "nʲæ nʲæn"
/// )
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ipa(Vec<Sound>);

/// Enum for possible errors when constructing a sequence of IPA sounds from &str.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Error {
    PalatalizedVowel(char),
    NotYetImplemented(char)
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PalatalizedVowel(vowel) => {
                write!(formatter, "Vowel ({}) cannot be palatalized", vowel)?;
            },
            Error::NotYetImplemented(symbol) => {
                write!(formatter, "'{}' is not yet implemented", symbol)?;
            },
        }
        Ok(())
    }
}

impl Ipa {
    pub fn new(ipa: &str) -> Result<Self, Error> {
        use Sound::*;
        use Consonants::*;
        use Vowels::*;
        use Error::*;

        let ipa: Vec<_> = ipa.chars().collect();
        let mut vec = Vec::with_capacity(ipa.len());
        for i in 0..ipa.len() {
            let is_palatalized = if i == ipa.len() - 1 {
                false
            } else {
                matches!(ipa[i + 1], 'ʲ')
            };
            let is_long = if i == ipa.len() - 1 {
                false
            } else if i < ipa.len() - 2 && is_palatalized {
                matches!(ipa[i + 2], 'ː')
            } else {
                matches!(ipa[i + 1], 'ː')
            };
            let sound = match ipa[i] {
                'n' => Some(Consonant { phoneme: VoicedAlveolarNasal,      is_long, is_palatalized }),
                'm' => Some(Consonant { phoneme: VoicedBilabialNasal,      is_long, is_palatalized }),
                'j' => Some(Consonant { phoneme: VoicedPalatalApproximant, is_long, is_palatalized }),
                'p' => Some(Consonant { phoneme: VoicelessBilabialPlosive, is_long, is_palatalized }),

                'u' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: CloseBackRounded,      is_long }) },
                'y' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: CloseFrontRounded,     is_long }) },
                'i' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: CloseFrontUnrounded,   is_long }) },
                'ø' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: CloseMidFrontRounded,  is_long }) },
                'ə' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: MidCentral,            is_long }) },
                'æ' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: NearOpenFrontUrounded, is_long }) },
                'ɑ' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: OpenBackUnrounded,     is_long }) },
                'a' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: OpenFrontUnrounded,    is_long }) },
                'ʌ' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: OpenMidBackUnrounded,  is_long }) },

                'ʲ' => None,
                'ː' => None,

                ' ' => Some(Space),

                _ => { return Err(NotYetImplemented(ipa[i])) }
            };
            if let Some(to_push) = sound {
                vec.push(to_push);
            }
        }
        Ok(Ipa(vec))
    }
}

impl fmt::Display for Ipa {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for sound in &self.0 {
            write!(formatter, "{}", match *sound {
                Sound::Vowel { phoneme, is_long } => {
                    format!("{}{}", match phoneme {
                        Vowels::CloseBackRounded      => 'u',
                        Vowels::CloseFrontRounded     => 'y',
                        Vowels::CloseFrontUnrounded   => 'i',
                        Vowels::CloseMidFrontRounded  => 'ø',
                        Vowels::MidCentral            => 'ə',
                        Vowels::NearOpenFrontUrounded => 'æ',
                        Vowels::OpenBackUnrounded     => 'ɑ',
                        Vowels::OpenFrontUnrounded    => 'a',
                        Vowels::OpenMidBackUnrounded  => 'ʌ',
                    }, if is_long {"ː"} else {""})
                },
                Sound::Consonant { phoneme, is_long, is_palatalized } => {
                    format!("{}{}{}", match phoneme {
                        Consonants::VoicedAlveolarNasal      => 'n',
                        Consonants::VoicedBilabialNasal      => 'm',
                        Consonants::VoicedPalatalApproximant => 'j',
                        Consonants::VoicelessBilabialPlosive => 'p',
                    }, if is_palatalized {"ʲ"} else {""},
                    if is_long {"ː"} else {""})
                },
                Sound::Space => " ".to_owned()
            })?;
        }
        Ok(())
    }
}

impl Deref for Ipa {
    type Target = [Sound];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Ipa {
    fn from(ipa_str: &str) -> Self {
        Self::new(ipa_str).unwrap()
    }
}

impl From<String> for Ipa {
    fn from(ipa_string: String) -> Self {
        Self::from(ipa_string.as_str())
    }
}

#[cfg(test)]
mod ipa_build_tests {
    use super::*;

    #[test]
    fn test_nja() {
        assert_eq!(
            Ipa::from("nʲæ"),
            Ipa(vec![
                Sound::Consonant {
                    phoneme: Consonants::VoicedAlveolarNasal,
                    is_long: false,
                    is_palatalized: true
                },
                Sound::Vowel {
                    phoneme: Vowels::NearOpenFrontUrounded,
                    is_long: false
                }
            ])
        );
        
    }

    #[test]
    fn test_palatalized_vowel() {
        assert_eq!(
            Ipa::new("æʲ"),
            Err(Error::PalatalizedVowel('æ'))
        );
        
    }

    #[test]
    fn test_not_implemented() {
        assert_eq!(
            Ipa::new("þ"),
            Err(Error::NotYetImplemented('þ'))
        );
        
    }
}

#[cfg(test)]
mod ipa_fmt_tests {
    use super::*;

    #[test]
    fn test_nja() {
        assert_eq!(
            format!("{}", Ipa::new("nʲæ").unwrap()),
            "nʲæ"
        );
        
    }
}
