use crate::{core::client::StoreClient, error::TraefikResult, features::etcd::Etcd, TraefikConfig};
use clap::Args;
use tracing::{error, info};

#[derive(Args, Debug)]
pub struct ApplyCommand {
    /// Dry run the apply command
    #[arg(short, long)]
    dry_run: bool,

    /// Clean the etcd before applying the config
    #[arg(short, long, default_value_t = false)]
    clean: bool,

    /// Show the rules that will be applied to the config
    #[arg(short, long, default_value_t = false)]
    rules: bool,
}

pub async fn run(
    command: &ApplyCommand,
    client: &StoreClient<Etcd>,
    traefik_config: &mut TraefikConfig,
) -> TraefikResult<()> {
    if command.clean && !command.dry_run {
        match traefik_config.clean_etcd(client).await {
            Ok(_) => {
                info!("Cleaned etcd");
            }
            Err(e) => {
                error!("Failed to clean etcd: {e}");
            }
        }
    }

    traefik_config
        .apply_to_etcd(client, command.dry_run, command.rules, command.clean)
        .await?;

    Ok(())
}
