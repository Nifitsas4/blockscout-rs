use crate::cli;
use clap::Parser;
use config::{Config as LibConfig, File};
use serde::Deserialize;
use std::{net::SocketAddr, num::NonZeroUsize, str::FromStr};
use url::Url;

#[derive(Deserialize, Clone, Default)]
#[serde(default)]
pub struct Config {
    pub server: ServerConfiguration,
    pub verifier: VerifierConfiguration,
    pub sourcify: SourcifyConfiguration,
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct ServerConfiguration {
    pub addr: SocketAddr,
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        Self {
            addr: SocketAddr::from_str("0.0.0.0:8043").expect("should be valid url"),
        }
    }
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct VerifierConfiguration {
    pub enabled: bool,
}

impl Default for VerifierConfiguration {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Deserialize, Clone)]
#[serde(default)]
pub struct SourcifyConfiguration {
    pub enabled: bool,
    pub api_url: Url,
    /// Number of attempts the server makes to Sourcify API.
    /// Should be at least one. Set to `3` by default.
    pub verification_attempts: NonZeroUsize,
    pub request_timeout: u64,
}

impl Default for SourcifyConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            api_url: Url::try_from("https://sourcify.dev/server/").unwrap(),
            verification_attempts: NonZeroUsize::new(3).expect("Is not zero"),
            request_timeout: 10,
        }
    }
}

impl Config {
    pub fn parse() -> Result<Self, config::ConfigError> {
        let args = cli::Args::parse();
        let mut builder =
            LibConfig::builder().add_source(config::Environment::with_prefix("VERIFICATION"));
        if args.config_path.exists() {
            builder = builder.add_source(File::from(args.config_path));
        }
        builder
            .build()
            .expect("Failed to build config")
            .try_deserialize()
    }
}