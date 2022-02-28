use std::{error::Error, sync::Arc};

use custard_use::{
	composition::{loaded::loaded_composition::LoadedComposition, unloaded::unloaded_composition::UnloadedComposition},
	dylib_management::safe_library::safe_library::{DebugMode, LibraryRecompile},
	utils::files::get_maybe_const_string,
};

fn main() -> Result<(), Arc<dyn Error>> {
	//tell custard_use to load all compositions and implicitly smush them together in from_string
	let root_composition_string = get_maybe_const_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/composition.ron"), include_str!("composition.ron")).0;
	let root_composition_unloaded = UnloadedComposition::from_string(root_composition_string, LibraryRecompile::Recompile, DebugMode::Debug)?;

	println!("Full UnloadedComposition: {:#?}", root_composition_unloaded);

	//tell lib to turn an unloaded composition (see above) into a loaded composition
	let root_composition = LoadedComposition::check(root_composition_unloaded, LibraryRecompile::Recompile, DebugMode::Debug)?;

	println!("Full LoadedComposition: {:#?}", root_composition);

	//run the loaded composition and wait
	let wait = root_composition.run();

	wait.wait();

	Ok(())
}
