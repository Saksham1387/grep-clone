use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
struct Cli{
    pattern:String,
    path:std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str) {
    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }
}

fn main() -> Result<()>{
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern);
    Ok(())
}

