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

    pub fn get_rates<T: AsRef<str> + std::fmt::Display>(
        &self,
        value: f64,
        target_currencies: &[T],
    ) -> HashMap<String, f64> {
        let currency_rates = self.rates.get(&self.currency).unwrap();
        target_currencies
            .iter()
            .map(|target_currency| {
                (
                    target_currency.to_string(),
                    currency_rates
                        .get(&target_currency.to_string().to_lowercase())
                        .unwrap()
                        * value,
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn test_get_rates() {
        let mut rng = rand::rng();
        let currency = String::from("USD");
        let mut rates = HashMap::<String, HashMap<String, f64>>::new();
        let target_currencies = ["BRL", "EUR", "CNY"];
        let usd_rates: HashMap<String, f64> = target_currencies
            .iter()
            .map(|c| (c.to_lowercase(), rng.random::<f64>()))
            .collect();
        rates.insert(currency.clone(), usd_rates.clone());
        let value: f64 = rng.random::<f64>();

        let exchange = Exchange {
            currency,
            date: String::from("2023-10-10"),
            rates,
        };

        let result = exchange.get_rates(value, &vec!["BRL", "EUR", "CNY"]);
        for currency in target_currencies {
            assert_eq!(
                *result.get(currency).unwrap(),
                usd_rates.get(&currency.to_lowercase()).unwrap() * value
            );
        }
    }
}
