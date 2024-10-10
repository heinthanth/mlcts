use std::path::Path;

use mlcts::core::*;
use mlcts::tokenizer::*;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct TokenizerInput
{
  class: String,
  input_burmese: String,
  input_mlcts: String,
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
}

/// Get the test inputs for the tokenizer.
fn get_tokenizer_inputs() -> Vec<TokenizerInput>
{
  let test_input_path = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("tests")
    .join("t_tokenizer_inputs_single.csv");

  let mut rdr = csv::Reader::from_path(test_input_path).unwrap();
  rdr.deserialize().into_iter().map(|r| r.unwrap()).collect()
}

/// Helper function to test syllables.
fn test_syllable(input_class: &str)
{
  let consonant_tests = get_tokenizer_inputs()
    .into_iter()
    .filter(|input| input.class == input_class)
    .collect::<Vec<TokenizerInput>>();

  for test in consonant_tests
  {
    let mut tokenizer = Tokenizer::new(&test.input_mlcts);
    let token = tokenizer.next_token();
    let expected_c = Consonant::new(test.consonant, test.medial_diacritic);
    let expected_v = Vowel::new(test.vowel, test.virama, test.tone);
    let expected = TokenKind::Syllable(Syllable::new(expected_c, expected_v));
    assert_eq!(token.kind, expected);
  }
}
