# LibMLCTS

An experimental implementation of The Myanmar Language Commission Transcription System (1980), also known as the MLC Transcription System (MLCTS).

## Motivation

In Myanmar, many people use Myanglish or Burglish (Romanized versions of Myanmar) on social networks and other places.
However, there is no standardized system for this Romanization, making it difficult for me to read comfortably.
Additionally, I admire how the Pinyin keyboard simplifies Mandarin input and am curious if I can create a similar one as a challenge for myself.

## Current Progress

- [x] Tokenizer (MLCTS to tokens)
- [ ] Basic Myanmar script generator (tokens to Myanmar script without ambiguity resolution)
- [ ] Stacked consonant recognition
- [ ] Ambiguity resolver (Resolve ambiguous MLCTS token to correct Myanmar script )

## Required Tools

- [Rust compiler](https://www.rust-lang.org/): I use nightly. Stable might works too.
- [Deno](https://deno.com/): To run scripts to prepare dictionary (optional)
- [mise-en-place](https://mise.jdx.dev/): Task runner (optional)

## Acknowledgement

I'd like to express my gratitude to the following projects. Without them, testing this library would have been much more difficult, if not impossible:

- [myG2P](https://github.com/ye-kyaw-thu/myG2P): I got a good dictionary ([assets/myg2p-dict.txt](assets/myg2p-dict.txt)) from this repo.
- [mya2rom](https://github.com/myanmaropenwordnet/mya2rom): I used `mya2rom.js` to generate test inputs by converting myG2P dictionary into MLCTS.

## License

This project is dual-licensed under the [MIT](LICENSE-CODE) License and the [CC BY-NC-SA 4.0](LICENSE) License.

The following components are licensed under [CC BY-NC-SA 4.0](LICENSE):

- [assets/myg2p-dict.txt](assets/myg2p-dict.txt) - the original myG2P dictionary.
- [assets/myg2p-dict-mlcts.csv](assets/myg2p-dict-mlcts.csv) - the modified dictionary with text written in MLCTS.
- [tools](tools/) - scripts used to generate dictionary and input csv.
- [tests](tests) - unit tests and test inputs which interact with the myG2P dictionary or its derivative works.

All other parts of the project are licensed under the [MIT](LICENSE-CODE) License, unless otherwise specified explicitly.
