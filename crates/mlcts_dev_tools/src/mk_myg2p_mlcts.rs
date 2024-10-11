use std::io::Read;
use std::path::Path;

use clap::Parser;
use deno_core::{serde_v8, v8, JsRuntime, RuntimeOptions};
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;

lazy_static! {
  static ref ROMANIZATIONS_JS: String = get_js_code("romanisations.js");
  static ref MYA2ROM_JS: String = get_js_code("mya2rom.js");
}

/// CLI options
#[derive(Parser)]
struct ClapOptions
{
  /// If the original dictionary should be re-downloaded
  #[arg(short, long, default_value_t = false)]
  download_original: bool,
}

fn main()
{
  let cli_options = ClapOptions::parse();

  let mut deno_rt = JsRuntime::new(RuntimeOptions::default());
  let libs = format!("{}{}", *ROMANIZATIONS_JS, *MYA2ROM_JS);
  // Load library code into Deno runtime
  deno_eval(&mut deno_rt, libs.clone());

  let output_path = Path::new(env!("CARGO_MANIFEST_DIR"))
    .join("..")
    .join("..")
    .join("assets")
    .join("myg2p-dict-mlcts.csv");

  let mut csv_writer = csv::Writer::from_path(output_path).unwrap();
  csv_writer
    .write_record(&[
      "myanmar_word",
      "mlcts_romanization",
      "syllable_count",
      "myanmar_syllables",
      "mlcts_syllables",
    ])
    .unwrap();

  let original_dict_content = get_original_dict(cli_options.download_original);
  let dict = load_original_dict(original_dict_content.as_str());

  let progress_bar = ProgressBar::new(dict.len() as u64);
  progress_bar.set_style(
    ProgressStyle::with_template("[*] generating {wide_bar} {pos}/{len}")
      .unwrap(),
  );

  println!("[*] generating MLCTS romanization");
  for row in dict
  {
    generate_mlcts(row, &mut csv_writer, &mut deno_rt);
    progress_bar.inc(1);
  }

  progress_bar.finish();
  println!("[*] done generating MLCTS romanization");
  csv_writer.flush().unwrap();
}

/// Load the original dictionary, clean it and prepare for processing.
///
/// # Returns
///
/// Rows of the cleaned dictionary
fn load_original_dict(content: &str) -> Vec<(&str, Vec<&str>)>
{
  content
    .lines()
    .filter(|line| !line.is_empty())
    .filter(|line| !line.contains("..."))
    .map(|line| line.split('\t').collect::<Vec<&str>>())
    .map(|parts| (parts[1], parts[2].split(' ').collect::<Vec<&str>>()))
    .collect()
}

/// Generate a new role with MLCTS romanization.
///
/// # Arguments
///
/// * `row` - A row of the original dictionary
///
/// # Returns
///
/// A new row with MLCTS romanization
fn generate_mlcts(
  row: (&str, Vec<&str>),
  csv_writer: &mut csv::Writer<std::fs::File>,
  deno_rt: &mut JsRuntime,
)
{
  csv_writer
    .write_record(&[
      row.0,
      &mya2rom(row.0, deno_rt),
      &row.1.len().to_string(),
      &row.1.join("|"),
      &row
        .1
        .iter()
        .map(|&s| mya2rom(s, deno_rt))
        .collect::<Vec<String>>()
        .join("|"),
    ])
    .unwrap();
}

/// Convert a Myanmar word to MLCTS romanization.
///
/// # Arguments
///
/// * `word` - Myanmar word
///
/// # Returns
///
/// MLCTS romanization of the word
fn mya2rom(word: &str, deno_rt: &mut JsRuntime) -> String
{
  let code = format!("mya2rom('{}', 'mlcts', false, false)", word);
  let res = deno_eval(deno_rt, code);

  res
    .as_str()
    .unwrap()
    .replace("nhg", "hng") // fix nhg -> hng
    .replace("nhy", "hny") // fix nhy -> hny
    .replace(" ", "")
}

/// Get required JS code from myanmaropenwordnet/mya2rom repo.
///
/// # Arguments
///
/// * `filename` - JS file name
///
/// # Returns
///
/// Content of the JS file
fn get_js_code(filename: &str) -> String
{
  let base =
    "https://raw.githubusercontent.com/myanmaropenwordnet/mya2rom/933c186";
  reqwest::blocking::get(&format!("{}/{}", base, filename))
    .unwrap()
    .text()
    .unwrap()
}

/// Get the original dictionary from the URL
///
/// # Returns
///
/// Original dictionary content
fn get_original_dict(download: bool) -> String
{
  let dict_path = Path::new(env!("CARGO_MANIFEST_DIR"))
  .join("..")
  .join("..")
    .join("assets")
    .join("myg2p-dict.txt");

  if !download && dict_path.exists()
  {
    return std::fs::read_to_string(&dict_path).unwrap();
  }

  println!("[*] downloading the original dictionary");
  let client = reqwest::blocking::Client::new();
  let url = "https://raw.githubusercontent.com/ye-kyaw-thu/myG2P/refs/heads/master/ver2/myg2p.ver2.0.txt";
  let mut response = client.get(url).send().unwrap();

  let total_size = response.content_length().unwrap();

  let progress_bar = ProgressBar::new(total_size);
  progress_bar.set_style(
    ProgressStyle::with_template(
      "[*] downloading {wide_bar} {bytes}/{total_bytes} ({eta})",
    )
    .unwrap(),
  );

  let mut content = Vec::new();
  let mut downloaded_size = 0;
  let mut buffer = [0; 8192]; // 8KB buffer

  loop
  {
    let bytes_read = response.read(&mut buffer).unwrap();
    if bytes_read == 0
    {
      break;
    }
    content.extend_from_slice(&buffer[.. bytes_read]);
    downloaded_size += bytes_read as u64;
    progress_bar.set_position(downloaded_size);
    std::thread::sleep(std::time::Duration::from_millis(5));
  }

  let content = String::from_utf8(content).unwrap().replace("စျ", "ဈ");

  progress_bar.finish();
  println!("[*] done downloading the original dictionary");
  std::fs::write(&dict_path, &content).unwrap();
  content
}

/// Evaluate JS code in Deno runtime.
///
/// # Arguments
///
/// * `context` - Deno runtime context
/// * `code` - JS code to evaluate
///
/// # Returns
///
/// Evaluated JS code result
fn deno_eval(context: &mut JsRuntime, code: String) -> serde_json::Value
{
  let res = context.execute_script("<anon>", code).unwrap();
  let scope = &mut context.handle_scope();
  let local = v8::Local::new(scope, res);
  // Deserialize a `v8` object into a Rust type using `serde_v8`,
  // in this case deserialize to a JSON `Value`.
  serde_v8::from_v8::<serde_json::Value>(scope, local).unwrap()
}
