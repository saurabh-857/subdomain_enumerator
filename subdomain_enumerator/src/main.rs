use clap::Parser;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};
// use std::io::Write;

#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
    help_template = "\n\n{bin} {version}\n{author}\n{about}\n\n{usage}\n\n{all-args}\n"
)]
struct Args{
    #[arg(
        long,
        short = 'd',
        required = true,
        // default_value = "example.com",
        help = "Target domain to enumerate (e.g., 'example.com')"
    )]
    domain: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    println!("\n[+] Target domain: {}", args.domain);

    let resolver = TokioAsyncResolver::tokio(
        ResolverConfig::default(),
        ResolverOpts::default(),
    )?;
    println!("[+] DNS resolver initialized: {:?}", resolver);

    let lookup = resolver.lookup_ip(format!("{}", args.domain)).await?;
    println!("\n[+] Found IP addresses for {} are: ", args.domain);
    for ip in lookup.iter() {
        println!(" - {}", ip);
    }
    
    let common_subdomains = vec!["www", "mail", "ftp", "login", "api", "test", "help", "support", "docs", "pay", "billing", "mobile", "m"];

    for subdomain in common_subdomains {
        match resolver.lookup_ip(format!("{}.{}", subdomain, args.domain)).await {
            Ok(lookup) => {
                println!{"\n[+] Found IP address for {}.{} are:", subdomain, args.domain};
                for ip in lookup.iter() {
                    println!(" - {}", ip);
                }
            }
            Err(e) => println!("\n[!] Error resolving {}.{}: {}", subdomain, args.domain, e),
        }
    }

    Ok(())
}
