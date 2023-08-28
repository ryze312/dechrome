use std::{path::Path, process::Command};
use crate::{PrepareInstaller, Installer, SystemInfo, DechromeError};
use crate::utils;

const WIN_ARCH: &str = if cfg!(target_arch = "x86_64") { "win64" } else { "win32" }; 

pub struct PreparedFirefox;
impl Installer for PreparedFirefox {
    fn install(self, info: &SystemInfo, installer_path: &Path) -> Result<(), DechromeError> {
        let mut command = Command::new(installer_path);
        command.arg("/S");

        if !info.is_elevated {
            command.arg("/InstallDirectoryPath".to_owned() + info.local_appdata.to_str().unwrap());
        }

        command.spawn()?.wait()?;
        Ok(())
    }
}

pub struct Firefox;
impl PrepareInstaller<PreparedFirefox> for Firefox {
    fn prepare(info: &SystemInfo, installer_path: &Path) -> Result<PreparedFirefox, DechromeError> {
        let urls = &info.preferred_languages.iter()
            .map(|lang| format!("https://download.mozilla.org/?product=firefox-latest-ssl&os={WIN_ARCH}&lang={lang}"));

        let res = utils::try_fetch_multiple(urls.clone(), installer_path);

        match res {
            Err(DechromeError::FileNotFound) => {
                utils::fetch_file(&format!("https://download.mozilla.org/?product=firefox-latest-ssl&os={WIN_ARCH}&lang=en-US"), installer_path)?;
                Ok(PreparedFirefox)
            },
            Ok(_) => Ok(PreparedFirefox),
            Err(v) => Err(v)
        }

    }
}
 