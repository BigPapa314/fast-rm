use anyhow::Result;
use fast_rm::{Config, FastRm};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(args[1].clone());
    let frm = FastRm::new(config);

    frm.run().await?;

    Ok(())
}
