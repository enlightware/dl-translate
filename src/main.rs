use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{env, fs, process::exit};

#[derive(Deserialize)]
struct Config {
    auth_key: String,
}

#[derive(Deserialize)]
struct Translation {
    detected_source_language: String,
    text: String,
}

#[derive(Deserialize)]
struct TranslationAnswer {
    translations: Vec<Translation>,
}

fn translate(
    text: &str,
    target_lang: &str,
    source_lang: Option<&String>,
    formality: Option<&String>,
) -> Result<()> {
    let filename = dirs::config_dir()
        .ok_or_else(|| anyhow!("Cannot get config file directory"))?
        .join("dl-translate.toml");
    let contents = fs::read_to_string(&filename)
        .map_err(|old| anyhow!("Cannot open config file {:?}: {}", &filename, old))?;
    let config = toml::from_str::<Config>(&contents)?;
    let client = reqwest::blocking::Client::new();
    let form = reqwest::blocking::multipart::Form::new()
        .text("auth_key", config.auth_key)
        .text("text", String::from(text))
        .text("target_lang", String::from(target_lang));
    let form = match source_lang {
        Some(lang) => form.text("source_lang", String::from(lang)),
        None => form,
    };
    let form = match formality {
        Some(formality) => form.text("formality", String::from(formality)),
        None => form,
    };
    let res = client
        .post("https://api.deepl.com/v2/translate")
        .multipart(form)
        .send()?;
    let json = res.json::<TranslationAnswer>()?;
    for translation in json.translations {
        println!(
            "{} (from {})",
            translation.text, translation.detected_source_language
        );
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!(
            "Not enough argument, usage: {} TEXT TARGET_LANG [SOURCE_LANG] [more/less (FORMALITY)]",
            &args[0]
        );
        exit(1);
    }
    if let Err(err) = translate(&args[1], &args[2], args.get(3), args.get(4)) {
        println!("Error during translation: {}", err);
        exit(2);
    }
}
