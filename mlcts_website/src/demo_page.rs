use leptos::html::*;
use leptos::*;
use leptos_meta::{Title, TitleProps};
use mlcts_tokenizer::mlcts_core::Syllable;
use mlcts_tokenizer::{tokenize, Token, TokenKind};

#[component]
pub fn DemoPage() -> impl IntoView
{
  (
    Title(TitleProps {
      text: Some("LibMLCTS Demo".into()),
      formatter: None,
    }),
    div()
      .classes("max-w-[var(--breakpoint-sm)] mx-auto px-4 py-4")
      .child(MainArea()),
  )
}

#[component]
fn MainArea() -> impl IntoView
{
  let (input, set_input) = create_signal(String::new());

  (
    h1()
      .classes("text-2xl font-bold mb-4")
      .child("LibMLCTS Demo"),
    hr().class("mb-4", true),
    move || {
      if input.get().is_empty()
      {
        div()
          .classes("border border-slate-100 rounded-md p-4 bg-slate-100")
          .classes("mb-4")
          .child("Enter some text to see the tokens.")
          .into_view()
      }
      else
      {
        OutputArea(OutputAreaProps { input }).into_view()
      }
    },
    InputArea(InputAreaProps { input, set_input }),
  )
}

/// The input area component.
///
/// # Arguments
///
/// * `input` - The input signal.
///
/// # Returns
///
/// The input area component.
#[component]
fn InputArea(
  input: ReadSignal<String>,
  set_input: WriteSignal<String>,
) -> impl IntoView
{
  let input_label = label()
    .child("Input:")
    .attr("for", "text-input")
    .classes("text-lg block mb-2");

  let textarea = textarea()
    .id("text-input")
    .classes("w-full border border-gray-300 rounded-md p-4")
    .attr("rows", "5")
    .attr("placeholder", "Enter Text Here")
    .attr("spellcheck", "false")
    .child(move || input.get())
    .on(ev::input, move |e| {
      let value = event_target_value(&e);
      set_input.set(value);
    });

  div().classes("mb-4").child((input_label, textarea))
}

/// The output area component.
///
/// # Arguments
///
/// * `input` - The input signal.
///
/// # Returns
///
/// The output area component.
#[component]
fn OutputArea(input: ReadSignal<String>) -> impl IntoView
{
  let (tokens, set_tokens) = create_signal(Vec::<Token>::new());

  create_effect(move |_| {
    let text = input.get();
    set_tokens(tokenize(&text).collect());
  });

  let label_elm = label()
    .child("Tokens:")
    .attr("for", "tokens-output")
    .classes("text-lg block mb-2");

  let tkn_box = div()
    .id("tokens-output")
    .classes("flex flex-wrap gap-2 items-stretch")
    .child(move || {
      tokens
        .get()
        .into_iter()
        .map(|token| token_view(token, input.get().clone()))
        .collect::<Vec<_>>()
    });

  div().classes("mb-4").child((label_elm, tkn_box))
}

/// Get a substring of a string.
///
/// # Arguments
///
/// * `s` - The string to get the substring from.
/// * `start` - The start index of the substring.
/// * `length` - The length of the substring.
///
/// # Returns
///
/// The substring of the string.
fn substring(s: &str, start: usize, length: usize) -> &str
{
  let end = s
    .char_indices()
    .nth(start + length)
    .map_or_else(|| s.len(), |(idx, _)| idx);

  let start = s.char_indices().nth(start).map_or(0, |(idx, _)| idx);

  &s[start .. end]
}

/// Component to display a syllable token.
///
/// # Arguments
///
/// * `syl` - The syllable to display.
///
/// # Returns
///
/// The syllable token component.
fn syllable_tkn(syl: Syllable) -> impl IntoView
{
  let cls = "px-2 border inline-block rounded-md text-white";

  let c = span()
    .classes(cls)
    .class("bg-sky-500", true)
    .child(format!("{:?}", syl.consonant.basic));

  let m = syl.consonant.medial.map(|medial| {
    span()
      .classes(cls)
      .class("bg-teal-500", true)
      .child(format!("{:?}", medial))
  });

  let v = span()
    .classes(cls)
    .class("bg-rose-500", true)
    .child(format!("{:?}", syl.vowel.basic));

  let vi = syl.vowel.virama.map(|virama| {
    span()
      .classes(cls)
      .class("bg-amber-500", true)
      .child(format!("{:?}", virama))
  });

  let t = syl.vowel.tone.map(|tone| {
    span()
      .classes(cls)
      .class("bg-indigo-500", true)
      .child(format!("{:?}", tone))
  });

  div()
    .classes("inline-flex items-center border border-stone-500 rounded-md p-1")
    .child((c, m, v, vi, t))
}

/// Component to display a token.
///
/// # Arguments
///
/// * `t` - The token to display.
/// * `input` - The input string.
///
/// # Returns
///
/// The token component.
fn token_view(t: Token, input: String) -> impl IntoView
{
  let text = substring(input.as_str(), t.start, t.len).to_string();
  let common_class = "px-4 inline-flex items-center rounded-md";

  match t.kind
  {
    TokenKind::Syllable(syl) => syllable_tkn(syl).into_view(),
    TokenKind::Unknown => div()
      .classes("bg-stone-600 text-white border border-stone-500")
      .classes(common_class)
      .child(text)
      .into_view(),
    TokenKind::Whitespace => div()
      .classes(common_class)
      .classes("bg-slate-50 text-slate-50 border border-stone-500 ")
      .child("\u{00A0}")
      .into_view(),
    TokenKind::EndOfInput => unreachable!(),
  }
}
