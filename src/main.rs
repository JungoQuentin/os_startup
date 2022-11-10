mod inputs;
mod installation;
mod types;
mod utils;
use inputs::{get_distro, get_installation_type};
use installation::{custom_installation, default_installation};
use types::{Distro, InstallationType};
use utils::print_welcome;

fn main() {
    print_welcome();
    let distro = get_distro().expect("This disto is not supported");
    match get_installation_type(&distro) {
        InstallationType::Default => default_installation(distro),
        InstallationType::Custom => custom_installation(distro),
    }
}
