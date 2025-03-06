use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
struct Currencies {
    #[serde(flatten)]
    currencies: HashMap<String, String>,
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let currencies: Vec<String> = reqwest::blocking::get(
        "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@latest/v1/currencies.json",
    )
    .unwrap()
    .json::<Currencies>()
    .unwrap()
    .currencies
    .iter()
    .map(|(k, _v)| k.to_uppercase())
    .collect();

    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let path = std::path::Path::new(&out_dir).join("currencies.rs");

    std::fs::write(
        &path,
        format!(
            "pub static VALID_CURRENCIES:[&'static str; {}] = {:?};",
            currencies.len(),
            currencies,
        ),
    )
    .unwrap();
}
