//! # mlcts_from_my
//!
//! This crate provides function to generate MLCTS text from Myanmar text.
//! Moreover, this also provides utilities functions such as syllable breaker,
//! etc.

use fancy_regex::Regex;
use mlcts_core::{consonant, vowel, BasicConsonant, Consonant, MedialDiacritic, Syllable, Tone, Virama};

pub fn from_my<'i>(input: &'i str) -> String
{
  split_syllables(input)
    .into_iter()
    .map(|s| {
      let r = parse_syllable(s);
      if r.is_err()
      {
        return s.to_string();
      }
      r.unwrap().0.to_mlcts()
    })
    .collect::<Vec<String>>()
    .join("")
}

const EOF_CHAR: char = '\0';

/// Parse the consonant from the input.
///
/// # Arguments
///
/// * `input` - The input to parse.
///
/// # Returns
///
/// The parsed consonant and the length of bytes used by the consonant.
fn parse_consonant(input: &str) -> Result<(Consonant, usize), &str>
{
  let mut cursor = input.chars();

  let consonant: Result<BasicConsonant, _> =
    BasicConsonant::from_my_alphabet(cursor.next().unwrap_or_else(|| EOF_CHAR));

  if consonant.is_err()
  {
    return Err(input);
  }

  let consonant = consonant.unwrap();
  let consonant = if consonant == BasicConsonant::A
  {
    consonant!(A)
  }
  else
  {
    let medial_pos_1 = cursor.clone().next().unwrap_or_else(|| EOF_CHAR);
    let medial_pos_2 = cursor.clone().nth(1).unwrap_or_else(|| EOF_CHAR);
    let medial_pos_3 = cursor.clone().nth(2).unwrap_or_else(|| EOF_CHAR);

    const MEDIAL_Y: char = 'ျ';
    const MEDIAL_R: char = 'ြ';
    const MEDIAL_W: char = 'ွ';
    const MEDIAL_H: char = 'ှ';

    match (medial_pos_1, medial_pos_2, medial_pos_3)
    {
      // r+w+h
      (MEDIAL_R, MEDIAL_W, MEDIAL_H) =>
      {
        cursor.next();
        cursor.next();
        cursor.next();
        Consonant::with_medial(consonant, MedialDiacritic::Hrw)
      }
      // y+w+h
      (MEDIAL_Y, MEDIAL_W, MEDIAL_H) =>
      {
        cursor.next();
        cursor.next();
        cursor.next();
        Consonant::with_medial(consonant, MedialDiacritic::Hyw)
      }
      // w+h
      (MEDIAL_W, MEDIAL_H, _) =>
      {
        cursor.next();
        cursor.next();
        Consonant::with_medial(consonant, MedialDiacritic::Hw)
      }
      // r+w
      (MEDIAL_R, MEDIAL_W, _) =>
      {
        cursor.next();
        cursor.next();
        Consonant::with_medial(consonant, MedialDiacritic::Rw)
      }
      // y+w
      (MEDIAL_Y, MEDIAL_W, _) =>
      {
        cursor.next();
        cursor.next();
        Consonant::with_medial(consonant, MedialDiacritic::Yw)
      }
      // r+h
      (MEDIAL_R, MEDIAL_H, _) =>
      {
        cursor.next();
        cursor.next();
        Consonant::with_medial(consonant, MedialDiacritic::Hr)
      }
      // y+h
      (MEDIAL_Y, MEDIAL_H, _) =>
      {
        cursor.next();
        cursor.next();
        Consonant::with_medial(consonant, MedialDiacritic::Hy)
      }
      // w
      (MEDIAL_W, ..) =>
      {
        cursor.next();
        Consonant::with_medial(consonant, MedialDiacritic::W)
      }
      // r
      (MEDIAL_R, ..) =>
      {
        cursor.next();
        Consonant::with_medial(consonant, MedialDiacritic::R)
      }
      // y
      (MEDIAL_Y, ..) =>
      {
        cursor.next();
        Consonant::with_medial(consonant, MedialDiacritic::Y)
      }
      // h
      (MEDIAL_H, ..) =>
      {
        cursor.next();
        Consonant::with_medial(consonant, MedialDiacritic::H)
      }
      // no medial diacritic
      _ => Consonant::simple(consonant),
    }
  };

  let consumed_len = input.len() - cursor.as_str().len();
  Ok((consonant, consumed_len))
}

/// Parse the syllable from the input.
///
/// # Arguments
///
/// * `syllable` - Possible syllable input
///
/// # Returns
///
/// The parsed syllable and the length of bytes used by the syllable.
pub fn parse_syllable<'i>(
  syllable: &'i str,
) -> Result<(Syllable, usize), &'i str>
{
  let (consonant, consumed_len) = parse_consonant(syllable)?;

  // if the consumed length is equal to the syllable length,
  if consumed_len == syllable.len()
  {
    return Ok((Syllable::new(consonant, vowel!(A), None), consumed_len));
  }

  let mut cursor = syllable[consumed_len ..].chars();

  let v_pos_1 = cursor.clone().next().unwrap_or_else(|| EOF_CHAR);
  let v_pos_2 = cursor.clone().nth(1).unwrap_or_else(|| EOF_CHAR);
  let v_pos_3 = cursor.clone().nth(2).unwrap_or_else(|| EOF_CHAR);

  const VOW_E: char = 'ေ';
  const VOW_A: char = 'ာ';
  const ASAT: char = '်';
  const VOW_AI_H: char = 'ဲ';
  const TONE_CREAKY: char = '့';
  const TONE_HIGH: char = 'း';
  const VOW_I: char = 'ီ';
  const VOW_I_CREAKY: char = 'ိ';
  const VOW_U_CREAKY: char = 'ု';
  const VOW_U: char = 'ူ';

  let (mut vowel, consumed_len) = match (v_pos_1, v_pos_2, v_pos_3)
  {
    // e.g. ား
    (VOW_A, TONE_HIGH, EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(A; High), None),
        consumed_len + 2,
      ));
    }
    // e.g. ာ EOF
    (VOW_A, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      return Ok((Syllable::new(consonant, vowel!(A), None), consumed_len + 1));
    }
    // e.g. ာ
    (VOW_A, ..) =>
    {
      cursor.next();
      (vowel!(A), consumed_len + 1)
    }
    // e.g. ယ်
    ('ယ', ASAT, EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(Ai), None),
        consumed_len + 2,
      ));
    }
    // e.g. ဲ့
    (VOW_AI_H, TONE_CREAKY, EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(Ai; Creaky), None),
        consumed_len + 2,
      ));
    }
    // e.g. ဲ
    (VOW_AI_H, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(Ai; High), None),
        consumed_len + 1,
      ));
    }
    // e.g. ော်
    (VOW_E, VOW_A, ASAT) =>
    {
      cursor.next();
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(Au), None),
        consumed_len + 3,
      ));
    }
    // e.g. ော့
    (VOW_E, VOW_A, TONE_CREAKY) =>
    {
      cursor.next();
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(Au; Creaky), None),
        consumed_len + 3,
      ));
    }
    // e.g. ော EOF
    (VOW_E, VOW_A, EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(Au), None),
        consumed_len + 2,
      ));
    }
    // e.g. ော
    (VOW_E, VOW_A, ..) =>
    {
      cursor.next();
      cursor.next();
      (vowel!(Au), consumed_len + 2) // need to check virama and stacked
                                     // consonant
    }
    // e.g. ူ
    (VOW_U, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      return Ok((Syllable::new(consonant, vowel!(U), None), consumed_len + 1));
    }
    // e.g. ူး
    (VOW_U, TONE_HIGH, EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(U; High), None),
        consumed_len + 2,
      ));
    }
    // e.g. ု EOF
    (VOW_U, TONE_CREAKY, EOF_CHAR) =>
    {
      cursor.next();
      return Ok((Syllable::new(consonant, vowel!(U), None), consumed_len + 1));
    }
    // e.g. ု
    (VOW_U_CREAKY, ..) =>
    {
      cursor.next();
      // need to check virama and stacked consonant
      (vowel!(U; Creaky), consumed_len + 1)
    }
    // e.g. ိုး
    (VOW_I_CREAKY, VOW_U, TONE_HIGH) =>
    {
      cursor.next();
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(Ui; High), None),
        consumed_len + 3,
      ));
    }
    // e.g. ို့
    (VOW_I_CREAKY, VOW_U, TONE_CREAKY) =>
    {
      cursor.next();
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(Ui; Creaky), None),
        consumed_len + 3,
      ));
    }
    // e.g. ို EOF
    (VOW_I_CREAKY, VOW_U, EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(Ui), None),
        consumed_len + 2,
      ));
    }
    // e.g. ို
    (VOW_I_CREAKY, VOW_U, ..) =>
    {
      cursor.next();
      cursor.next();
      (vowel!(Ui), consumed_len + 2) // need to check virama and stacked
                                     // consonant
    }
    // e.g. ီ
    (VOW_I, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      return Ok((Syllable::new(consonant, vowel!(I), None), consumed_len + 1));
    }
    // e.g. ီး
    (VOW_I, TONE_HIGH, EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(I; High), None),
        consumed_len + 2,
      ));
    }
    // e.g. ိ EOF
    (VOW_I_CREAKY, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      return Ok((Syllable::new(consonant, vowel!(I), None), consumed_len + 1));
    }
    // e.g. ိ
    (VOW_I_CREAKY, ..) =>
    {
      cursor.next();
      (vowel!(I), consumed_len + 1) // need to check virama and stacked
                                    // consonant
    }
    // e.g. ေး
    (VOW_E, TONE_HIGH, EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(E; High), None),
        consumed_len + 2,
      ));
    }
    // e.g. ေ့
    (VOW_E, TONE_CREAKY, EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      return Ok((
        Syllable::new(consonant, vowel!(E; Creaky), None),
        consumed_len + 2,
      ));
    }
    // e.g. ေ EOF
    (VOW_E, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      return Ok((Syllable::new(consonant, vowel!(E), None), consumed_len + 1));
    }
    // e.g. ေ
    (VOW_E, ..) =>
    {
      cursor.next();
      (vowel!(E), consumed_len + 1) // need to check virama and stacked
                                    // consonant
    }
    // the rest
    // need to check virama and stacked consonant
    _ => (vowel!(A), consumed_len),
  };

  // almost all possible ending of a syllable
  // is already handled above, so it's safe to assume
  // that next character is a consonant.
  let top_consonant = cursor.next().unwrap_or_else(|| EOF_CHAR);
  let sign = cursor.clone().next().unwrap_or_else(|| EOF_CHAR);
  let virama_sign_or_bottom_consonant =
    cursor.clone().nth(1).unwrap_or_else(|| EOF_CHAR);
  let extra_char = cursor.clone().nth(2).unwrap_or_else(|| EOF_CHAR);

  const STACK_SIGN: char = '္';

  match (
    top_consonant,
    sign,
    virama_sign_or_bottom_consonant,
    extra_char,
  )
  {
    ('က', ASAT, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::K);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 2));
    }
    ('က', STACK_SIGN, 'က' | 'ခ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::K);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('ဂ', STACK_SIGN, 'ဂ' | 'ဃ', ..) =>
    {
      vowel.virama = Some(Virama::G);
      cursor.next();
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('င', ASAT, STACK_SIGN, EOF_CHAR) =>
    {
      // this is invalid
      return Err(syllable);
    }
    ('င', ASAT, STACK_SIGN, ..) =>
    {
      vowel.virama = Some(Virama::Ng);
      cursor.next();
      cursor.next();

      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('င', ASAT, ':', EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      vowel.virama = Some(Virama::Ng);
      vowel.tone = Some(Tone::High);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 3));
    }
    ('င', ASAT, '.', EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      vowel.virama = Some(Virama::Ng);
      vowel.tone = Some(Tone::Creaky);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 3));
    }
    ('စ', ASAT, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::C);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 2));
    }
    ('စ', STACK_SIGN, 'စ' | 'ဆ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::C);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('ဇ', STACK_SIGN, 'ဇ' | 'ဈ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::J);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('ည' | 'ဉ', ASAT, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::Ny);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 2));
    }
    ('ည' | 'ဉ', ASAT, '.', EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      vowel.virama = Some(Virama::Ny);
      vowel.tone = Some(Tone::Creaky);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 3));
    }
    ('ည' | 'ဉ', ASAT, ':', EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      vowel.virama = Some(Virama::Ny);
      vowel.tone = Some(Tone::High);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 3));
    }
    ('ည' | 'ဉ', STACK_SIGN, 'စ' | 'ဇ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::Ny);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('ဋ', ASAT, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::T);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 2));
    }
    ('ဋ', STACK_SIGN, 'ဋ' | 'ဌ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::T);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('ဍ', STACK_SIGN, 'ဍ' | 'ဎ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::D);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('ဏ', ASAT, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::N);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 2));
    }
    ('ဏ', STACK_SIGN, 'ဍ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::N);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('တ', ASAT, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::T);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 2));
    }
    ('တ', STACK_SIGN, 'တ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::T);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('ထ', STACK_SIGN, 'ထ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::Ht);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('ဒ', STACK_SIGN, 'ဒ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::D);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('န', ASAT, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::N);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 2));
    }
    ('န', ASAT, ':', EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      vowel.virama = Some(Virama::N);
      vowel.tone = Some(Tone::High);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 3));
    }
    ('န', ASAT, '.', EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      vowel.virama = Some(Virama::N);
      vowel.tone = Some(Tone::Creaky);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 3));
    }
    ('န', STACK_SIGN, 'တ' | 'ထ' | 'ဒ' | 'ဓ' | 'န', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::N);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('ပ', ASAT, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      vowel.virama = Some(Virama::P);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 2));
    }
    ('ပ', STACK_SIGN, 'ပ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::P);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('ဗ', STACK_SIGN, 'ဗ' | 'ဘ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::B);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('မ', ASAT, EOF_CHAR, EOF_CHAR) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::M);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 2));
    }
    ('မ', ASAT, ':', EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      vowel.virama = Some(Virama::M);
      vowel.tone = Some(Tone::High);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 3));
    }
    ('မ', ASAT, '.', EOF_CHAR) =>
    {
      cursor.next();
      cursor.next();
      vowel.virama = Some(Virama::M);
      vowel.tone = Some(Tone::Creaky);
      return Ok((Syllable::new(consonant, vowel, None), consumed_len + 3));
    }
    ('မ', STACK_SIGN, 'ပ' | 'ဗ' | 'ဘ' | 'မ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::M);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    ('ဿ', ..) =>
    {
      vowel.virama = Some(Virama::S);
      let new_str = format!("သ{}", cursor.as_str());
      let c = match parse_syllable(&new_str)
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 1,
      ));
    }
    ('လ', STACK_SIGN, 'လ', ..) =>
    {
      cursor.next();
      vowel.virama = Some(Virama::L);
      let c = match parse_syllable(cursor.as_str())
      {
        Ok((c, l)) => (c, l),
        Err(_) => return Err(syllable),
      };
      return Ok((
        Syllable::new(consonant, vowel, Some(c.0.into())),
        consumed_len + 2 + c.1,
      ));
    }
    _ => Err(syllable),
  }
}

/// Splits the input text into syllables.
/// Keep in mind that bottom consonant in a stacked consonants are not
/// separated from the top consonant. (E.g. တက္ကသိုလ် is split as တက္က သိုလ်)
///
/// # Example
///
/// ```
/// let input = "ကျွန်တော်က တက္ကသိုလ်ကျောင်းသားပါ။";
/// let syllables = mlcts_from_my::split_syllables(input);
/// let expected = &["ကျွန်", "တော်", "က", " ", "တက္က", "သိုလ်", "ကျောင်း", "သား", "ပါ", "။"];
/// assert_eq!(syllables, expected);
/// ```
///
/// # Arguments
///
/// * `input` - The input text to split.
///
/// # Returns
///
/// A vector of syllables.
pub fn split_syllables<'i>(input: &'i str) -> Vec<&'i str>
{
  // this regex is borrowed from https://github.com/ye-kyaw-thu/myWord.
  let p = format!(
    r"((?<!္)([က-အ])(?![်္])|[a-zA-Z0-9{}])",
    "ဣဤဥဦဧဩဪ၌၍၏၀-၉၊။!-/:-@\\[-`{-~\\s"
  );

  let matches = Regex::new(&p)
    .unwrap()
    .find_iter(input)
    .filter_map(Result::ok)
    .map(|m| m.start())
    .collect::<Vec<usize>>();

  matches
    .windows(2)
    .map(|w| &input[w[0] .. w[1]])
    .chain(matches.last().map(|&l| &input[l ..]))
    .collect()
}

#[cfg(test)]
mod tests
{
  #[test]
  fn test_split_syllables()
  {
    // sample text is also borrowed from https://github.com/ye-kyaw-thu/myWord
    let input = "ကျွန်တော်ကသုတေသနသမားပါ။\n\
    နေ့ရောညရောမြန်မာစာနဲ့ကွန်ပျူတာနဲ့ပဲအလုပ် များ ပါ တယ်\n\
    မင်းကကောဘာအလုပ်လုပ်တာလဲ။\n\
    ပြောပြပါအုံး\n\
    ကော်ဖီလည်းထပ်သောက်ချင်ရင်ပြောကွာ";

    let syllables = super::split_syllables(input);
    #[rustfmt::skip]
    let expected = &[
      "ကျွန်", "တော်", "က", "သု", "တေ", "သ", "န", "သ", "မား", "ပါ", "။", "\n",
      "နေ့", "ရော", "ည", "ရော", "မြန်", "မာ", "စာ", "နဲ့",
      "ကွန်", "ပျူ", "တာ", "နဲ့", "ပဲ",
      "အ", "လုပ်", " ", "များ", " ", "ပါ", " ", "တယ်", "\n",
      "မင်း", "က", "ကော","ဘာ", "အ", "လုပ်", "လုပ်", "တာ", "လဲ", "။", "\n",
      "ပြော", "ပြ", "ပါ", "အုံး", "\n",
      "ကော်", "ဖီ", "လည်း", "ထပ်", "သောက်", "ချင်", "ရင်", "ပြော", "ကွာ"];
    assert_eq!(syllables, expected)
  }

  #[test]
  fn test_mlcts_from_my()
  {
    let input = "ပိဿာ";
    let mlcts = super::from_my(input);
    assert_eq!(mlcts, "pissa");
  }
}
