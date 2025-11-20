#![recursion_limit = "256"]

use brush_cli::{Cli, process_ui};
use brush_process::process::process_stream;
use tokio::runtime::Builder;

fn main() -> anyhow::Result<()> {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .init();

    let cli = Cli::parse_headless()?;
    let (sender, args_receiver) = tokio::sync::oneshot::channel();
    let _ = sender.send(cli.process.clone());

    let source = cli
        .source
        .expect("CLI validation should ensure a source is provided");

    Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async move {
            let device = brush_render::burn_init_setup().await;
            let stream = process_stream(source, args_receiver, device);
            process_ui(stream, cli.process).await
        })?;

    Ok(())
}
