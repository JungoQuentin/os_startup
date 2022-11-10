mod inputs;
mod installation;
mod types;
mod utils;
use inputs::{get_distro, get_installation_type, get_sudo};
use installation::{custom_installation, default_installation};
use types::{Distro, Infos, InstallationType};
use utils::print_welcome;

fn main() {
    let distro = get_distro().expect("This disto is not supported");
    print_welcome();
    get_sudo();
    //todo!("differenciate when sudo (apt-get) and when not (git clone)");
    match get_installation_type(&distro) {
        InstallationType::Default => default_installation(distro),
        InstallationType::Custom => custom_installation(distro),
    }
}
