use std::{fmt, ops::Deref};
use fp_vec::Fpushable;

/// Enum whith IPA vowels.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Vowels {
    CloseBackRounded,
    CloseBackUnrounded,
    CloseCentralRounded,
    CloseCentralUnrounded,
    CloseFrontRounded,
    CloseFrontUnrounded,
    CloseMidFrontRounded,
    CloseMidFrontUnrounded,
    MidCentral,
    NearCloseNearBackRounded,
    NearCloseNearFrontRounded,
    NearCloseNearFrontUnrounded,
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
        use Sound::*;
        use Consonants::*;
        use Vowels::*;
        use Error::*;

        let ipa: Vec<_> = ipa.chars().collect();
        (0..ipa.len()).fold(Ok(Vec::with_capacity(ipa.len())), |acc, i| acc.and_then(|vec| {
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
                'n' => Ok(Some(Consonant { phoneme: VoicedAlveolarNasal,      is_long, is_palatalized })),
                'm' => Ok(Some(Consonant { phoneme: VoicedBilabialNasal,      is_long, is_palatalized })),
                'j' => Ok(Some(Consonant { phoneme: VoicedPalatalApproximant, is_long, is_palatalized })),
                'p' => Ok(Some(Consonant { phoneme: VoicelessBilabialPlosive, is_long, is_palatalized })),

                'u' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: CloseBackRounded,            is_long })) },
                'ɯ' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: CloseBackUnrounded,          is_long })) },
                'ʉ' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: CloseCentralRounded,         is_long })) },
                'ɨ' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: CloseCentralUnrounded,       is_long })) },
                'y' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: CloseFrontRounded,           is_long })) },
                'i' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: CloseFrontUnrounded,         is_long })) },
                'ø' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: CloseMidFrontRounded,        is_long })) },
                'e' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: CloseMidFrontUnrounded,      is_long })) },
                'ə' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: MidCentral,                  is_long })) },
                'ʊ' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: NearCloseNearBackRounded,    is_long })) },
                'ʏ' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: NearCloseNearFrontRounded,   is_long })) },
                'ɪ' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: NearCloseNearFrontUnrounded, is_long })) },
                'æ' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: NearOpenFrontUrounded,       is_long })) },
                'ɑ' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: OpenBackUnrounded,           is_long })) },
                'a' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: OpenFrontUnrounded,          is_long })) },
                'ʌ' => if is_palatalized { Err(PalatalizedVowel(ipa[i])) } else { Ok(Some(Vowel { phoneme: OpenMidBackUnrounded,        is_long })) },

                'ʲ' => Ok(None),
                'ː' => Ok(None),

                ' ' => Ok(Some(Space)),

                _ => Err(NotYetImplemented(ipa[i]))
            }
            .map(|err| err.map(|to_push| (*vec).fpush(to_push)).unwrap_or(vec))
        }))
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
                    format!("{}{}", match phoneme {
                        Vowels::CloseBackRounded            => 'u',
                        Vowels::CloseBackUnrounded          => 'ɯ',
                        Vowels::CloseCentralRounded         => 'ʉ',
                        Vowels::CloseCentralUnrounded       => 'ɨ',
                        Vowels::CloseFrontRounded           => 'y',
                        Vowels::CloseFrontUnrounded         => 'i',
                        Vowels::CloseMidFrontRounded        => 'ø',
                        Vowels::CloseMidFrontUnrounded      => 'e',
                        Vowels::MidCentral                  => 'ə',
                        Vowels::NearCloseNearBackRounded    => 'ʊ',
                        Vowels::NearCloseNearFrontRounded   => 'ʏ',
                        Vowels::NearCloseNearFrontUnrounded => 'ɪ',
                        Vowels::NearOpenFrontUrounded       => 'æ',
                        Vowels::OpenBackUnrounded           => 'ɑ',
                        Vowels::OpenFrontUnrounded          => 'a',
                        Vowels::OpenMidBackUnrounded        => 'ʌ',
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
