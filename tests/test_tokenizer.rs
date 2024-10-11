use std::path::Path;

use lazy_static::lazy_static;
use mlcts::core::*;
use mlcts::tokenizer::*;

lazy_static! {
  static ref consonant_tests: Vec<TokenizerInput> =
    get_tokenizer_single_syllable_inputs()
      .into_iter()
      .collect::<Vec<TokenizerInput>>();
}

#[derive(Debug, serde::Deserialize)]
struct TokenizerInput
{
  input_class: String,
  #[allow(dead_code)]
  myanmar_syllable: String,
  mlcts_syllable: String,
  consonant: BasicConsonant,
  medial_diacritic: Option<MedialDiacritic>,
  vowel: BasicVowel,
  virama: Option<Virama>,
  tone: Option<Tone>,
}

#[cfg(test)]
mod tests
{
  use super::*;

  #[test]
  fn test_basic_consonants()
  {
    test_syllable("basic_consonant");
  }

  #[test]
  fn test_vowel_a()
  {
    test_syllable("vowel_a");
  }

  #[test]
  fn test_vowel_a_high()
  {
    test_syllable("vowel_a_high");
  }

  #[test]
  fn test_vowel_ak()
  {
    test_syllable("vowel_ak");
  }

  #[test]
  fn test_vowel_ac()
  {
    test_syllable("vowel_ac");
  }

  #[test]
  fn test_vowel_at()
  {
    test_syllable("vowel_at");
  }

  #[test]
  fn test_vowel_ap()
  {
    test_syllable("vowel_ap");
  }
}

/// Get the test inputs for the tokenizer.
fn get_tokenizer_single_syllable_inputs() -> Vec<TokenizerInput>
{
  let test_input_path = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("tests")
    .join("t_tokenizer_inputs_single_syllable.csv");

  let mut rdr = csv::Reader::from_path(test_input_path).unwrap();
  rdr.deserialize().into_iter().map(|r| r.unwrap()).collect()
}

/// Helper function to test syllables.
fn test_syllable(input_class: &str)
{
  for test in consonant_tests
    .iter()
    .filter(|input| input.input_class == input_class)
  {
    let expected_c = Consonant::new(test.consonant, test.medial_diacritic);
    let expected_v = Vowel::new(test.vowel, test.virama, test.tone);
    let expected = TokenKind::Syllable(Syllable::new(expected_c, expected_v));

    let mut tokenizer = Tokenizer::new(&test.mlcts_syllable);
    let token = tokenizer.next_token();

    assert_eq!(token.kind, expected);
  }
}
