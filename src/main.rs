use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use spin_factors::RuntimeFactors;
use spin_runtime_factors::FactorsBuilder;
use spin_trigger::cli::FactorsTriggerCommand;
use spin_trigger::{Trigger, TriggerApp};
use std::time::Duration;

#[derive(Clone, Debug, Deserialize)]
pub struct TriggerConfig {
    pub component: String,
    #[serde(default)]
    pub delay_secs: u64,
}

pub struct DelayedCommandTrigger {
    config: TriggerConfig,
}

impl<F: RuntimeFactors> Trigger<F> for DelayedCommandTrigger {
    const TYPE: &'static str = "delayed-command";
    type CliArgs = ();
    type InstanceState = ();

    fn new(_args: Self::CliArgs, app: &spin_trigger::App) -> Result<Self> {
        let configs = app
            .trigger_configs::<TriggerConfig>(<Self as Trigger<F>>::TYPE)?;
        let (_, config) = configs
            .into_iter()
            .next()
            .context("No delayed-command trigger configured in manifest")?;
        Ok(Self { config })
    }

    async fn run(self, trigger_app: TriggerApp<Self, F>) -> Result<()> {
        if self.config.delay_secs > 0 {
            println!(
                "Pausing {}s before executing '{}'...",
                self.config.delay_secs, self.config.component
            );
            tokio::time::sleep(
                Duration::from_secs(self.config.delay_secs),
            ).await;
        }

        let builder = trigger_app.prepare(&self.config.component)?;
        let (instance, mut store) = builder.instantiate(()).await?;

        // Resolve the WASI Command export interface (wasi:cli/run)
        let command = wasmtime_wasi::p2::bindings::Command::new(
            &mut store,
            &instance,
        )?;

        command
            .wasi_cli_run()
            .call_run(&mut store)
            .await?
            .map_err(|_| anyhow::anyhow!("guest call failed"))?;

        println!("Execution completed successfully.");
        Ok(())
    }
}

type Command = FactorsTriggerCommand<
    DelayedCommandTrigger,
    FactorsBuilder,
>;

#[tokio::main]
async fn main() -> Result<()> {
    let command = Command::parse();
    command.run().await
}
