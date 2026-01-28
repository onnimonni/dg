use crate::serve::run_server;
use anyhow::Result;
use std::path::Path;

#[tokio::main]
pub async fn run(docs_dir: &str, port: u16, open: bool, watch: bool) -> Result<()> {
    let docs_path = Path::new(docs_dir);
    run_server(docs_path, port, open, watch).await
}
