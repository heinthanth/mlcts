import * as path from "jsr:@std/path";
import { stringify } from "jsr:@std/csv";
import progress from "https://esm.sh/cli-progress@3.12.0";

// forgive me for writing unreadable code

// converter
const baseURL = "https://raw.githubusercontent.com/myanmaropenwordnet/mya2rom";
const my2rom = baseURL + "/refs/heads/master/mya2rom.js";
const romanisations = baseURL + "/refs/heads/master/romanisations.js";
const rm_code = await fetch(romanisations).then((response) => response.text());
const my2rom_content = await fetch(my2rom).then((response) => response.text());

// code template for converter
const mk_code = (content: string) => `
  ${rm_code}
  ${my2rom_content.replaceAll("for (letter in", "for (const letter in")}
  mya2rom_all(${JSON.stringify(content)})`;

// convert Burmese script to MLCTS
const cvrt = (c: string) => eval(mk_code(c))[1].replaceAll(" ", "");
const cvrt_syl = (c: string) => c.split(" ").map(cvrt);
const cvrt_ent = (w: string, e: string) => [...cvrt_syl(e), cvrt(w)];
const proc_ent = ([, w, e]: string[]) => cvrt_ent(w, e);
const syl = (s: string) => [s.split(" ").length, ...s.split(" ")];
const prepare_row = (d: string[]) => [...syl(d[2]), d[1], ...proc_ent(d)];

// data
const __dirname = new URL(".", import.meta.url).pathname;
const dict_path = path.join(__dirname, "..", "assets", "myg2p-dict.txt");
const myg2p_dict = Deno.readTextFileSync(dict_path);

const dict_entries = myg2p_dict
  .split("\n")
  .filter((l) => l.trim().length > 0)
  .map((line) => line.split("\t"));

// write utils
const csv_path = path.join(__dirname, "..", "assets", "myg2p-dict-mlcts.csv");
Deno.writeTextFileSync(csv_path, "");
const f = Deno.openSync(csv_path, { create: true, append: true });
const e = new TextEncoder();

const convert_r = (r: string[]) => stringify([prepare_row(r)]);
const w_row = (r: string[]) => f.write(e.encode(convert_r(r)));

// progress bar
const preset = progress.Presets.shades_classic;
const opt = { format: "[*] Progress [{bar}] {percentage}% | {value}/{total}" };
const bar = new progress.SingleBar(opt, preset);

// Conversion process
console.log("[*] Converting dict to MLCTS ...");

const entries = dict_entries.slice(4);
const ent_len = entries.length;

bar.start(ent_len, 0);
entries.forEach((ent, i) => (bar.update(i + 1), w_row(ent)));
bar.stop();
console.log("[*] Done");
