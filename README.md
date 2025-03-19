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

## Installation

**Important: Coinvert currently only supports x86_64 architecture. It will not work on ARM-based systems (like Raspberry Pi or Apple Silicon M1/M2 without Rosetta).**

### Using Install Script (Linux and macOS)

The fastest way to install Coinvert on x86_64 systems is using our installation script:

```bash
curl -sSL https://raw.githubusercontent.com/hnrq/coinvert/main/install.sh | bash
```

This script will:
- Automatically detect your operating system
- Download the appropriate prebuilt binary for your system
- Install it to `/usr/local/bin/` (may require sudo permission)
- Make it executable and ready to use

### Using Cargo

If you have Rust installed, you can install Coinvert directly from crates.io:

```bash
cargo install coinvert
```

### Manual Installation (Prebuilt Binaries)

If you prefer to manually install, you can download prebuilt binaries for x86_64 Linux and macOS from the [releases page](https://github.com/hnrq/coinvert/releases).

After downloading:
1. Make the binary executable
2. Move it to a directory in your PATH

```bash
chmod +x coinvert
sudo mv coinvert /usr/local/bin/
```

This method gives you control over the installation process and allows you to verify the binary before installation.

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

