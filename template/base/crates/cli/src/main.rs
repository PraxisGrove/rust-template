use anyhow::Result;
use {{crate_name}}_app::build_greeting;
use {{crate_name}}_infra::EnvRecipient;

fn main() -> Result<()> {
    let provider = EnvRecipient::from_args(std::env::args());
    println!("{}", build_greeting(&provider));

    Ok(())
}
