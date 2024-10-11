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

- [Rust compiler](https://www.rust-lang.org/): I use nightly. Stable channel might work too.
- [cargo-make](https://mise.jdx.dev/): Task runner (optional)

## Some Helpers

Run tests:

```
cargo make test
```

Run tests for specific component:

```
cargo make test-$component

# example
cargo make test-tokenizer
```

Note: Running tokenizer test will download `myG2P` dictionary and use it as input.

## Acknowledgement

I'd like to express my gratitude to the following projects. Without them, testing this library would have been much more difficult, if not impossible:

- [`ye-kyaw-thu/myG2P`](https://github.com/ye-kyaw-thu/myG2P): I use [`myg2p.ver2.0.txt`](https://github.com/ye-kyaw-thu/myG2P/blob/master/ver2/myg2p.ver2.0.txt) from this repo to generate test inputs.
- [`myanmaropenwordnet/mya2rom`](https://github.com/myanmaropenwordnet/mya2rom): I use [`mya2rom.js`](https://github.com/myanmaropenwordnet/mya2rom/blob/master/mya2rom.js) and [`romanisations.js`](https://github.com/myanmaropenwordnet/mya2rom/blob/master/romanisations.js) to generate test inputs by converting myG2P dictionary into MLCTS.

## License

This project is licensed under the [MIT](LICENSE) License.

This project uses [CC-BY-NC-SA-4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) licensed items such as [myg2p.ver2.0.txt](https://github.com/ye-kyaw-thu/myG2P/blob/master/ver2/myg2p.ver2.0.txt) from [myG2P](https://github.com/ye-kyaw-thu/myG2P) repo for testing. Currently I don't distribute or embed those items directly in my project, I think I can license this project under [MIT](LICENSE) license. Let me know if I'm wrong. In the future, licensing of this project has to be reconsidered.
