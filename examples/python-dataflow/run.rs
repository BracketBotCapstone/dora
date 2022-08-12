use eyre::{bail, Context};
use std::{env, path::Path};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    std::env::set_current_dir(root.join(file!()).parent().unwrap())
        .wrap_err("failed to set working dir")?;

    build_package("dora-runtime").await?;

    install_python_dependencies(root).await?;

    let dataflow = if env::var("CI").is_ok() {
        Path::new("dataflow_without_webcam.yml").to_owned()
    } else {
        Path::new("dataflow.yml").to_owned()
    };

    dora_coordinator::run(dora_coordinator::Command::Run {
        dataflow,
        runtime: Some(root.join("target").join("release").join("dora-runtime")),
    })
    .await?;

    Ok(())
}

async fn build_package(package: &str) -> eyre::Result<()> {
    let cargo = std::env::var("CARGO").unwrap();
    let mut cmd = tokio::process::Command::new(&cargo);
    cmd.arg("build").arg("--release");
    cmd.arg("--package").arg(package);
    if !cmd.status().await?.success() {
        bail!("failed to build {package}");
    };
    Ok(())
}

async fn install_python_dependencies(root: &Path) -> eyre::Result<()> {
    let mut install = tokio::process::Command::new("sh");
    install.arg("./install.sh");
    if !install.status().await?.success() {
        bail!("failed to create venv");
    };
    Ok(())
}
