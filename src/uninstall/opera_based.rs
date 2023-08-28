use std::collections::HashSet;

use crate::utils;
use crate::{DechromeResult, SystemInfo, UninstallInfo, Uninstaller};

const BROWSERS: [&str; 2] = ["Opera", "Opera GX"];

#[derive(Hash, PartialEq, Eq)]
pub struct OperaUninstaller {
    invoke_info: UninstallInfo,
}

impl OperaUninstaller {
    pub fn new(invoke_info: UninstallInfo) -> Self {
        Self { invoke_info }
    }
}

impl Uninstaller for OperaUninstaller {
    fn uninstall(self, _: &SystemInfo) -> DechromeResult<()> {
        let mut uninstaller = self.invoke_info.invoke()?;
        uninstaller.wait()?; // Wait for the installer to close, can't allow multiple of them to run

        Ok(())
    }
}

pub fn get_opera_based(info: &SystemInfo) -> HashSet<OperaUninstaller> {
    let uninstall_info = BROWSERS
        .iter()
        .flat_map(|browser| utils::find_browser_uninstall_info_starting_with(info, browser));

    let uninstallers = uninstall_info.map(|mut info| {
        let version = &info.display_version;

        // It's capitalised in the registry, but the actual filename isn't
        // Blame Opera
        let uninstaller_dir = info.invoke_command.trim_end_matches("Launcher.exe");
        let new_command = format!("{uninstaller_dir}/{version}/installer.exe");

        info.invoke_command = new_command;
        info.add_arg("--runimmediately"); // Unattended execution

        OperaUninstaller::new(info)
    });

    uninstallers.collect()
}
