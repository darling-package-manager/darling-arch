use darling_api as darling;

pub static PACKAGE_MANAGER: Pacman = Pacman;

pub struct Pacman;

impl darling::PackageManager for Pacman {
    fn name(&self) -> String {
        "arch".to_owned()
    }

    fn install(&self, _context: &darling::Context, package: &darling::InstallationEntry) -> anyhow::Result<Option<String>> {
        std::process::Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("--noconfirm")
            .arg(&package.name)
            .stdout(std::process::Stdio::null())
            .status()?;
        Ok(None)
    }

    fn uninstall(&self, _context: &darling::Context, package: &darling::InstallationEntry) -> anyhow::Result<()> {
        std::process::Command::new("sudo").arg("pacman").arg("-Rcns").arg(&package.name).status()?;
        Ok(())
    }

    fn get_all_explicit(&self, _context: &darling::Context) -> anyhow::Result<Vec<(String, String)>> {
        Ok(String::from_utf8(std::process::Command::new("pacman").arg("-Qt").output()?.stdout)?
            .lines()
            .map(|line| {
                let splits = line.split(' ').collect::<Vec<_>>();
                (splits[0].to_owned(), splits[1].to_owned())
            })
            .collect())
    }
}
