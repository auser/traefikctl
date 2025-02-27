use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre::eyre;
use tracing::{debug, error, instrument};

use crate::{
    config::traefik_config::TraefikConfig,
    core::client::StoreClient,
    error::{TraefikError, TraefikResult},
    features::etcd::{Etcd, EtcdConfig, PartialEtcdConfig},
    tracing::{init_tracing, LogConfig},
    NAME,
};

mod apply;
mod clean;
mod codegen;
#[cfg(feature = "etcd")]
mod diff;
mod generate;
mod get;
mod graph;
#[cfg(feature = "etcd")]
mod load;
mod render;
#[cfg(feature = "api")]
pub(crate) mod serve;
mod show;
mod ssl;
mod tofile;
mod validate;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None, name = NAME)]
pub struct Cli {
    /// The command to run
    #[command(subcommand)]
    pub command: Commands,

    /// The log level
    #[arg(long, short = 'l', default_value = "info", global = true)]
    pub log_level: String,

    /// The config file
    #[arg(
        long,
        short = 'f',
        global = true,
        default_value = "/etc/traefikctl/traefikctl.yaml",
        env = "TRAFEIKCTL_CONFIG_FILE"
    )]
    pub config_file: Option<PathBuf>,

    /// The etcd config
    #[cfg_attr(feature = "etcd", arg(long, short = 'e'))]
    pub etcd_config: Option<String>,

    /// The variable files
    #[arg(long, short = 'v', global = true)]
    pub variable_files: Vec<String>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Get specific key or prefix
    Get(get::GetCommand),
    /// Show the current traefik configuration
    Show(show::ShowCommand),
    /// Apply the current traefik configuration
    Apply(apply::ApplyCommand),
    /// Clean the current traefik configuration
    Clean(clean::CleanCommand),
    /// Validate the current traefik configuration
    Validate,
    /// Generate a starter traefik configuration
    Generate(generate::GenerateCommand),
    #[cfg(feature = "api")]
    /// Serve the API
    Serve(serve::ServeCommand),
    /// Generate the typescript types
    Codegen(codegen::CodegenCommand),
    /// Diff the current traefik configuration
    Diff(diff::DiffCommand),
    /// Load the traefik configuration from a key-value file
    Load(load::LoadCommand),
    /// Render the traefik configuration
    Render(render::RenderCommand),
    /// Generate ssl certificates for the etcd server
    Ssl(ssl::SslCommand),
    /// Generate a graph of the traefik configuration
    Graph(graph::GraphCommand),
    /// Write the traefik configuration to a file
    ToFile(tofile::ToFileCommand),
}

#[instrument]
pub async fn run() -> TraefikResult<()> {
    color_eyre::install()?;
    let cli: Cli = Cli::parse();
    let log_level = cli.log_level.clone();
    let log_config = LogConfig {
        max_level: log_level.clone(),
        filter: format!("{}={}", NAME, &log_level),
        rolling_file_path: None,
    };
    init_tracing(NAME, &log_config)?;

    let config_file = cli.config_file.unwrap_or_default();
    debug!("Using config file: {:?}", config_file);

    // let config = std::fs::read_to_string(&config_file).unwrap_or_default();
    // let mut traefik_config: TraefikConfig = match serde_yaml::from_str(&config) {
    //     Ok(config) => config,
    //     Err(e) => {
    //         let err = eyre!("Parse error: {e}");
    //         error!("{err}");
    //         return Err(TraefikError::ParsingError(err));
    //     }
    // };

    let mut traefik_config = parse_config_file(&config_file, cli.variable_files)?;

    #[cfg(feature = "etcd")]
    let etcd_client = match cli.etcd_config {
        Some(config) => {
            let default_config = EtcdConfig::default();
            let partial_config = PartialEtcdConfig::from(config);
            let config = default_config.merge(partial_config);
            Etcd::new(&config).await?
        }
        None => Etcd::new(&traefik_config.etcd).await?,
    };

    #[cfg(feature = "etcd")]
    let client = StoreClient::new(etcd_client);

    match cli.command {
        Commands::Get(get_command) => {
            get::run(&get_command, &client, &traefik_config).await?;
        }
        Commands::Show(show_command) => {
            show::run(&show_command, &client, &traefik_config).await?;
        }
        Commands::Apply(apply_command) => {
            apply::run(&apply_command, &client, &mut traefik_config).await?;
        }
        Commands::Clean(clean_command) => {
            clean::run(&clean_command, &client, &mut traefik_config).await?;
        }
        Commands::Validate => {
            validate::run(&client, &mut traefik_config).await?;
        }
        Commands::Generate(generate_command) => {
            generate::run(&generate_command, &client, &mut traefik_config).await?;
        }
        #[cfg(feature = "api")]
        Commands::Serve(serve_command) => {
            serve::run(&serve_command, &client, &mut traefik_config).await?;
        }
        Commands::Codegen(codegen_command) => {
            codegen::run(&codegen_command, &client, &mut traefik_config).await?;
        }
        #[cfg(feature = "etcd")]
        Commands::Diff(diff_command) => {
            diff::run(&diff_command, &client, &mut traefik_config).await?;
        }
        Commands::Load(load_command) => {
            load::run(&load_command, &client, &mut traefik_config).await?;
        }
        Commands::Render(render_command) => {
            render::run(&render_command, &client, &mut traefik_config).await?;
        }
        Commands::Ssl(ssl_command) => {
            ssl::run(&ssl_command, &client, &mut traefik_config).await?;
        }
        Commands::Graph(graph_command) => {
            graph::run(&graph_command, &client, &mut traefik_config).await?;
        }
        Commands::ToFile(tofile_command) => {
            tofile::run(&tofile_command, &client, &mut traefik_config).await?;
        }
    }

    Ok(())
}

fn parse_config_file(
    config_file: &PathBuf,
    variable_files: Vec<String>,
) -> TraefikResult<TraefikConfig> {
    let config = std::fs::read_to_string(&config_file).unwrap_or_default();
    if config.is_empty() {
        return Err(TraefikError::ParsingError(eyre!("Config file is empty")));
    }
    let mut config_ctx = tera::Context::new();

    for variable_file in variable_files.iter() {
        let file = std::fs::File::open(variable_file)?;
        let variables: std::collections::HashMap<String, serde_json::Value> =
            serde_json::from_reader(file)?;
        for (key, value) in variables.iter() {
            debug!("Adding variable: {} = {:#?}", key, value);
            config_ctx.insert(key, value);
        }
    }

    match tera::Tera::one_off(&config, &config_ctx, false) {
        Ok(rendered_config) => {
            println!("-- {}", rendered_config);
            let traefik_config: TraefikConfig = match serde_yaml::from_str(&rendered_config) {
                Ok(config) => config,
                Err(e) => {
                    let err = eyre!("Parse error: {e}");
                    error!("{err}");
                    return Err(TraefikError::ParsingError(err));
                }
            };
            Ok(traefik_config)
        }
        Err(_e) => {
            let traefik_config: TraefikConfig = serde_yaml::from_str(&config)?;
            println!("-- {:?}", traefik_config);
            Ok(traefik_config)
        }
    }
}
