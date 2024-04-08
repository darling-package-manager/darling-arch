use darling_api as darling;

pub static PACKAGE_MANAGER: Pacman = Pacman;

pub struct Pacman;

impl darling::PackageManager for Pacman {
    fn name(&self) -> String {
        "arch".to_owned()
    }

    fn install(
        &self,
        _context: &darling::Context,
        package: &darling::InstallationEntry,
    ) -> anyhow::Result<Option<String>> {
        std::process::Command::new("pacman")
            .arg("-S")
            .arg(&package.name)
            .spawn()?;
        Ok(None)
    }

    fn uninstall(
        &self,
        _context: &darling::Context,
        package: &darling::InstallationEntry,
    ) -> anyhow::Result<()> {
        std::process::Command::new("pacman")
            .arg("-Rcns")
            .arg(&package.name)
            .spawn()?;
        Ok(())
    }

    fn get_all_explicit(&self, _context: &darling::Context) -> anyhow::Result<Vec<String>> {
        Ok(String::from_utf8(
            std::process::Command::new("pacman")
                .arg("-Qt")
                .output()?
                .stdout,
        )?
        .lines()
        .map(|line| line.to_owned())
        .collect())
    }
}
