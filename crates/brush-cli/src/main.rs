use anyhow::bail;
use brush_cli::{Cli, run_headless};
use clap::Parser;

fn main() -> Result<(), anyhow::Error> {
    let mut logger = env_logger::Builder::from_env(env_logger::Env::default());
    logger.target(env_logger::Target::Stdout);
    let _ = logger.try_init();

    let args = Cli::parse().validate()?;

    if args.with_viewer {
        bail!("--with-viewer is not supported by brush-cli; use the brush-app binary instead.");
    }

    let Cli {
        source,
        with_viewer: _,
        process,
    } = args;

    let source = source
        .ok_or_else(|| anyhow::anyhow!("When --with-viewer is false, --source must be provided"))?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move { run_headless(source, process).await })?;

    Ok(())
}
