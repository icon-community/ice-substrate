#![allow(clippy::borrowed_box)]

use crate::{
	chain_spec,
	cli::{Cli, RelayChainCli, Subcommand},
	primitives::Block,
	service::parachain,
	service::parachain::{arctic, snow, start_arctic_node, start_snow_node},
	service::solo,
};
use codec::Encode;
use cumulus_client_service::genesis::generate_genesis_block;
use cumulus_primitives_core::ParaId;
use log::info;
use sc_cli::{
	ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
	NetworkParams, Result, RuntimeVersion, SharedParams, SubstrateCli,
};
use sc_service::{
	config::{BasePath, PrometheusConfig},
	PartialComponents,
};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::traits::Block as BlockT;
use std::{io::Write, net::SocketAddr};

use crate::chain_spec::snow::{snow_kusama_config, snow_staging_rococo_config};
#[cfg(feature = "runtime-benchmarks")]
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};

trait IdentifyChain {
	fn is_snow(&self) -> bool;
	fn is_arctic(&self) -> bool;
	fn is_dev(&self) -> bool;
}

impl IdentifyChain for dyn sc_service::ChainSpec {
	fn is_snow(&self) -> bool {
		self.id().starts_with("snow")
	}
	fn is_arctic(&self) -> bool {
		self.id().starts_with("arctic")
	}
	fn is_dev(&self) -> bool {
		self.id().starts_with("dev")
	}
}

impl<T: sc_service::ChainSpec + 'static> IdentifyChain for T {
	fn is_arctic(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_arctic(self)
	}
	fn is_snow(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_snow(self)
	}
	fn is_dev(&self) -> bool {
		<dyn sc_service::ChainSpec>::is_dev(self)
	}
}

fn load_spec(id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
	Ok(match id {
		"dev" => Box::new(chain_spec::frost::development_config()?),
		"arctic-dev" => Box::new(chain_spec::arctic::get_dev_chain_spec()),
		"arctic" => Box::new(chain_spec::arctic::get_chain_spec()),
		"snow-dev" => Box::new(chain_spec::snow::get_dev_chain_spec()),
		"snow-testnet" => Box::new(chain_spec::snow::testnet_spec()),
		"snow-kusama" => Box::new(snow_kusama_config()?),
		"snow-staging-rococo" => Box::new(snow_staging_rococo_config()?),

		path => {
			let chain_spec = chain_spec::snow::SnowChainSpec::from_json_file(path.into())?;
			if chain_spec.is_snow() {
				Box::new(chain_spec::snow::SnowChainSpec::from_json_file(
					path.into(),
				)?)
			} else if chain_spec.is_arctic() {
				Box::new(chain_spec::arctic::ArcticChainSpec::from_json_file(
					path.into(),
				)?)
			} else {
				Box::new(chain_spec::frost::FrostChainSpec::from_json_file(
					std::path::PathBuf::from(path),
				)?)
			}
		}
	})
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"SNOW".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		env!("CARGO_PKG_DESCRIPTION").into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/web3labs/ice-substrate/issues".into()
	}

	fn copyright_start_year() -> i32 {
		2022
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		load_spec(id)
	}

	fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		if chain_spec.is_snow() {
			&snow_runtime::VERSION
		} else if chain_spec.is_arctic() {
			&arctic_runtime::VERSION
		} else if chain_spec.is_dev() {
			&frost_runtime::VERSION
		} else {
			unreachable!();
		}
	}
}

impl SubstrateCli for RelayChainCli {
	fn impl_name() -> String {
		"SNOW Collator".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		"Snow parachain collator\n\nThe command-line arguments provided first will be \
        passed to the parachain node, while the arguments provided after -- will be passed \
        to the relaychain node.\n\n\
        ice-node [parachain-args] -- [relaychain-args]"
			.into()
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/web3labs/ice-substrate/issues".into()
	}

	fn copyright_start_year() -> i32 {
		2022
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn sc_service::ChainSpec>, String> {
		polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
	}

	fn native_runtime_version(chain_spec: &Box<dyn ChainSpec>) -> &'static RuntimeVersion {
		polkadot_cli::Cli::native_runtime_version(chain_spec)
	}
}

fn extract_genesis_wasm(chain_spec: &Box<dyn sc_service::ChainSpec>) -> Result<Vec<u8>> {
	let mut storage = chain_spec.build_storage()?;

	storage
		.top
		.remove(sp_core::storage::well_known_keys::CODE)
		.ok_or_else(|| "Could not find wasm file in genesis state!".into())
}

// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		}
		Some(Subcommand::CheckBlock(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			if runner.config().chain_spec.is_snow() {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						import_queue,
						..
					} = parachain::new_partial::<snow::RuntimeApi, snow::Executor, _>(
						&config,
						parachain::build_import_queue,
					)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			} else if runner.config().chain_spec.is_arctic() {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						import_queue,
						..
					} = parachain::new_partial::<arctic::RuntimeApi, arctic::Executor, _>(
						&config,
						parachain::build_import_queue,
					)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			} else {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						import_queue,
						..
					} = solo::new_partial(&config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			}
		}
		Some(Subcommand::ExportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			if runner.config().chain_spec.is_snow() {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						..
					} = parachain::new_partial::<snow::RuntimeApi, snow::Executor, _>(
						&config,
						parachain::build_import_queue,
					)?;
					Ok((cmd.run(client, config.database), task_manager))
				})
			} else if runner.config().chain_spec.is_arctic() {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						..
					} = parachain::new_partial::<arctic::RuntimeApi, arctic::Executor, _>(
						&config,
						parachain::build_import_queue,
					)?;
					Ok((cmd.run(client, config.database), task_manager))
				})
			} else {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						..
					} = solo::new_partial(&config)?;
					Ok((cmd.run(client, config.database), task_manager))
				})
			}
		}
		Some(Subcommand::ExportState(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			if runner.config().chain_spec.is_snow() {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						..
					} = parachain::new_partial::<snow::RuntimeApi, snow::Executor, _>(
						&config,
						parachain::build_import_queue,
					)?;
					Ok((cmd.run(client, config.chain_spec), task_manager))
				})
			} else if runner.config().chain_spec.is_arctic() {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						..
					} = parachain::new_partial::<arctic::RuntimeApi, arctic::Executor, _>(
						&config,
						parachain::build_import_queue,
					)?;
					Ok((cmd.run(client, config.chain_spec), task_manager))
				})
			} else {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						..
					} = solo::new_partial(&config)?;
					Ok((cmd.run(client, config.chain_spec), task_manager))
				})
			}
		}
		Some(Subcommand::ImportBlocks(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			if runner.config().chain_spec.is_snow() {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						import_queue,
						..
					} = parachain::new_partial::<snow::RuntimeApi, snow::Executor, _>(
						&config,
						parachain::build_import_queue,
					)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			} else if runner.config().chain_spec.is_arctic() {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						import_queue,
						..
					} = parachain::new_partial::<arctic::RuntimeApi, arctic::Executor, _>(
						&config,
						parachain::build_import_queue,
					)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			} else {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						import_queue,
						..
					} = solo::new_partial(&config)?;
					Ok((cmd.run(client, import_queue), task_manager))
				})
			}
		}
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			runner.sync_run(|config| {
				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()]
						.iter()
						.chain(cli.relaychain_args.iter()),
				);
				let polkadot_config = SubstrateCli::create_configuration(
					&polkadot_cli,
					&polkadot_cli,
					config.tokio_handle.clone(),
				)
				.map_err(|err| format!("Relay chain argument error: {}", err))?;

				cmd.run(config, polkadot_config)
			})
		}
		Some(Subcommand::Revert(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			if runner.config().chain_spec.is_snow() {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						backend,
						..
					} = parachain::new_partial::<snow::RuntimeApi, snow::Executor, _>(
						&config,
						parachain::build_import_queue,
					)?;
					Ok((cmd.run(client, backend, None), task_manager))
				})
			} else if runner.config().chain_spec.is_arctic() {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						backend,
						..
					} = parachain::new_partial::<arctic::RuntimeApi, arctic::Executor, _>(
						&config,
						parachain::build_import_queue,
					)?;
					Ok((cmd.run(client, backend, None), task_manager))
				})
			} else {
				runner.async_run(|config| {
					let PartialComponents {
						client,
						task_manager,
						backend,
						..
					} = solo::new_partial(&config)?;
					Ok((cmd.run(client, backend, None), task_manager))
				})
			}
		}
		Some(Subcommand::ExportGenesisState(params)) => {
			let mut builder = sc_cli::LoggerBuilder::new("");
			builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
			let _ = builder.init();

			let spec = cli.load_spec(&params.chain.clone().unwrap_or_default())?;
			let state_version = Cli::native_runtime_version(&spec).state_version();

			let block: Block = generate_genesis_block(&spec, state_version)?;
			let raw_header = block.header().encode();
			let output_buf = if params.raw {
				raw_header
			} else {
				format!("0x{:?}", HexDisplay::from(&block.header().encode())).into_bytes()
			};

			if let Some(output) = &params.output {
				std::fs::write(output, output_buf)?;
			} else {
				std::io::stdout().write_all(&output_buf)?;
			}

			Ok(())
		}
		Some(Subcommand::ExportGenesisWasm(params)) => {
			let mut builder = sc_cli::LoggerBuilder::new("");
			builder.with_profiling(sc_tracing::TracingReceiver::Log, "");
			let _ = builder.init();

			let raw_wasm_blob =
				extract_genesis_wasm(&cli.load_spec(&params.chain.clone().unwrap_or_default())?)?;
			let output_buf = if params.raw {
				raw_wasm_blob
			} else {
				format!("0x{:?}", HexDisplay::from(&raw_wasm_blob)).into_bytes()
			};

			if let Some(output) = &params.output {
				std::fs::write(output, output_buf)?;
			} else {
				std::io::stdout().write_all(&output_buf)?;
			}

			Ok(())
		}
		Some(Subcommand::Key(cmd)) => cmd.run(&cli),
		Some(Subcommand::Sign(cmd)) => cmd.run(),
		Some(Subcommand::Verify(cmd)) => cmd.run(),
		Some(Subcommand::Vanity(cmd)) => cmd.run(),
		#[cfg(feature = "runtime-benchmarks")]
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;
			let is_arctic = chain_spec.is_arctic();
			let is_snow = chain_spec.is_snow();
			info!("Starting benchmarking");
			match cmd {
				BenchmarkCmd::Pallet(cmd) => {
					info!("Benchmarking for pallet");
					if cfg!(feature = "runtime-benchmarks") {
						info!("Runtime benchmarking enabled");
						if is_arctic {
							info!("running pallet benchmarking for arctic");
							runner.sync_run(|config| {
								cmd.run::<arctic_runtime::Block, arctic::Executor>(config)
							})
						} else if is_snow {
							info!("running pallet benchmarking for snow");
							runner.sync_run(|config| {
								cmd.run::<snow_runtime::Block, snow::Executor>(config)
							})
						} else {
							info!("running pallet benchmarking for frost");
							runner.sync_run(|config| {
								cmd.run::<frost_runtime::Block, solo::ExecutorDispatch>(config)
							})
						}
					} else {
						info!("error no benchmarking enabled");
						Err("Benchmarking wasn't enabled when building the node. \
                You can enable it with `--features runtime-benchmarks`."
							.into())
					}
				}
				BenchmarkCmd::Block(cmd) => runner.sync_run(|config| {
					if is_arctic {
						let partials = parachain::new_partial::<
							arctic::RuntimeApi,
							arctic::Executor,
							_,
						>(&config, parachain::build_import_queue)?;
						cmd.run(partials.client)
					} else if is_snow {
						let partials = parachain::new_partial::<snow::RuntimeApi, snow::Executor, _>(
							&config,
							parachain::build_import_queue,
						)?;
						cmd.run(partials.client)
					} else {
						let partials = solo::new_partial(&config)?;
						cmd.run(partials.client)
					}
				}),
				BenchmarkCmd::Storage(cmd) => runner.sync_run(|config| {
					if is_arctic {
						let partials = parachain::new_partial::<
							arctic::RuntimeApi,
							arctic::Executor,
							_,
						>(&config, parachain::build_import_queue)?;
						let db = partials.backend.expose_db();
						let storage = partials.backend.expose_storage();
						cmd.run(config, partials.client.clone(), db, storage)
					} else if is_snow {
						let partials = parachain::new_partial::<snow::RuntimeApi, snow::Executor, _>(
							&config,
							parachain::build_import_queue,
						)?;
						let db = partials.backend.expose_db();
						let storage = partials.backend.expose_storage();
						cmd.run(config, partials.client.clone(), db, storage)
					} else {
						let partials = solo::new_partial(&config)?;
						let db = partials.backend.expose_db();
						let storage = partials.backend.expose_storage();
						cmd.run(config, partials.client.clone(), db, storage)
					}
				}),
				BenchmarkCmd::Overhead(_) => Err("Unsupported benchmarking command".into()),
				BenchmarkCmd::Machine(cmd) => {
					runner.sync_run(|config| cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone()))
				}
			}
		}
		#[cfg(feature = "try-runtime")]
		Some(Subcommand::TryRuntime(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			if chain_spec.is_snow() {
				runner.async_run(|config| {
					let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
					let task_manager =
						sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
							.map_err(|e| {
								sc_cli::Error::Service(sc_service::Error::Prometheus(e))
							})?;
					Ok((
						cmd.run::<snow_runtime::Block, snow::Executor>(config),
						task_manager,
					))
				})
			} else if chain_spec.is_arctic() {
				runner.async_run(|config| {
					let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
					let task_manager =
						sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
							.map_err(|e| {
								sc_cli::Error::Service(sc_service::Error::Prometheus(e))
							})?;
					Ok((
						cmd.run::<arctic_runtime::Block, arctic::Executor>(config),
						task_manager,
					))
				})
			} else {
				runner.async_run(|config| {
					let registry = config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
					let task_manager =
						sc_service::TaskManager::new(config.tokio_handle.clone(), registry)
							.map_err(|e| {
								sc_cli::Error::Service(sc_service::Error::Prometheus(e))
							})?;
					Ok((cmd.run::<Block, arctic::Executor>(config), task_manager))
				})
			}
		}
		None => {
			let runner = cli.create_runner(&cli.run.normalize())?;
			let chain_spec = &runner.config().chain_spec;

			set_default_ss58_version(chain_spec);

			let collator_options = cli.run.collator_options();

			runner.run_node_until_exit(|config| async move {
				if config.chain_spec.is_dev() {
					info!("Starting Frost Node");
					return solo::start_frost_node(config).map_err(Into::into);
				}
				let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
					.map(|e| e.para_id)
					.ok_or_else(|| "Could not find parachain ID in chain-spec.".to_string())?;

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()]
						.iter()
						.chain(cli.relaychain_args.iter()),
				);

				info!("Relaychain Args: {}", cli.relaychain_args.join(" "));
				let id = ParaId::from(para_id);

				let state_version = Cli::native_runtime_version(&config.chain_spec).state_version();
				let block: Block = generate_genesis_block(&config.chain_spec, state_version)
					.map_err(|e| format!("{:?}", e))?;
				let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header().encode()));

				let polkadot_config = SubstrateCli::create_configuration(
					&polkadot_cli,
					&polkadot_cli,
					config.tokio_handle.clone(),
				)
				.map_err(|err| format!("Relay chain argument error: {}", err))?;

				info!("Parachain id: {:?}", id);
				info!("Parachain genesis state: {}", genesis_state);
				info!(
					"Is collating: {}",
					if config.role.is_authority() {
						"yes"
					} else {
						"no"
					}
				);

				if config.chain_spec.is_snow() {
					start_snow_node(config, polkadot_config, collator_options, id)
						.await
						.map(|r| r.0)
						.map_err(Into::into)
				} else {
					start_arctic_node(config, polkadot_config, collator_options, id)
						.await
						.map(|r| r.0)
						.map_err(Into::into)
				}
			})
		}
	}
}

fn set_default_ss58_version(spec: &Box<dyn sc_service::ChainSpec>) {
	let ss58_version = if spec.is_arctic() || spec.is_dev() {
		// Ss58AddressFormatRegistry::ArcticAccount
		sp_core::crypto::Ss58AddressFormat::custom(2208)
	} else if spec.is_snow() {
		// Ss58AddressFormatRegistry::SnowAccount
		sp_core::crypto::Ss58AddressFormat::custom(2207)
	} else {
		// Ss58AddressFormatRegistry::PolkadotAccount
		sp_core::crypto::Ss58AddressFormat::custom(42)
	}
	.into();

	sp_core::crypto::set_default_ss58_version(ss58_version);
}

impl DefaultConfigurationValues for RelayChainCli {
	fn p2p_listen_port() -> u16 {
		30334
	}

	fn rpc_ws_listen_port() -> u16 {
		9945
	}

	fn rpc_http_listen_port() -> u16 {
		9934
	}

	fn prometheus_listen_port() -> u16 {
		9616
	}
}

impl CliConfiguration<Self> for RelayChainCli {
	fn shared_params(&self) -> &SharedParams {
		self.base.base.shared_params()
	}

	fn import_params(&self) -> Option<&ImportParams> {
		self.base.base.import_params()
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		self.base.base.keystore_params()
	}

	fn network_params(&self) -> Option<&NetworkParams> {
		self.base.base.network_params()
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		Ok(self
			.shared_params()
			.base_path()
			.or_else(|| self.base_path.clone().map(Into::into)))
	}

	fn role(&self, is_frost: bool) -> Result<sc_service::Role> {
		self.base.base.role(is_frost)
	}

	fn transaction_pool(&self) -> Result<sc_service::config::TransactionPoolOptions> {
		self.base.base.transaction_pool()
	}

	fn state_cache_child_ratio(&self) -> Result<Option<usize>> {
		self.base.base.state_cache_child_ratio()
	}

	fn chain_id(&self, is_frost: bool) -> Result<String> {
		let chain_id = self.base.base.chain_id(is_frost)?;

		Ok(if chain_id.is_empty() {
			self.chain_id.clone().unwrap_or_default()
		} else {
			chain_id
		})
	}

	fn rpc_http(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_http(default_listen_port)
	}

	fn rpc_ipc(&self) -> Result<Option<String>> {
		self.base.base.rpc_ipc()
	}

	fn rpc_ws(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_ws(default_listen_port)
	}

	fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
		self.base.base.rpc_methods()
	}

	fn rpc_ws_max_connections(&self) -> Result<Option<usize>> {
		self.base.base.rpc_ws_max_connections()
	}

	fn rpc_cors(&self, is_frost: bool) -> Result<Option<Vec<String>>> {
		self.base.base.rpc_cors(is_frost)
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<PrometheusConfig>> {
		self.base
			.base
			.prometheus_config(default_listen_port, chain_spec)
	}

	fn telemetry_endpoints(
		&self,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
		self.base.base.telemetry_endpoints(chain_spec)
	}

	fn default_heap_pages(&self) -> Result<Option<u64>> {
		self.base.base.default_heap_pages()
	}

	fn force_authoring(&self) -> Result<bool> {
		self.base.base.force_authoring()
	}

	fn disable_grandpa(&self) -> Result<bool> {
		self.base.base.disable_grandpa()
	}

	fn max_runtime_instances(&self) -> Result<Option<usize>> {
		self.base.base.max_runtime_instances()
	}

	fn announce_block(&self) -> Result<bool> {
		self.base.base.announce_block()
	}

	fn init<F>(
		&self,
		_support_url: &String,
		_impl_version: &String,
		_logger_hook: F,
		_config: &sc_service::Configuration,
	) -> Result<()>
	where
		F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
	{
		unreachable!("PolkadotCli is never initialized; qed");
	}
}
