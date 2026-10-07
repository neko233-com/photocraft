//! Generate the embedded registry from the same versioned files used for runtime language packs.
#![forbid(unsafe_code)]

#[allow(dead_code)]
#[path = "src/i18n/catalog.rs"]
mod catalog;
#[allow(dead_code)]
#[path = "src/i18n/config.rs"]
mod config;

use std::{path::PathBuf, process::ExitCode};

fn read(path: &std::path::Path, limit: usize) -> Result<String, String> {
    use std::io::Read;
    let file = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut text = String::new();
    file.take((limit as u64).saturating_add(1)).read_to_string(&mut text).map_err(|e| format!("{}: {e}", path.display()))?;
    if text.len() > limit {
        return Err(format!("{} exceeds {limit} bytes", path.display()));
    }
    Ok(text)
}

fn run() -> Result<(), String> {
    println!("cargo:rerun-if-changed=locales");
    let root = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").ok_or("missing CARGO_MANIFEST_DIR")?).join("locales");
    let text = read(&root.join("manifest.json"), config::MAX_MANIFEST_BYTES)?;
    let manifest = config::Manifest::parse(&text)?;
    manifest.validate_registry(&manifest.languages)?;
    let mut generated = format!("static BUNDLED_LANGUAGES: [LangInfo; {}] = [\n", manifest.languages.len());
    for language in &manifest.languages {
        let source = if let Some(file) = &language.catalog {
            let path = root.join(file);
            let contents = read(&path, config::MAX_CATALOG_BYTES)?;
            catalog::Catalog::parse_checked(&contents, language.plural_rule).map_err(|e| format!("{file}: {e}"))?;
            format!("include_str!({:?})", path.to_string_lossy())
        } else {
            "\"\"".to_string()
        };
        generated.push_str(&format!(
            "LangInfo {{ code: {:?}, name: {:?}, source: {source}, plural: config::PluralRule::{:?}, complete_menus: {}, fallback: {:?}, catalog: OnceLock::new() }},\n",
            language.code, language.name, language.plural_rule, language.complete_menus, language.fallback(&manifest.fallback_locale)
        ));
    }
    generated.push_str("];\npub static LANGUAGES: &[LangInfo] = &BUNDLED_LANGUAGES;\n");
    let out = PathBuf::from(std::env::var_os("OUT_DIR").ok_or("missing OUT_DIR")?).join("locales.rs");
    std::fs::write(out, generated).map_err(|e| format!("generate registry: {e}"))
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("PhotoCraft localisation: {error}");
            ExitCode::FAILURE
        }
    }
}
