//! Optional experiment proving that the standalone name-generator crate can be consumed on demand.

use name_generator::anatomy::speaker::SpeakerAnatomy;
use name_generator::language::LanguageConfiguration;
use name_generator::phonology::PhonologyConfiguration;
use name_generator::phonology::consonants::pulmonics::{
    B_BILABIAL, M_BILABIAL_VOICELESS, P_BILABIAL,
};
use name_generator::phonology::phonemes::AllowedPhoneme;
use name_generator::phonology::vowels::{A, O, U};
use name_generator::syllables::SyllableConfiguration;
use name_generator::validation::FormattedValidationErrors;

fn main() -> Result<(), FormattedValidationErrors> {
    let phonology = PhonologyConfiguration::new()
        .add_vowels(vec![&A, &O, &U])?
        .add_consonants(vec![&P_BILABIAL, &B_BILABIAL, &M_BILABIAL_VOICELESS])?;
    let allowed_consonants = vec![
        AllowedPhoneme {
            phoneme: P_BILABIAL.name.to_string(),
            weight: 1.0,
        },
        AllowedPhoneme {
            phoneme: B_BILABIAL.name.to_string(),
            weight: 1.0,
        },
        AllowedPhoneme {
            phoneme: M_BILABIAL_VOICELESS.name.to_string(),
            weight: 1.0,
        },
    ];
    let syllables = SyllableConfiguration::new()
        .add_pattern("Cv", 0.1)?
        .add_pattern("CVC", 0.2)?
        .set_onset(allowed_consonants.clone(), vec![], vec![])?
        .set_nucleus(
            vec![
                AllowedPhoneme {
                    phoneme: A.name.to_string(),
                    weight: 1.0,
                },
                AllowedPhoneme {
                    phoneme: O.name.to_string(),
                    weight: 1.0,
                },
            ],
            vec![],
            vec![],
            vec![],
            vec![],
        )?
        .set_coda(allowed_consonants, vec![], vec![])?;

    let language = LanguageConfiguration::new("Stellar naming experiment")
        .set_anatomy(SpeakerAnatomy::human())?
        .set_phonology(phonology)?
        .set_syllables(syllables)?;

    println!("{language:#?}");
    Ok(())
}
