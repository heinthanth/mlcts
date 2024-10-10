#[cfg(test)]
mod tests
{
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

  fn get_tokenizer_inputs() -> Vec<TokenizerInput>
  {
    let test_input_path = Path::new(env!("CARGO_MANIFEST_DIR"))
      .join("tests")
      .join("t_tokenizer_inputs_single.csv");

    let mut rdr = csv::Reader::from_path(test_input_path).unwrap();
    rdr.deserialize().into_iter().map(|r| r.unwrap()).collect()
  }

  #[test]
  fn test_basic_consonants()
  {
    let consonant_tests = get_tokenizer_inputs()
      .into_iter()
      .filter(|input| input.class == "consonant")
      .collect::<Vec<TokenizerInput>>();

    for test in consonant_tests
    {
      let mut tokenizer = Tokenizer::new(&test.input_mlcts);
      let token = tokenizer.next_token();

      assert_eq!(
        token.kind,
        TokenKind::Syllable(Syllable::new(
          Consonant::new(test.consonant, test.medial_diacritic),
          Vowel::new(test.vowel, test.virama, test.tone)
        ))
      );
    }
  }
}
