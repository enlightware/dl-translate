use anyhow::{anyhow, Result};
use atty::Stream;
use reqwest::header::AUTHORIZATION;
use serde::Deserialize;
use std::{
    env, fs,
    io::{self, Read},
    process::exit,
};

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
    human_print: bool,
) -> Result<()> {
    let filename = dirs::config_dir()
        .ok_or_else(|| anyhow!("Cannot get config file directory"))?
        .join("dl-translate.toml");
    let contents = fs::read_to_string(&filename)
        .map_err(|old| anyhow!("Cannot open config file {:?}: {}", &filename, old))?;
    let config = toml::from_str::<Config>(&contents)?;
    let client = reqwest::blocking::Client::new();
    let form = reqwest::blocking::multipart::Form::new()
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
        .header(AUTHORIZATION, format!("DeepL-Auth-Key {}", config.auth_key))
        .multipart(form)
        .send()?;
    if !res.status().is_success() {
        return Err(anyhow!(
            "Error during translation: {}",
            res.text().unwrap_or_else(|_| String::from("Unknown error"))
        ));
    }
    let json = res.json::<TranslationAnswer>()?;
    for translation in json.translations {
        if human_print {
            println!(
                "{} (from {})",
                translation.text, translation.detected_source_language
            );
        } else {
            print!("{}", translation.text);
        }
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let stdin_not_redirected = atty::is(Stream::Stdin);
    let result = if stdin_not_redirected {
        if args.len() < 3 {
            eprintln!(
                "Not enough argument, usage: {} TEXT TARGET_LANG [SOURCE_LANG] [more/less (FORMALITY)]",
                &args[0]
            );
            exit(1);
        }
        translate(&args[1], &args[2], args.get(3), args.get(4), true)
    } else {
        if args.len() < 2 {
            eprintln!(
                "Not enough argument, usage: {} TARGET_LANG [SOURCE_LANG] [more/less (FORMALITY)]",
                &args[0]
            );
            exit(1);
        }
        let mut text = String::new();
        io::stdin()
            .read_to_string(&mut text)
            .expect("Cannot read redirected input");
        translate(&text, &args[1], args.get(2), args.get(3), false)
    };
    if let Err(err) = result {
        eprintln!("Error during translation: {}", err);
        exit(2);
    }
}
