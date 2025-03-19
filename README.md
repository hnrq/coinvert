[![Release](https://github.com/hnrq/coinvert/actions/workflows/release.yml/badge.svg)](https://github.com/hnrq/coinvert/actions/workflows/release.yml)

# Coinvert
A fast and flexible command-line currency converter that provides historical exchange rates.

## Project Description

Coinvert is a Rust-based CLI tool for currency conversion that uses real-time exchange rates. It allows you to quickly convert between different currencies using the command line, supports historical exchange rates, and offers various output formats to fit your needs.

## Features

- Convert between 170+ currencies using their ISO-4217 codes
- Exchange rates from [exchange-api](https://github.com/fawazahmed0/exchange-api)
- Historical exchange rates lookup
- Fast and efficient with minimal dependencies
- Simple and intuitive command-line interface
- Cross-platform support

## Installation

### Using Cargo

If you have Rust installed, you can install Coinvert directly from crates.io:

```
cargo install coinvert
```

### Using Install Script (Linux and macOS)

You can install Coinvert using our installation script:

```bash
curl -sSL https://raw.githubusercontent.com/hnrq/coinvert/main/install.sh | bash
```

This script will automatically download and install the latest version of Coinvert for your system.

### Download Prebuilt Binaries

You can download prebuilt binaries for your platform from the [releases page](https://github.com/username/coinvert/releases).

After downloading, make the binary executable and place it in your PATH:

```bash
chmod +x coinvert
sudo mv coinvert /usr/local/bin/
```

## Usage

### Basic Usage

Convert 100 USD to EUR:

```
coinvert 100 --from USD --to EUR
```

Or using short options:

```
coinvert 100 -f USD -t EUR
```

### Historical Rates

Convert 100 USD to EUR using the exchange rate from a specific date:

```
coinvert 100 -f USD -t EUR --date 2023-01-15
```

### Output Formats

Change the output format:

```
coinvert 100 -f USD -t EUR --format short
```

Available formats:
- `full` (default): Shows full human-readable text with date and currency names
- `short`: Shows only the conversion values
### Examples

Convert 50 GBP to JPY:
```
coinvert 50 -f GBP -t JPY
```

Convert 75 EUR to CAD on January 1, 2022:
```
coinvert 75 -f EUR -t CAD -d 2022-01-01
```

Get short format output for 200 AUD to NZD:
```
coinvert 200 -f AUD -t NZD --format short
```

Get full format output for 100 USD to EUR:
```
coinvert 100 -f USD -t EUR --format full
```

## Building from Source

### Prerequisites

- Rust toolchain (1.56.0 or later)
- Cargo package manager

### Build Steps

1. Clone the repository:
   ```
   git clone https://github.com/username/coinvert.git
   cd coinvert
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. The binary will be available at `target/release/coinvert`

## Technologies

Coinvert is built with:
- Rust programming language
- clap for command-line argument parsing
- reqwest for HTTP requests
- serde for serialization/deserialization

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

