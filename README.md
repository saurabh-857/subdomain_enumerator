# SubSniffer
SubSniffer is a linux-based subdomain enumerating tool.

To run this tool in your local machine, ensure Rust is installed.

### How to run

1. clone the repository:

```bash
git clone https://github.com/saurabh-857/subsniffer.git
```

2. navigate to the directory:

```bash
cd subsniffer
```

3. install

```bash
cargo install --path .
```

Now `subsniffer` is installed for the user. It can run from anywhere.

3. run

- help
```bash
subsniffer -h
```
This will display the help the help message.

- execute
```bash
subsniffer -d example.com
```
Replace `example.com` with the domain you want to enumerate.

### Contributing

Contributions are welcome! Please check [`CONTRIBUTING.md`](./CONTRIBUTING.md) to learn how to contribute to our codebase.