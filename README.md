# Subdomain-Enumerator

Subdomain-Enumerator is a Rust-based tool for enumerating subdomains of a given domain and resolving their A (IPv4) and AAAA (IPv6) DNS records.

To run this tool in your local machine, ensure Rust is installed.

### How to run

1. clone the repository:

```bash
git clone https://github.com/saurabh-857/subdomain_enumerator.git
```

2. navigate to the directory:

```bash
cd subdomain_enumerator
```

3. run

- help
	```bash
	cargo run -- -h
	```
	This will display the help the help message.
	
- execute
	```bash
	cargo run -- -d example.com
	```
	Replace `example.com` with the domain you want to enumerate.

### Contributing
Contributions are welcome! Please check [`CONTRIBUTING.md`](./CONTRIBUTING.md) to learn how to contribute to our codebase.