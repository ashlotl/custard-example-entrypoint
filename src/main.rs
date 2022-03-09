use std::{error::Error, sync::Arc};

use custard_use::{
	custard_instance::{CustardInstance, CustardInstanceSettings},
	dylib_management::safe_library::safe_library::{DebugMode, LibraryRecompile},
	utils::files::get_maybe_const_string,
};

fn main() -> Result<(), Arc<dyn Error>> {
	//tell custard_use to load all compositions and implicitly smush them together in from_string
	let root_composition_string = get_maybe_const_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/composition.ron"), include_str!("composition.ron")).0;

	let instance = CustardInstance::new(CustardInstanceSettings { root_composition_string, debug_mode: DebugMode::Debug, recompile: LibraryRecompile::Recompile });

	instance.run();

	Ok(())
}
