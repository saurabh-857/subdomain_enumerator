# SubSniffer

![Rust](https://img.shields.io/badge/Rust-1.87-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

SubSniffer is a lightweight tool for enumerating subdomains of a target domain using DNS resolution. Built in Rust, it supports both IPv4 and IPv6 lookups and can use custom wordlists for brute-forcing subdomains.

## Features

- Enumerate subdomains for a given domain
- Support for IPv4, IPv6, or both (default) DNS lookups
- Custom wordlist support for brute-forcing
- Output results to a file
- Fast and efficient using asynchronous DNS resolution
- Easy-to-use command-line interface

## Prerequisites

To run SubSniffer, ensure you have installed latest version of [Rust](https://www.rust-lang.org/tools/install) (recommended rustc 1.87.0).

## Installation

1. **Clone the repository**:
    
    ```bash
    git clone https://github.com/saurabh-857/subsniffer.git
    ```
    
2. **Navigate to the project directory**:
    
    ```bash
    cd subsniffer
    ```
    
3. **Install the tool**:
    
    ```bash
    cargo install --path .
    ```
    
    This installs `subsniffer` globally for the user, allowing it to be run from any directory.
    

## Usage

### Command-Line Options

Run `subsniffer -h` to display the help message:

```bash
subsniffer -h
```

This will show available options and their descriptions:

![help screenshot](images/help.png)

### Examples

1. **Enumerate subdomains with default settings** (uses common subdomains, queries both IPv4 and IPv6):
    
    ```bash
    subsniffer -d example.com
    ```
    
2. **Enumerate subdomains with a custom wordlist**:
    
    ```bash
    subsniffer -d example.com -w wordlist.txt
    ```
    
3. **Enumerate subdomains for IPv4 only**:
    
    ```bash
    subsniffer -d example.com 4
    ```
    
4. **Enumerate subdomains for IPv6 only**:
    
    ```bash
    subsniffer -d example.com 6
    ```
    
5. **Specify a custom output file**:
    
    ```bash
    subsniffer -d example.com -o results.txt
    ```
    

### Example Output

Running `subsniffer -d example.com` might produce:

```
[+] Target domain: example.com
example.com -> 93.184.216.34
www.example.com -> 93.184.216.34
```

Results are also saved to the specified output file (e.g., `subdomains.txt`).

## Uninstallation

To remove SubSniffer from your system:

```bash
cargo uninstall subsniffer
```

## Contributing

Contributions are welcome! Whether it's reporting bugs, suggesting features, or submitting code, we appreciate your help. Please read our [CONTRIBUTING.md](./CONTRIBUTING.md) file for guidelines on how to contribute.

## License

This project is licensed under the MIT License. See the [LICENSE.md](./LICENSE.md) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- Inspired by the need for a fast and simple subdomain enumeration tool