use serde::Deserialize;
use std::collections::HashMap;

pub mod currencies {
    include!(concat!(env!("OUT_DIR"), "/currencies.rs"));
}

#[derive(Deserialize, Debug)]
pub struct Exchange {
    #[serde(skip_deserializing)]
    currency: String,

    pub date: String,

    #[serde(flatten)]
    rates: HashMap<String, HashMap<String, f64>>,
}

impl Exchange {
    pub fn new(currency: &str, date: &str) -> Self {
        let mut response = reqwest::blocking::get(format!(
            "https://cdn.jsdelivr.net/npm/@fawazahmed0/currency-api@{date}/v1/currencies/{}.json",
            currency.to_lowercase()
        ))
        .unwrap()
        .json::<Exchange>()
        .unwrap();

        response.currency = currency.to_lowercase();

        response
    }

    pub fn get_rates(&self, value: f64, target_currencies: &Vec<String>) -> HashMap<String, f64> {
        self.rates
            .get(&self.currency)
            .unwrap()
            .iter()
            .filter(|&(k, _v)| target_currencies.contains(&k.to_uppercase()))
            .map(|(k, v)| (k.to_uppercase(), v * value))
            .collect()
    }
}
