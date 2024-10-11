/// The starting offset value to make providing emum values easier.
/// If we want to use 0x1000 as the value for 'k', we can just
/// set this value to 0x1000 and then all the following values will be
/// automatically calculated.
pub const BASIC_CONSONANT_START_VALUE: u8 = 0x00;

/// Represents a basic consonant letter in the Myanmar script.
#[repr(u8)]
#[derive(
  serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum BasicConsonant
{
  /// က
  K = BASIC_CONSONANT_START_VALUE + 0x00,
  /// ခ
  Hk,
  /// ဂ
  G,
  /// ဃ
  Gh,
  /// င
  Ng,
  /// စ
  C,
  /// ဆ
  Hc,
  /// ဇ
  J,
  /// ဈ
  Jh,
  // 0x1009 (ဉ) is skipped since the same character (ny) is used.
  /// ည
  Ny = BASIC_CONSONANT_START_VALUE + 0x0A,
  /// ဋ
  T,
  /// ဌ
  Ht,
  /// ဍ
  D,
  /// ဎ
  Dh,
  /// ဏ
  N,
  // 0x100F to 0x1014 (တ to န) is skipped
  // since the same characters (t to n) is used.
  /// ပ
  P = BASIC_CONSONANT_START_VALUE + 0x15,
  /// ဖ
  Hp,
  /// ဗ
  B,
  /// ဘ
  Bh,
  /// မ
  M,
  /// ယ
  Y,
  /// ရ
  R,
  /// လ
  L,
  /// ဝ
  W,
  /// သ
  S,
  /// ဟ
  H,
  // 0x1020 (ဠ) is skipped since the same character (l) is used.
  /// အ
  A = BASIC_CONSONANT_START_VALUE + 0x21,
}

/// Represents medial diacritics in the Myanmar script.
#[repr(u8)]
#[derive(
  serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum MedialDiacritic
{
  /// ယပင့်
  Y,
  /// ရရစ်
  R,
  /// ဝဆွဲ
  W,
  /// ဟထိုး
  H,
  /// ယပင့် ဝဆွဲ
  Yw,
  /// ရရစ် ဝဆွဲ
  Rw,
  /// ယပင့် ဟထိုး
  Hy,
  /// ရရစ် ဟထိုး
  Hr,
  /// ဝဆွဲ ဟထိုး
  Hw,
  /// ယပင့် ဝဆွဲ ဟထိုး
  Hyw,
  /// ရရစ် ဝဆွဲ ဟထိုး
  Hrw,
}

impl MedialDiacritic
{
  /// Combine two medial diacritics into one.
  /// If the two medial diacritics cannot be combined, this function will return
  /// an error.
  ///
  /// # Arguments
  ///
  /// * `first` - The first medial diacritic.
  /// * `second` - The second medial diacritic.
  ///
  /// # Returns
  ///
  /// The combined medial diacritic if the two medial diacritics can be
  /// combined. Otherwise, an error.
  pub fn combine(self, b: Self) -> Result<Self, ()>
  {
    match (self, b)
    {
      (Self::H, Self::Y) => Ok(Self::Hy),
      (Self::H, Self::R) => Ok(Self::Hr),
      (Self::H, Self::W) => Ok(Self::Hw),
      (Self::Y, Self::W) => Ok(Self::Yw),
      (Self::R, Self::W) => Ok(Self::Rw),
      (Self::Hy, Self::W) => Ok(Self::Hyw),
      (Self::Hr, Self::W) => Ok(Self::Hrw),
      _ => Err(()),
    }
  }

  /// Combine two optional medial diacritics into one.
  /// If the two medial diacritics cannot be combined, this function will return
  /// an error.
  ///
  /// # Arguments
  ///
  /// * `first` - The first optional medial diacritic.
  /// * `second` - The second optional medial diacritic.
  ///
  /// # Returns
  ///
  /// The combined medial diacritic if the two medial diacritics can be
  /// combined. Otherwise, an error.
  pub fn combine_medial_diacritics(
    first: Option<Self>,
    second: Option<Self>,
  ) -> Result<Option<Self>, ()>
  {
    match (first, second)
    {
      (Some(a), Some(b)) => Ok(Some(a.combine(b)?)),
      (Some(a), None) => Ok(Some(a)),
      (None, Some(b)) => Ok(Some(b)),
      (None, None) => Ok(None),
    }
  }
}

/// Represents the consonant part of a Myanmar syllable.
/// This can be a basic consonant or a basic consonant followed by one or more
/// medial diacritics (three at most).
#[derive(
  serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq,
)]
pub struct Consonant
{
  /// The basic consonant part.
  pub basic: BasicConsonant,
  /// The optional medial diacritic part.
  pub medial: Option<MedialDiacritic>,
}

impl Consonant
{
  /// Creates a new consonant with the given basic consonant and medial
  /// diacritic.
  ///
  /// # Arguments
  ///
  /// * `basic` - The basic consonant part.
  /// * `medial` - The medial diacritic part.
  ///
  /// # Returns
  ///
  /// A new consonant with the given basic consonant and medial diacritic.
  pub fn new(basic: BasicConsonant, medial: Option<MedialDiacritic>) -> Self
  {
    Self { basic, medial }
  }

  /// Creates a new simple consonant with the given basic consonant.
  /// This is a shorthand for `Consonant::new(basic, None)`.
  ///
  /// # Arguments
  ///
  /// * `basic` - The basic consonant part.
  ///
  /// # Returns
  ///
  /// A new consonant with the given basic consonant.
  pub fn simple(basic: BasicConsonant) -> Self
  {
    Self::new(basic, None)
  }

  /// Creates a new consonant with the given basic consonant and medial
  /// diacritic.
  ///
  /// # Arguments
  ///
  /// * `basic` - The basic consonant part.
  /// * `medial` - The medial diacritic part.
  ///
  /// # Returns
  ///
  /// A new consonant with the given basic consonant and medial diacritic.
  pub fn with_medial(basic: BasicConsonant, medial: MedialDiacritic) -> Self
  {
    Self::new(basic, Some(medial))
  }
}

/// A macro to create a simple consonant.
#[macro_export]
macro_rules! consonant {
  ($name:ident) => {
    $crate::core::Consonant::simple($crate::core::BasicConsonant::$name)
  };
  ($name:ident, $medial:ident) => {
    $crate::core::Consonant::with_medial(
      $crate::core::BasicConsonant::$name,
      $crate::core::MedialDiacritic::$medial,
    )
  };
}

/// Represents a tone mark in the Myanmar script.
/// A syllable can have at most one tone mark. But some vowel combinations
/// cannot have a tone mark.
#[repr(u8)]
#[derive(
  serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum Tone
{
  /// ဝစ္စပေါက် (Visarga)
  High,
  /// အောက်မြစ် (Anusvara)
  Creaky,
}

/// Represents a Virama (အသတ်) in the Myanmar script.
/// Virama can follow a consonant or vowel. But a vowel cannot follow a virama.
#[repr(u8)]
#[derive(
  serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum Virama
{
  /// က်
  K,
  /// င်
  Ng,
  /// စ်
  C,
  /// ည် or ဉ်
  Ny,
  /// ဋ် or တ်
  T,
  /// ဏ် or န်
  N,
  /// ပ်
  P,
  /// မ် or ံ
  M,
}

/// Represents a basic vowel letter in the Myanmar script.
/// This enum contains only vowels classified as "basic" vowels and vowels with
/// same sound but different tone will be treated as the same vowels.
#[repr(u8)]
#[derive(
  serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum BasicVowel
{
  /// အ, အာ, အား
  A,
  /// အိ, အီ, အီး
  I,
  /// အု, အူ, အူး
  U,
  /// အေ, အေ့, အေး
  E,
  /// အဲ့, အယ်, အဲ
  Ai,
  /// အော့, အော်, အော
  Au,
  /// အို, အို့, အိုး
  Ui,
}

/// Represents the vowel part of a Myanmar syllable.
/// This can be a basic vowel, vowel with tone mark or a basic vowel
/// followed by a virama and a tone mark.
///
/// Virama with consonantal finals (က, စ, ဋ, တ, ပ) cannot be
/// followed by a tone mark since they already sounds Creaky tone.
#[derive(
  serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq,
)]
pub struct Vowel
{
  /// The basic vowel part.
  pub basic: BasicVowel,
  /// The optional virama part.
  pub virama: Option<Virama>,
  /// The optional tone mark part.
  pub tone: Option<Tone>,
}

impl Vowel
{
  /// Creates a new vowel with the given basic vowel, virama and tone mark.
  ///
  /// # Arguments
  ///
  /// * `basic` - The basic vowel part.
  /// * `virama` - The virama part.
  /// * `tone` - The tone mark part.
  ///
  /// # Returns
  ///
  /// A new vowel with the given basic vowel, virama and tone mark.
  pub fn new(
    basic: BasicVowel,
    virama: Option<Virama>,
    tone: Option<Tone>,
  ) -> Self
  {
    Self {
      basic,
      virama,
      tone,
    }
  }

  /// Creates a new vowel with the given basic vowel.
  /// This is a shorthand for `Vowel::new(basic, None, None)`.
  ///
  /// # Arguments
  ///
  /// * `basic` - The basic vowel part.
  ///
  /// # Returns
  ///
  /// A new simple vowel with the given basic vowel.
  pub fn simple(basic: BasicVowel) -> Self
  {
    Self::new(basic, None, None)
  }

  /// Creates a new vowel with the given basic vowel and tone mark.
  /// This is a shorthand for `Vowel::new(basic, None, tone)`.
  ///
  /// # Arguments
  ///
  /// * `basic` - The basic vowel part.
  /// * `tone` - The tone mark part.
  ///
  /// # Returns
  ///
  /// A new vowel with the given basic vowel and tone mark.
  pub fn with_tone(basic: BasicVowel, tone: Option<Tone>) -> Self
  {
    Self::new(basic, None, tone)
  }

  /// Creates a new vowel with the given basic vowel and virama.
  /// This is a shorthand for `Vowel::new(basic, virama, None)`.
  ///
  /// # Arguments
  ///
  /// * `basic` - The basic vowel part.
  /// * `virama` - The virama part.
  ///
  /// # Returns
  ///
  /// A new vowel with the given basic vowel and virama.
  pub fn with_virama(basic: BasicVowel, virama: Virama) -> Self
  {
    Self::new(basic, Some(virama), None)
  }
}

/// A macro to create a simple vowel.
#[macro_export]
macro_rules! vowel {
  ($name:ident) => {
    $crate::core::Vowel::simple($crate::core::BasicVowel::$name)
  };
  ($name:ident; $tone:ident) => {
    $crate::core::Vowel::with_tone(
      $crate::core::BasicVowel::$name,
      Some($crate::core::Tone::$tone),
    )
  };
  ($name:ident, $virama:ident) => {
    $crate::core::Vowel::with_virama(
      $crate::core::BasicVowel::$name,
      $crate::core::Virama::$virama,
    )
  };
  ($name:ident, $virama:ident; $tone:ident) => {
    $crate::core::Vowel::new(
      $crate::core::BasicVowel::$name,
      Some($crate::core::Virama::$virama),
      Some($crate::core::Tone::$tone),
    )
  };
}

/// Represents a Myanmar syllable.
/// A syllable can have at most one consonant part and one vowel part.
/// Syllable will always contains both consonant and vowel parts since 'a' can
/// be both a consonant and a vowel.
#[derive(
  serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq,
)]
pub struct Syllable
{
  /// The consonant part.
  pub consonant: Consonant,
  /// The vowel part.
  pub vowel: Vowel,
}

impl Syllable
{
  /// Creates a new syllable with the given consonant and vowel.
  ///
  /// # Arguments
  ///
  /// * `consonant` - The consonant part.
  /// * `vowel` - The vowel part.
  ///
  /// # Returns
  ///
  /// A new syllable with the given consonant and vowel.
  pub fn new(consonant: Consonant, vowel: Vowel) -> Self
  {
    Self { consonant, vowel }
  }

  /// Creates a new syllable with just the vowel part.
  /// This is a shorthand for
  /// `Syllable::new(consonant!(A), vowel)`.
  ///
  /// # Arguments
  ///
  /// * `vowel` - The vowel part.
  ///
  /// # Returns
  ///
  /// A new syllable with just the vowel part.
  pub fn with_vowel(vowel: Vowel) -> Self
  {
    Self::new(consonant!(A), vowel)
  }
}

/// A macro to create a simple syllable.
#[macro_export]
macro_rules! syllable {
  ($vowel:expr) => {
    $crate::core::Syllable::with_vowel($vowel)
  };
  ($consonant:expr, $vowel:expr) => {
    $crate::core::Syllable::new($consonant, $vowel)
  };
}
