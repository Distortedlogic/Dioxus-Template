mod configs;
use configs::dioxus_cfg::*;

use anyhow::Result;
use clap::{Parser, Subcommand};
use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use std::{fs, path::PathBuf};

macro_rules! impl_json_schema {
	($type:ty, $schema_name:expr) => {
		impl JsonSchema for $type {
			fn schema_name() -> String {
				$schema_name.into()
			}

			fn json_schema(gen: &mut SchemaGenerator) -> Schema {
				gen.subschema_for::<ConfigWrapper<$type>>()
			}
		}
	};
}

#[derive(JsonSchema)]
struct ConfigWrapper<T>(#[schemars(with = "T")] T);

impl_json_schema!(DioxusConfig, "DioxusConfig");
impl_json_schema!(ApplicationConfig, "ApplicationConfig");
impl_json_schema!(WebConfig, "WebConfig");
impl_json_schema!(WebAppConfig, "WebAppConfig");
impl_json_schema!(WebProxyConfig, "WebProxyConfig");
impl_json_schema!(WebWatcherConfig, "WebWatcherConfig");
impl_json_schema!(WebResourceConfig, "WebResourceConfig");
impl_json_schema!(WebDevResourceConfig, "WebDevResourceConfig");
impl_json_schema!(WebHttpsConfig, "WebHttpsConfig");
impl_json_schema!(WasmOptConfig, "WasmOptConfig");
impl_json_schema!(WasmOptLevel, "WasmOptLevel");
impl_json_schema!(DesktopConfig, "DesktopConfig");
impl_json_schema!(BundleConfig, "BundleConfig");
impl_json_schema!(DebianSettings, "DebianSettings");
impl_json_schema!(MacOsSettings, "MacOsSettings");
impl_json_schema!(WindowsSettings, "WindowsSettings");
impl_json_schema!(WixSettings, "WixSettings");
impl_json_schema!(NsisSettings, "NsisSettings");
impl_json_schema!(WebviewInstallMode, "WebviewInstallMode");
impl_json_schema!(NSISInstallerMode, "NSISInstallerMode");
impl_json_schema!(CustomSignCommandSettings, "CustomSignCommandSettings");

#[derive(Parser)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Schema {
		#[arg(short, long, default_value = "dioxus.schema.json")]
		output: PathBuf,
	},
}

fn main() -> Result<()> {
	let cli = Cli::parse();

	match cli.command {
		Commands::Schema { output } => {
			let schema = schemars::schema_for!(DioxusConfig);
			fs::write(&output, serde_json::to_string_pretty(&schema)?)?;
			let schema_comment = format!("# schema = \"{}\"\n", output.display());
			let config_path = PathBuf::from("Dioxus.toml");
			let content = if config_path.exists() {
				let mut content = fs::read_to_string(&config_path)?;
				if !content.starts_with("# schema") {
					content = schema_comment + &content;
				}
				content
			} else {
				schema_comment
			};
			fs::write(config_path, content)?;
		},
	}
	Ok(())
}
