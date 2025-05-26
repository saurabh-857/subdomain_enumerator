use clap::Parser;
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};

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
    domain: String,

    #[arg(
        long,
        short = 'w',
        help = "Path to subdomain wordlist file"
    )]
    wordlist: Option<String>,
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

    let lookup = resolver.lookup_ip(format!("www.{}", args.domain)).await?;
    println!("[+] Found IP addresses for {} are: ", args.domain);
    for ip in lookup.iter() {
        println!(" - {}", ip);
    }

    Ok(())
}
