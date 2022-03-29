use custard_use::{
	custard_instance::{CustardInstance, CustardInstanceSettings},
	dylib_management::safe_library::safe_library::{DebugMode, LibraryRecompile},
	utils::files::get_maybe_const_string,
};

use log::info;

use std::{error::Error, sync::Arc};

fn main() -> Result<(), Arc<dyn Error>> {
	env_logger::init();

	info!("Entrypoint reached.");

	info!("Fetching serialized composition.");
	//If the composition can be found at runtime, it's loaded via fs::read, otherwise it is included in the binary using `include_str!`.
	let root_composition_string = get_maybe_const_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/composition.ron"), include_str!("composition.ron")).0;
	info!("Fetched serialized composition.");

	info!("Creating instance.");
	//Make an instance with some reasonable settings for development. In reality, you might want to make this `#[cfg()]`.
	let instance = CustardInstance::new(CustardInstanceSettings { root_composition_string, debug_mode: DebugMode::Debug, recompile: LibraryRecompile::Recompile });
	info!("Created instance.");

	info!("Running instance.");
	//What it sounds like.
	instance.run();
	info!("Done, exiting entrypoint.");

	Ok(())
}
