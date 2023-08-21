# dl-translate

An [Enlightware® software](https://enlightware.ch).

## Overview

A Rust-based CLI to query a translation from [DeepL](https://www.deepl.com) using an API key.

You need to place your API key in the file `CONFIG/dl-translate.toml` with the following format:

```
auth_key = "KEY_UUID"
```

where `CONFIG` is the [standard config directory of your platform](https://docs.rs/dirs/3.0.1/dirs/fn.config_dir.html).

Then, [install Rust](https://www.rust-lang.org/) and build:

```
cargo build
```

You can then translate a single text snippet:

```
target/debug/dl-translate TEXT TARGET_LANG [SOURCE_LANG] [more/less (FORMALITY)]
```

Or pipe some input:

```
CMD | target/debug/dl-translate TARGET_LANG [SOURCE_LANG] [more/less (FORMALITY)]
```


where `X_LANG` is a DeepL-support [language tag](https://en.wikipedia.org/wiki/IETF_language_tag).

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.