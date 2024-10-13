//! # mlcts_core
//!
//! This crate provides the core enums and structs used in the MLCTS.
//! Enums like consonants, vowels, etc. are only related to the MLCTS and might
//! not be able to map one-to-one with the Myanmar alphabets.

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

impl BasicConsonant
{
  /// Converts a BasicConsonant into MLCTS string
  ///
  /// # Returns
  ///
  /// The corresponding MLCTS string.
  pub fn to_mlcts(&self) -> &str
  {
    match self
    {
      Self::K => "k",
      Self::Hk => "hk",
      Self::G => "g",
      Self::Gh => "gh",
      Self::Ng => "ng",
      Self::C => "c",
      Self::Hc => "hc",
      Self::J => "j",
      Self::Jh => "jh",
      Self::Ny => "ny",
      Self::T => "t",
      Self::Ht => "ht",
      Self::D => "d",
      Self::Dh => "dh",
      Self::N => "n",
      Self::P => "p",
      Self::Hp => "hp",
      Self::B => "b",
      Self::Bh => "bh",
      Self::M => "m",
      Self::Y => "y",
      Self::R => "r",
      Self::L => "l",
      Self::W => "w",
      Self::S => "s",
      Self::H => "h",
      Self::A => "a",
    }
  }

  /// Converts a char into a BasicConsonant.
  ///
  /// # Arguments
  ///
  /// * `c` - The char in Myanmar alphabet.
  ///
  /// # Returns
  ///
  /// The corresponding BasicConsonant value if the char is a valid Myanmar
  /// consonant. Otherwise, an error.
  pub fn from_my_alphabet(c: char) -> Result<BasicConsonant, ()>
  {
    match c
    {
      'က' => Ok(BasicConsonant::K),
      'ခ' => Ok(BasicConsonant::Hk),
      'ဂ' => Ok(BasicConsonant::G),
      'ဃ' => Ok(BasicConsonant::Gh),
      'င' => Ok(BasicConsonant::Ng),
      'စ' => Ok(BasicConsonant::C),
      'ဆ' => Ok(BasicConsonant::Hc),
      'ဇ' => Ok(BasicConsonant::J),
      'ဈ' => Ok(BasicConsonant::Jh),
      'ဉ' => Ok(BasicConsonant::Ny),
      'ည' => Ok(BasicConsonant::Ny),
      'ဋ' => Ok(BasicConsonant::T),
      'ဌ' => Ok(BasicConsonant::Ht),
      'ဍ' => Ok(BasicConsonant::D),
      'ဎ' => Ok(BasicConsonant::Dh),
      'ဏ' => Ok(BasicConsonant::N),
      'တ' => Ok(BasicConsonant::T),
      'ထ' => Ok(BasicConsonant::Ht),
      'ဒ' => Ok(BasicConsonant::D),
      'ဓ' => Ok(BasicConsonant::Dh),
      'န' => Ok(BasicConsonant::N),
      'ပ' => Ok(BasicConsonant::P),
      'ဖ' => Ok(BasicConsonant::Hp),
      'ဗ' => Ok(BasicConsonant::B),
      'ဘ' => Ok(BasicConsonant::Bh),
      'မ' => Ok(BasicConsonant::M),
      'ယ' => Ok(BasicConsonant::Y),
      'ရ' => Ok(BasicConsonant::R),
      'လ' => Ok(BasicConsonant::L),
      'ဝ' => Ok(BasicConsonant::W),
      'သ' => Ok(BasicConsonant::S),
      'ဟ' => Ok(BasicConsonant::H),
      'ဠ' => Ok(BasicConsonant::L),
      'အ' => Ok(BasicConsonant::A),
      _ => Err(()),
    }
  }
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

  /// Convert Consontant to MLCTS string
  ///
  /// # Returns
  ///
  /// The corresponding MLCTS string.
  pub fn to_mlcts(&self) -> String
  {
    let result = self.basic.to_mlcts().to_string();
    match self.medial
    {
      Some(MedialDiacritic::Hrw) => format!("h{}rw", result),
      Some(MedialDiacritic::Hyw) => format!("h{}yw", result),
      Some(MedialDiacritic::Hw) => format!("h{}w", result),
      Some(MedialDiacritic::Hr) => format!("h{}r", result),
      Some(MedialDiacritic::Hy) => format!("h{}y", result),
      Some(MedialDiacritic::H) => format!("h{}", result),
      Some(MedialDiacritic::Rw) => format!("r{}w", result),
      Some(MedialDiacritic::R) => format!("r{}", result),
      Some(MedialDiacritic::Yw) => format!("y{}w", result),
      Some(MedialDiacritic::Y) => format!("y{}", result),
      Some(MedialDiacritic::W) => format!("w{}", result),
      None => result,
    }
  }
}

/// A macro to create a simple consonant.
#[macro_export]
macro_rules! consonant {
  ($name:ident) => {
    $crate::Consonant::simple($crate::BasicConsonant::$name)
  };
  ($name:ident, $medial:ident) => {
    $crate::Consonant::with_medial(
      $crate::BasicConsonant::$name,
      $crate::MedialDiacritic::$medial,
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

impl Tone
{
  /// Converts a Tone into MLCTS string
  ///
  /// # Returns
  ///
  /// The corresponding MLCTS string.
  pub fn to_mlcts(&self) -> &str
  {
    match self
    {
      Self::High => ":",
      Self::Creaky => ".",
    }
  }
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
  /// ဂ်
  G,
  /// င်
  Ng,
  /// စ်
  C,
  /// ဇ်
  J,
  /// ည် or ဉ်
  Ny,
  /// ဋ် or တ်
  T,
  /// ထ်
  Ht,
  /// ဒ်
  D,
  /// ဏ် or န်
  N,
  /// ပ်
  P,
  /// ဗ်
  B,
  /// မ် or ံ
  M,
  /// သ်
  S,
  /// လ်
  L,
}

impl Virama
{
  /// Converts virama into MLCTS string
  ///
  /// # Returns
  ///
  /// The corresponding MLCTS string.
  pub fn to_mlcts(&self) -> &str
  {
    match self
    {
      Self::K => "k",
      Self::G => "g",
      Self::Ng => "ng",
      Self::C => "c",
      Self::J => "j",
      Self::Ny => "ny",
      Self::T => "t",
      Self::Ht => "ht",
      Self::D => "d",
      Self::N => "n",
      Self::P => "p",
      Self::B => "b",
      Self::M => "m",
      Self::S => "s",
      Self::L => "l",
    }
  }
}

impl Into<BasicConsonant> for Virama
{
  /// Converts a Virama into a BasicConsonant.
  ///
  /// # Returns
  ///
  /// The corresponding BasicConsonant value.
  fn into(self) -> BasicConsonant
  {
    match self
    {
      Self::K => BasicConsonant::K,
      Self::G => BasicConsonant::G,
      Self::Ng => BasicConsonant::Ng,
      Self::C => BasicConsonant::C,
      Self::J => BasicConsonant::J,
      Self::Ny => BasicConsonant::Ny,
      Self::T => BasicConsonant::T,
      Self::Ht => BasicConsonant::Ht,
      Self::D => BasicConsonant::D,
      Self::N => BasicConsonant::N,
      Self::P => BasicConsonant::P,
      Self::B => BasicConsonant::B,
      Self::M => BasicConsonant::M,
      Self::S => BasicConsonant::S,
      Self::L => BasicConsonant::L,
    }
  }
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

impl BasicVowel
{
  /// Converts a BasicVowel into MLCTS string
  ///
  /// # Returns
  ///
  /// The corresponding MLCTS string.
  pub fn to_mlcts(&self) -> &str
  {
    match self
    {
      Self::A => "a",
      Self::I => "i",
      Self::U => "u",
      Self::E => "e",
      Self::Ai => "ai",
      Self::Au => "au",
      Self::Ui => "ui",
    }
  }
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

  /// Convert Vowel to MLCTS string
  ///
  /// # Returns
  ///
  /// The corresponding MLCTS string.
  pub fn to_mlcts(&self) -> String
  {
    let result = self.basic.to_mlcts().to_string();
    let virama = if self.virama.is_some()
    {
      self.virama.unwrap().to_mlcts().to_string()
    }
    else
    {
      "".to_string()
    };
    let tone = if self.tone.is_some()
    {
      self.tone.unwrap().to_mlcts().to_string()
    }
    else
    {
      "".to_string()
    };
    format!("{}{}{}", result, virama, tone)
  }
}

/// A macro to create a simple vowel.
#[macro_export]
macro_rules! vowel {
  ($name:ident) => {
    $crate::Vowel::simple($crate::BasicVowel::$name)
  };
  ($name:ident; $tone:ident) => {
    $crate::Vowel::with_tone(
      $crate::BasicVowel::$name,
      Some($crate::Tone::$tone),
    )
  };
  ($name:ident, $virama:ident) => {
    $crate::Vowel::with_virama(
      $crate::BasicVowel::$name,
      $crate::Virama::$virama,
    )
  };
  ($name:ident, $virama:ident; $tone:ident) => {
    $crate::Vowel::new(
      $crate::BasicVowel::$name,
      Some($crate::Virama::$virama),
      Some($crate::Tone::$tone),
    )
  };
}

/// Represents a base syllable.
/// A base syllable is a syllable that does not contain a bottom syllable.
#[derive(
  serde::Serialize, serde::Deserialize, Debug, Clone, Copy, PartialEq, Eq,
)]
pub struct BaseSyllable
{
  pub consonant: Consonant,
  pub vowel: Vowel,
}

impl Into<Syllable> for BaseSyllable
{
  /// Converts a BaseSyllable into a Syllable.
  ///
  /// # Returns
  ///
  /// The corresponding Syllable value.
  fn into(self) -> Syllable
  {
    Syllable {
      consonant: self.consonant,
      vowel: self.vowel,
      bottom_syllable: None,
    }
  }
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
  /// The optional bottom consonant of stacked consonants.
  /// This will be Some only if vowel has a virama.
  /// And this can be a complete syllable itself.
  pub bottom_syllable: Option<BaseSyllable>,
}

impl Into<BaseSyllable> for Syllable
{
  /// Converts a Syllable into a BaseSyllable.
  ///
  /// # Returns
  ///
  /// The corresponding BaseSyllable value.
  fn into(self) -> BaseSyllable
  {
    BaseSyllable {
      consonant: self.consonant,
      vowel: self.vowel,
    }
  }
}

impl Syllable
{
  /// Creates a new syllable with the given consonant and vowel.
  ///
  /// # Arguments
  ///
  /// * `consonant` - The consonant part.
  /// * `vowel` - The vowel part.
  /// * `bottom_syllable` - The optional bottom consonant of stacked
  ///
  /// # Returns
  ///
  /// A new syllable with the given consonant and vowel.
  pub fn new(
    consonant: Consonant,
    vowel: Vowel,
    bottom_syllable: Option<BaseSyllable>,
  ) -> Self
  {
    Self {
      consonant,
      vowel,
      bottom_syllable,
    }
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
  pub fn simple(vowel: Vowel) -> Self
  {
    Self::new(consonant!(A), vowel, None)
  }

  /// Convert Syllable to MLCTS string
  ///
  /// # Returns
  ///
  /// The corresponding MLCTS string.
  pub fn to_mlcts(&self) -> String
  {
    let consonant = self.consonant.to_mlcts();
    let vowel = self.vowel.to_mlcts();
    let bottom = if self.bottom_syllable.is_some()
    {
      let s: Syllable = self.bottom_syllable.unwrap().into();
      s.to_mlcts()
    }
    else
    {
      "".to_string()
    };
    format!("{}{}{}", consonant, vowel, bottom)
  }
}

/// A macro to create a simple syllable.
#[macro_export]
macro_rules! syllable {
  ($vowel:expr) => {
    $crate::Syllable::simple($vowel)
  };
  ($consonant:expr, $vowel:expr) => {
    $crate::Syllable::new($consonant, $vowel, None)
  };
  ($consonant:expr, $vowel:expr, $bottom_consonant:expr) => {
    $crate::Syllable::new($consonant, $vowel, Some($bottom_consonant))
  };
}
