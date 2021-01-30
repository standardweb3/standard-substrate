
				use substrate_wasm_builder::build_project_with_default_rustflags;

				fn main() {
					build_project_with_default_rustflags(
						"/home/hskang9/standard-substrate/target/release/build/node-template-runtime-f246aa2a246b0760/out/wasm_binary.rs",
						"/home/hskang9/standard-substrate/runtime/Cargo.toml",
						"-Clink-arg=--export=__heap_base -C link-arg=--import-memory ",
					)
				}
			