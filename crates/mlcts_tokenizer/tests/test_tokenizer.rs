#![recursion_limit = "1000"]

#[cfg(test)]
mod tokenizer
{
  use mlcts_core::*;
  use mlcts_proc_macros::gen_tokenizer_single_syllable_tests;
  use mlcts_tokenizer::*;
  use rstest::rstest;

  gen_tokenizer_single_syllable_tests!();
}
