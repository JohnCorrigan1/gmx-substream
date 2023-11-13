use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("GMX", "abi/gmx.json")?
        .generate()?
        .write_to_file("src/abi/gmx.rs")?;

    Ok(())
}
