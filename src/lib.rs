#![feature(iterator_try_collect)]

//! Crate with IPA sounds. Use it to parse and process IPA.

use std::{fmt, ops::Deref};
use alt_enum::alt_val_enum;

alt_val_enum!(
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
/// Enum for IPA vowels.
pub vowels -> char:
    Close back rounded: 'u',
    Close back unrounded: 'ɯ',
    Close central rounded: 'ʉ',
    Close central unrounded: 'ɨ',
    Close front rounded: 'y',
    Close front unrounded: 'i',
    Close-mid back rounded: 'o',
    Close-mid back unrounded: 'ɤ',
    Close-mid central rounded: 'ɵ',
    Close-mid central unrounded: 'ɘ',
    Close-mid front rounded: 'ø',
    Close-mid front unrounded: 'e',
    Mid central: 'ə',
    Near close near back rounded: 'ʊ',
    Near close near front rounded: 'ʏ',
    Near close near front unrounded: 'ɪ',
    NearOpen front urounded: 'æ',
    Open back unrounded: 'ɑ',
    Open front unrounded: 'a',
    Open mid back unrounded: 'ʌ'
);

alt_val_enum!(
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
/// Enum for IPA consonants.
pub consonants -> char:
    Voiced alveolar nasal: 'n',
    Voiced bilabial nasal: 'm',
    Voiced palatal approximant: 'j',
    Voiceless bilabial plosive: 'p',
);

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
///     format!("{}", ipa_sounds::Ipa::try_from("nʲæ nʲæn").unwrap()),
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
                write!(formatter, "Vowel ({}) cannot be palatalized", vowel)
            },
            Error::NotYetImplemented(symbol) => {
                write!(formatter, "'{}' is not yet implemented", symbol)
            },
        }
    }
}

impl TryFrom<&str> for Ipa {
    type Error = Error;

    fn try_from(ipa: &str) -> Result<Self, Self::Error> {

        let ipa: Vec<_> = ipa.chars().collect();
        (0..ipa.len()).filter_map(|i| {
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
            match ipa[i] {
                'ʲ' | 'ː' => None,

                ' ' => Some(Ok(Sound::Space)),

                ch => {
                    Some(match (Consonants::try_from(ch), Vowels::try_from(ch)) {
                        (Ok(consonant), _) => Ok(
                            Sound::Consonant {
                                phoneme: consonant,
                                is_long,
                                is_palatalized
                            }
                        ),
                        (_, Ok(vowel)) => if is_palatalized {
                                Err(Error::PalatalizedVowel(ch))
                            } else {
                                Ok(Sound::Vowel {
                                    phoneme: vowel,
                                    is_long
                                })
                            },
                        _ => Err(Error::NotYetImplemented(ch))
                    })
                }
            }
        })
        .try_collect()
        .map(Ipa)
    }
}

impl TryFrom<String> for Ipa {
    type Error = Error;

    fn try_from(ipa_string: String) -> Result<Self, Self::Error> {
        Self::try_from(ipa_string.as_str())
    }
}

impl fmt::Display for Ipa {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter().try_for_each(|&sound|
            write!(formatter, "{}", match sound {
                Sound::Vowel { phoneme, is_long } => {
                    format!("{}{}",
                        char::from(phoneme),
                        if is_long {"ː"} else {""}
                    )
                },
                Sound::Consonant { phoneme, is_long, is_palatalized } => {
                    format!("{}{}{}",
                        char::from(phoneme),
                        if is_palatalized {"ʲ"} else {""},
                        if is_long {"ː"} else {""}
                    )
                },
                Sound::Space => " ".to_owned()
            })
        )
    }
}

impl Deref for Ipa {
    type Target = [Sound];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod ipa_build_tests {
    use super::*;

    #[test]
    fn test_nja() {
        assert_eq!(
            Ipa::try_from("nʲæ"),
            Ok(Ipa(vec![
                Sound::Consonant {
                    phoneme: Consonants::VoicedAlveolarNasal,
                    is_long: false,
                    is_palatalized: true
                },
                Sound::Vowel {
                    phoneme: Vowels::NearOpenFrontUrounded,
                    is_long: false
                }
            ]))
        );
        
    }

    #[test]
    fn test_palatalized_vowel() {
        assert_eq!(
            Ipa::try_from("æʲ"),
            Err(Error::PalatalizedVowel('æ'))
        );
        
    }

    #[test]
    fn test_not_implemented() {
        assert_eq!(
            Ipa::try_from("þ"),
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
            Ipa::try_from("nʲæ").map(|ipa| format!("{}", ipa)),
            Ok("nʲæ".to_owned())
        );
        
    }
}
