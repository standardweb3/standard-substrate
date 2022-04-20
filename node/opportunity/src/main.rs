mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod rpc;
mod command_helper;

fn main() -> sc_cli::Result<()> {
	command::run()
}
