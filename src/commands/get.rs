use clap::Args;
use tracing::debug;

use crate::{config::TraefikConfig, error::TraefikResult, etcd::Etcd};

#[derive(Args, Debug)]
pub struct GetCommand {
    #[arg(index(1))]
    name: String,

    #[arg(short, long)]
    keys: bool,
}

pub async fn run(
    command: &GetCommand,
    etcd_client: &Etcd,
    _traefik_config: &TraefikConfig,
) -> TraefikResult<()> {
    let key = format!("{}", command.name);
    let values: Vec<_> = if command.keys {
        etcd_client.get_keys(key).await?
    } else {
        etcd_client.get_with_prefix(key).await?
    };

    debug!("values: {:?}", values);

    let values = values
        .iter()
        .map(|v| v.value_str().unwrap_or_default())
        .collect::<Vec<&str>>();
    let value_str = values.join("\n");
    println!("{}", value_str);

    Ok(())
}
