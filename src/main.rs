use clap::{Parser, ValueEnum};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use trust_dns_resolver::{
    TokioAsyncResolver,
    config::{ResolverConfig, ResolverOpts},
};

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
    help_template = "\n\n{bin} {version}\n{author}\n{about}\n\n{usage}\n\n{all-args}\n"
)]
struct Args {
    #[arg(
        long,
        short = 'd',
        required = true,
        help = "Target domain to enumerate (e.g., 'example.com')"
    )]
    domain: String,

    #[arg(long, short = 'o', help = "Output file for subdomain results")]
    output: Option<String>,

    #[arg(
        long,
        short = 'w',
        required = true,
        help = "Path to wordlist file for brute-forcing subdomains"
    )]
    wordlist: String,

    #[arg(help = "IP version to query ('4' for IPv4, '6' for IPv6, omit for both)")]
    ip_type: Option<IpType>,

    #[arg(
        short = 'v',
        long = "verbose",
        help = "Show verbose output including IP addresses"
    )]
    verbose: bool,
}

#[derive(ValueEnum, Clone)]
enum IpType {
    #[value(name = "4")]
    IPv4,
    #[value(name = "6")]
    IPv6,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut output_file = if let Some(output_path) = &args.output {
        Some(File::create(output_path)?)
    } else {
        None
    };

    println!("\n[+] Target domain: {}", args.domain);
    if args.verbose {
        println!("[+] Verbose mode enabled - showing IP addresses");
    }

    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default())?;

    let root_domain = format!("{}", args.domain);
    resolve_subdomain(&resolver, &root_domain, &mut output_file, &args.ip_type, args.verbose).await?;

    let wordlist_path = args.wordlist;
    if !Path::new(&wordlist_path).exists() {
        return Err(format!("[!] Wordlist file '{}' not found", wordlist_path).into());
    }
    let wordlist = fs::read_to_string(wordlist_path)?;
    for line in wordlist.lines() {
        let subdomain = line.trim();
        if subdomain.is_empty() {
            continue;
        }
        let fqdn = format!("{}.{}", subdomain, args.domain);
        resolve_subdomain(&resolver, &fqdn, &mut output_file, &args.ip_type, args.verbose).await?;
    }

    Ok(())
}

async fn resolve_subdomain(
    resolver: &TokioAsyncResolver,
    fqdn: &str,
    output_file: &mut Option<File>,
    ip_type: &Option<IpType>,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut found = false;

    match ip_type {
        None => {
            if let Ok(lookup) = resolver.lookup_ip(fqdn).await {
                found = true;
                if verbose {
                    for ip in lookup.iter() {
                        println!("{} -> {}", fqdn, ip);
                        if let Some(file) = output_file {
                            writeln!(file, "{} -> {}", fqdn, ip)?;
                        }
                    }
                } else {
                    println!("{}", fqdn);
                    if let Some(file) = output_file {
                        writeln!(file, "{}", fqdn)?;
                    }
                }
            }
        }
        Some(IpType::IPv4) => {
            if let Ok(lookup) = resolver.ipv4_lookup(fqdn).await {
                found = true;
                if verbose {
                    for ip in lookup.iter() {
                        println!("{} -> {}", fqdn, ip);
                        if let Some(file) = output_file {
                            writeln!(file, "{} -> {}", fqdn, ip)?;
                        }
                    }
                } else {
                    println!("{}", fqdn);
                    if let Some(file) = output_file {
                        writeln!(file, "{}", fqdn)?;
                    }
                }
            }
        }
        Some(IpType::IPv6) => {
            if let Ok(lookup) = resolver.ipv6_lookup(fqdn).await {
                found = true;
                if verbose {
                    for ip in lookup.iter() {
                        println!("{} -> {}", fqdn, ip);
                        if let Some(file) = output_file {
                            writeln!(file, "{} -> {}", fqdn, ip)?;
                        }
                    }
                } else {
                    println!("{}", fqdn);
                    if let Some(file) = output_file {
                        writeln!(file, "{}", fqdn)?;
                    }
                }
            }
        }
    }

    if !found && verbose {
        println!("{} -> Not resolved", fqdn);
        if let Some(file) = output_file {
            writeln!(file, "{} -> Not resolved", fqdn)?;
        }
    }

    Ok(())
}
