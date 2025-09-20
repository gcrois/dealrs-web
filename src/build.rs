use vergen::{BuildBuilder, CargoBuilder, Emitter, RustcBuilder, SysinfoBuilder};
use vergen_git2::Git2Builder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let build = BuildBuilder::all_build()?;
    let cargo = CargoBuilder::all_cargo()?;
    let rustc = RustcBuilder::all_rustc()?;
    let si = SysinfoBuilder::all_sysinfo()?;
    let git2 = Git2Builder::all_git()?;

    Emitter::new()
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .add_instructions(&si)?
        .add_instructions(&git2)?
        .emit()?;

    Ok(())
}
