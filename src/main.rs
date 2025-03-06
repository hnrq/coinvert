use clap::{Parser, ValueEnum};
use serde::Serialize;

mod exchange;

#[derive(ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Format {
    // Full, human-readable, text
    #[default]
    Full,
    // Currency conversion values only, same order as <FROM> was provided
    Short,
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // value in Source currency
    #[arg(default_value_t = 1.0)]
    value: f64,

    // Source currency in ISO-4217 format
    #[arg(short, long, value_parser = clap::builder::PossibleValuesParser::new(exchange::currencies::VALID_CURRENCIES))]
    from: String,

    // Target currency in ISO-4217 format
    #[arg(short, long, value_parser, num_args = 1.., value_delimiter = ',', value_parser = clap::builder::PossibleValuesParser::new(exchange::currencies::VALID_CURRENCIES))]
    to: Vec<String>,

    // Date in ISO-8601 format (YYYY-MM-DD)
    #[arg(short, long, default_value = "latest")]
    date: String,

    // Output format
    #[arg(long, value_enum, default_value_t)]
    format: Format,
}

fn main() {
    let Cli {
        from,
        to,
        date,
        value,
        format,
    } = Cli::parse();

    let result = exchange::Exchange::new(&from, &date);

    match format {
        Format::Full => {
            println!("Exchange rates at {}", result.date);
            for (currency, rate) in result.get_rates(value, &to) {
                println!("{value} {from} is {:.2} {currency}", rate);
            }
        }
        Format::Short => {
            for (_, rate) in result.get_rates(value, &to) {
                println!("{:.2}", rate);
            }
        }
    }
}
