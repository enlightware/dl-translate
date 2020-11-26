dl-translate
============

An [Enlightware software](https://enlightware.ch).

A Rust-based CLI to query a translation from [DeepL](https://www.deepl.com) using an API key.

You need to place your API key in the file `CONFIG/dl-translate.toml` with the following format:

```
auth_key = "KEY_UUID"
```

Then, [install Rust](https://www.rust-lang.org/), then run:

```
cargo run dl-translate TARGET_LANG [SOURCE_LANG] [more/less (FORMALITY)]
```

where `X_LANG` is a DeepL-support [language tag](https://en.wikipedia.org/wiki/IETF_language_tag).
