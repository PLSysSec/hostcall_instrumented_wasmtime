//! The `wasmtime` command line tool.
//!
//! Primarily used to run WebAssembly modules.
//! See `wasmtime --help` for usage.

use anyhow::Result;
use structopt::{clap::AppSettings, clap::ErrorKind, StructOpt};
use wasmtime_cli::commands::{
    CompileCommand, ConfigCommand, RunCommand, SettingsCommand, WastCommand,
};

/// Wasmtime WebAssembly Runtime
#[derive(StructOpt)]
#[structopt(
    name = "wasmtime",
    version = env!("CARGO_PKG_VERSION"),
    global_settings = &[
        AppSettings::VersionlessSubcommands,
        AppSettings::ColoredHelp
    ],
    after_help = "If a subcommand is not provided, the `run` subcommand will be used.\n\
                  \n\
                  Usage examples:\n\
                  \n\
                  Running a WebAssembly module with a start function:\n\
                  \n  \
                  wasmtime example.wasm
                  \n\
                  Passing command line arguments to a WebAssembly module:\n\
                  \n  \
                  wasmtime example.wasm arg1 arg2 arg3\n\
                  \n\
                  Invoking a specific function (e.g. `add`) in a WebAssembly module:\n\
                  \n  \
                  wasmtime example.wasm --invoke add 1 2\n"
)]
enum WasmtimeApp {
    // !!! IMPORTANT: if subcommands are added or removed, update `parse_module` in `src/commands/run.rs`. !!!
    /// Controls Wasmtime configuration settings
    Config(ConfigCommand),
    /// Compiles a WebAssembly module.
    Compile(CompileCommand),
    /// Runs a WebAssembly module
    Run(RunCommand),
    /// Displays available Cranelift settings for a target.
    Settings(SettingsCommand),
    /// Runs a WebAssembly test script file
    Wast(WastCommand),
}

impl WasmtimeApp {
    /// Executes the command.
    pub fn execute(self) -> Result<()> {
        match self {
            Self::Config(c) => c.execute(),
            Self::Compile(c) => c.execute(),
            Self::Run(c) => c.execute(),
            Self::Settings(c) => c.execute(),
            Self::Wast(c) => c.execute(),
        }
    }
}

fn main() -> Result<()> {
    let res = WasmtimeApp::from_iter_safe(std::env::args())
        .unwrap_or_else(|e| match e.kind {
            ErrorKind::HelpDisplayed
            | ErrorKind::VersionDisplayed
            | ErrorKind::MissingArgumentOrSubcommand => e.exit(),
            _ => WasmtimeApp::Run(
                RunCommand::from_iter_safe(std::env::args()).unwrap_or_else(|_| e.exit()),
            ),
        })
        .execute();
    
    use statistical::mean;
    use wiggle::timing::results;
    use std::fs::File;
    use std::io::Write;
    use statistical::univariate::geometric_mean;
    let mut f = File::create("./wasmtime_results.txt").expect("Unable to open file");
    results.with(|r| {
            for (k,v) in r.borrow().iter(){
                if !v.is_empty(){
                    let mean = mean(v);
                    let geomean = geometric_mean(v);
                    writeln!(f, "{:?},{:?},{:?},{:?}", k, v.len(), mean, geomean);
                }
            }
        });
    return res;
}
