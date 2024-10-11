#![feature(proc_macro_quote)]

use std::path::Path;

use csv::StringRecord;
use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

/// Helper function to read CSV rows from a file.
/// Path are relative to the workspace root crates
fn read_csv_rows(path: &str) -> Vec<StringRecord>
{
  let test_input_path =
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join(path);

  let mut rdr = csv::Reader::from_path(test_input_path).unwrap();
  rdr.records().into_iter().map(|r| r.unwrap()).collect()
}

/// Generates test cases for the tokenizer using the single-syllable test cases.
/// Basically, this macro reads the CSV file containing the test cases and
/// generates a test function for each row in the CSV file.
#[proc_macro]
pub fn gen_tokenizer_single_syllable_tests(_input: TokenStream) -> TokenStream
{
  let rs_test_cases =
    read_csv_rows("mlcts_tokenizer/tests/inputs_single_syllable.csv")
      .into_iter()
      .map(|row| {
        let i = row.get(2).unwrap();
        let c = row.get(3).unwrap();
        let m = row.get(4).unwrap();
        let v = row.get(5).unwrap();
        let vi = row.get(6).unwrap();
        let t = row.get(7).unwrap();

        let ident_str = i.replace(".", "_dot").replace(":", "_colon");
        let ident =
          Ident::new(ident_str.as_str(), proc_macro2::Span::call_site());

        quote! {
          #[case::#ident(#i, (#c, #m, #v, #vi, #t))]
        }
      });

  quote! {
    #[rstest]
    #(#rs_test_cases)*
    fn single_syllable(#[case] input: &str, #[case] expected: (&str, &str, &str, &str, &str)) {
        let consonant: BasicConsonant = serde_plain::from_str(expected.0).unwrap();
        let vowel: BasicVowel = serde_plain::from_str(expected.2).unwrap();
        let medial: Option<MedialDiacritic> = serde_plain::from_str(expected.1).unwrap();
        let virama: Option<Virama> =  serde_plain::from_str(expected.3).unwrap();
        let tone: Option<Tone> = serde_plain::from_str(expected.4).unwrap();

        let expected_consonant = Consonant::new(consonant, medial);
        let expected_vowel = Vowel::new(vowel, virama, tone);
        let expected_syllable = Syllable::new(expected_consonant, expected_vowel);

        let mut tokenizer = Tokenizer::new(input);
        let next_token = tokenizer.next_token();
        assert_eq!(next_token.kind, TokenKind::Syllable(expected_syllable));
    }
  }
  .into()
}
