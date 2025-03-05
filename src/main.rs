use clap::Parser;

mod exchange;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // value in Source currency
    value: f64,

    // Source currency in ISO-4217 format
    from_currency: String,

    // Target currency in ISO-4217 format
    #[arg(value_parser, last = true, num_args = 1.., value_delimiter = ',')]
    to_currency: Vec<String>,

    // Date in ISO-8601 format (YYYY-MM-DD)
    #[arg(short, long, default_value = "latest")]
    date: String,
}

fn main() {
    let Cli {
        from_currency,
        to_currency,
        date,
        value,
    } = Cli::parse();

    println!(
        "{:?} {:?} {:?} {:?}",
        from_currency, to_currency, date, value
    );

    let result = exchange::Exchange::new(&from_currency, &date);

    println!("{:?}", result.get_rates(value, &to_currency));
}
