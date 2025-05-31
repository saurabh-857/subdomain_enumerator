use clap::Parser;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};
use std::fs::File;
use std::io::Write;

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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut output_file = File::create("subdomains.txt")?;

    println!("\n[+] Target domain: {}", args.domain);

    let resolver = TokioAsyncResolver::tokio(
        ResolverConfig::default(),
        ResolverOpts::default(),
    )?;
    println!("[+] DNS resolver initialized: {:?}", resolver);

    let root_domain = format!("{}", args.domain);
    resolve_subdomain(&resolver, &root_domain, &mut output_file).await?;

    let common_subdomains = vec!["www", "mail", "ftp", "login", "api", "test", "help", "support", "docs", "pay", "billing", "mobile", "m"];

    for subdomain in common_subdomains {
        let fqdn = format!("{}.{}", subdomain, args.domain);
        resolve_subdomain(&resolver, &fqdn, &mut output_file).await?;
    }

    Ok(())
}

async fn resolve_subdomain (
    resolver: &TokioAsyncResolver,
    fqdn: &str,
    output_file: &mut File,
) -> Result<(), Box<dyn std::error::Error>> {
    match resolver.lookup_ip(fqdn).await {
        Ok(lookup) => {
            // println!("\n[+] Found IP address for {}.{} are:", subdomain, domain);
            for ip in lookup.iter() {
                println!("{} -> {}", fqdn, ip);
                writeln!(output_file, "{} -> {}", fqdn, ip)?;
            }
        }
        Err(_) => {},
    }
    Ok(())
}
