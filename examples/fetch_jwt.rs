use anyhow::Result;
use spire_workload::get_jwt_tokens;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    spire_workload::init();
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let svids = get_jwt_tokens(None).await;
    println!("{:?}", svids);
    Ok(())
}
