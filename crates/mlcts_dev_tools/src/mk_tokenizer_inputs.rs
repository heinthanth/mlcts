use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, serde::Deserialize)]
pub struct MyG2pMlcTsRow
{
  #[allow(dead_code)]
  myanmar_word: String,
  #[allow(dead_code)]
  mlcts_romanization: String,
  #[allow(dead_code)]
  syllable_count: usize,
  myanmar_syllables: String,
  mlcts_syllables: String,
}

fn main()
{
  let sg_syllable_path = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("..")
    .join("mlcts_tokenizer")
    .join("tests")
    .join("inputs_single_syllable.csv");

  let g2p_mlcts_dict = load_g2p_mlcts_dict();
  gen_single_syllable_test_inputs(&sg_syllable_path, &g2p_mlcts_dict);
}

/// Collect single syllables from the G2P MLCTS dictionary.
///
/// # Arguments
///
/// * `dict` - The G2P MLCTS dictionary.
///
/// # Returns
///
/// Unique single syllables from the G2P MLCTS dictionary.
fn collect_single_syllables<'i>(
  dict: &'i Vec<MyG2pMlcTsRow>,
) -> Vec<(&'i str, &'i str)>
{
  dict
    .iter()
    .flat_map(|row| {
      row
        .mlcts_syllables
        .split("|")
        .into_iter()
        .zip(row.myanmar_syllables.split("|"))
        .collect::<Vec<_>>()
    })
    .collect::<HashSet<_>>()
    .into_iter()
    .collect()
}

/// Generate test inputs for single syllable words.
///
/// # Arguments
///
/// * `output_path` - The output path for the test inputs.
/// * `g2p_mlcts_dict` - The G2P MLCTS dictionary.
fn gen_single_syllable_test_inputs(
  output_path: &Path,
  g2p_mlcts_dict: &Vec<MyG2pMlcTsRow>,
)
{
  let mut wtr = csv::Writer::from_path(output_path).unwrap();
  wtr
    .write_record(&[
      "input_class",
      "myanmar_syllable",
      "mlcts_syllable",
      "consonant",
      "medial_diacritic",
      "vowel",
      "virama",
      "tone",
    ])
    .unwrap();

  let syllables = collect_single_syllables(g2p_mlcts_dict);

  // vowel 'a.'
  extract_vowel_and_generate_input(
    &mut wtr,
    &syllables,
    "consonant",
    "a.",
    "A",
    None,
    Some("Creaky"),
    None,
  );

  // vowel 'a'
  extract_vowel_and_generate_input(
    &mut wtr,
    &syllables,
    "consonant",
    "a",
    "A",
    None,
    None,
    Some(|_, mm_input| mm_input != "နျာ"),
  );

  // vowel 'a:' with high tone
  extract_vowel_and_generate_input(
    &mut wtr,
    &syllables,
    "consonant",
    "a:",
    "A",
    None,
    Some("High"),
    Some(|inp: &str, _| !inp.contains("yauka")),
  );

  // vowel 'ak'
  extract_vowel_and_generate_input(
    &mut wtr,
    &syllables,
    "consonant",
    "ak",
    "A",
    Some("K"),
    None,
    None,
  );

  // vowel 'at'
  extract_vowel_and_generate_input(
    &mut wtr,
    &syllables,
    "consonant",
    "at",
    "A",
    Some("T"),
    None,
    None,
  );

  // vowel 'ap'
  extract_vowel_and_generate_input(
    &mut wtr,
    &syllables,
    "consonant",
    "ac",
    "A",
    Some("C"),
    None,
    None,
  );

  // vowel 'ap'
  extract_vowel_and_generate_input(
    &mut wtr,
    &syllables,
    "consonant",
    "ap",
    "A",
    Some("P"),
    None,
    Some(|inp, _| !inp.contains("kywanap")),
  );
}

/// Generate single syllable test inputs based on the certain conditions.
///
/// # Arguments
///
/// * `csv_writer` - The CSV writer
/// * `syllables` - The syllable collection
/// * `input_class` - The input class to generate
/// * `mlcts_vowel` - The MLCTS vowel to be used
/// * `vowel_enum` - The expected vowel enum
/// * `virama` - The expected virama
/// * `tone` - The expected tone
/// * `additional_filter_fn` - Additional filter function to exclude certain
///   syllables
fn extract_vowel_and_generate_input(
  csv_writer: &mut csv::Writer<std::fs::File>,
  syllables: &Vec<(&str, &str)>,
  input_class: &str,
  mlcts_vowel: &str,
  vowel_enum: &str,
  virama: Option<&str>,
  tone: Option<&str>,
  additional_filter_fn: Option<fn(&str, &str) -> bool>,
)
{
  for (mlcts_syllable, myanmar_syllable) in syllables
    .iter()
    .filter(|(mlcts_input, _)| mlcts_input.ends_with(mlcts_vowel))
    .filter(|(mlcts_input, mm_input)| {
      additional_filter_fn
        .map(|f| f(mlcts_input, mm_input))
        .unwrap_or(true)
    })
  {
    let (consonant, medial_diacritic) = extract_consonant_from_mlcts(
      mlcts_syllable,
      myanmar_syllable,
      mlcts_vowel,
    );

    csv_writer
      .write_record(&[
        input_class,
        myanmar_syllable,
        mlcts_syllable,
        consonant.as_str(),
        medial_diacritic.unwrap_or(""),
        vowel_enum,
        virama.unwrap_or(""),
        tone.unwrap_or(""),
      ])
      .unwrap();
  }

  csv_writer.flush().unwrap();
}

/// Captialize the first letter of the string.
///
/// # Arguments
///
/// * `s` - The string to capitalize.
///
/// # Returns
///
/// The capitalized string.
fn capitalize_first_letter(s: &str) -> String
{
  let mut chars = s.chars();
  match chars.next()
  {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
  }
}

/// Extract the consonant from the MLCTS input by
/// removing the vowel, virama, and tone.
///
/// # Arguments
///
/// * `mlcts_input` - The MLCTS input
/// * `mm_input` - The Myanmar input
/// * `vowel` - The vowel to remove
///
/// # Returns
///
/// The consonant and the medial diacritic.
fn extract_consonant_from_mlcts(
  mlcts_input: &str,
  mm_input: &str,
  vowel: &str,
) -> (String, Option<&'static str>)
{
  let without_vowel = mlcts_input.replace(vowel, "");
  let length = without_vowel.len();

  if length == 0
  {
    return ("A".to_string(), None);
  }

  if mm_input.contains("ျွှ")
  {
    // remove 'h' at the beginning and 'yw' at the end
    (
      capitalize_first_letter(&without_vowel[1 .. length - 2]),
      Some("Hyw"),
    )
  }
  else if mm_input.contains("ြွှ")
  {
    // remove 'h' at the beginning and 'rw' at the end
    (
      capitalize_first_letter(&without_vowel[1 .. length - 2]),
      Some("Hrw"),
    )
  }
  else if mm_input.contains("ြွ")
  {
    // remove 'rw' at the end
    (
      capitalize_first_letter(&without_vowel[0 .. length - 2]),
      Some("Rw"),
    )
  }
  else if mm_input.contains("ျွ")
  {
    // remove 'yw' at the end
    (
      capitalize_first_letter(&without_vowel[0 .. length - 2]),
      Some("Yw"),
    )
  }
  else if mm_input.contains("ွှ")
  {
    // remove 'h' at the beginning and 'w' at the end
    (
      capitalize_first_letter(&without_vowel[1 .. length - 1]),
      Some("Hw"),
    )
  }
  else if mm_input.contains("ျှ")
  {
    // remove 'h' at the beginning and 'y' at the end
    (
      capitalize_first_letter(&without_vowel[1 .. length - 1]),
      Some("Hy"),
    )
  }
  else if mm_input.contains("ြှ")
  {
    // remove 'h' at the beginning and 'r' at the end
    (
      capitalize_first_letter(&without_vowel[1 .. length - 1]),
      Some("Hr"),
    )
  }
  else if mm_input.contains("ြ")
  {
    // remove 'r' at the end
    (
      capitalize_first_letter(&without_vowel[0 .. length - 1]),
      Some("R"),
    )
  }
  else if mm_input.contains("ျ")
  {
    // remove 'y' at the end
    (
      capitalize_first_letter(&without_vowel[0 .. length - 1]),
      Some("Y"),
    )
  }
  else if mm_input.contains("ွ")
  {
    // remove 'w' at the end
    (
      capitalize_first_letter(&without_vowel[0 .. length - 1]),
      Some("W"),
    )
  }
  else if mm_input.contains("ှ")
  {
    // remove 'h' at the beginning
    (
      capitalize_first_letter(&without_vowel[1 .. length]),
      Some("H"),
    )
  }
  else
  {
    (capitalize_first_letter(&without_vowel), None)
  }
}

/// Load the G2P MLCTS dictionary.
///
/// # Returns
///
/// A vector of rows from the G2P MLCTS dictionary.
fn load_g2p_mlcts_dict() -> Vec<MyG2pMlcTsRow>
{
  let path = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("..")
    .join("..")
    .join("assets")
    .join("myg2p-dict-mlcts.csv");

  let mut rdr = csv::Reader::from_path(path).unwrap();
  rdr.deserialize().into_iter().map(|r| r.unwrap()).collect()
}
