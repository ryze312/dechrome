use std::env;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Child;
use std::process::Command;

use winreg::enums::HKEY_CURRENT_USER;
use winreg::enums::HKEY_LOCAL_MACHINE;
use winreg::RegKey;

use is_elevated::is_elevated;

use crate::utils;
use crate::DechromeError;
use crate::DechromeResult;

const USER_PROFILE_REG_PATH: &str = r"Control Panel\International\User Profile";

pub trait PrepareInstaller<T: Installer> {
    fn prepare(info: &SystemInfo, installer_path: &Path) -> DechromeResult<T>;
}

pub trait Installer {
    fn install(self, info: &SystemInfo, installer_path: &Path) -> DechromeResult<()>;
}

pub trait Uninstaller {
    fn uninstall(self, info: &SystemInfo) -> DechromeResult<()>;
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct UninstallInfo {
    pub display_version: String,
    pub invoke_command: String,
    pub invoke_args: Vec<String>,
}

impl UninstallInfo {
    pub fn new(display_version: String, invoke_command: String, invoke_args: Vec<String>) -> Self {
        Self {
            display_version,
            invoke_command,
            invoke_args,
        }
    }

    pub fn add_arg(&mut self, arg: &str) {
        self.invoke_args.push(arg.to_owned())
    }

    pub fn invoke(self) -> io::Result<Child> {
        Command::new(self.invoke_command)
            .args(self.invoke_args)
            .spawn()
    }
}

impl TryFrom<RegKey> for UninstallInfo {
    type Error = DechromeError;

    fn try_from(regkey: RegKey) -> Result<Self, Self::Error> {
        let display_version = regkey.get_value("DisplayVersion")?;
        let uninstall_string: String = regkey.get_value("UninstallString")?;
        let (invoke_command, invoke_args) = utils::parse_shell(&uninstall_string)?;

        Ok(Self {
            display_version,
            invoke_command,
            invoke_args,
        })
    }
}

#[derive(Debug)]
pub struct SystemInfo {
    pub is_elevated: bool,
    pub temp: PathBuf,
    pub appdata: PathBuf,
    pub local_appdata: PathBuf,
    pub all_users_data: PathBuf,
    pub public: PathBuf,
    pub program_files_x86: PathBuf,
    pub preferred_languages: Box<[String]>,
    pub uninstall_regkeys: Box<[RegKey]>,
}

impl SystemInfo {
    pub fn build() -> io::Result<Self> {
        let is_elevated = is_elevated();
        let temp = env::temp_dir();
        let appdata = get_env_path("APPDATA");
        let local_appdata = get_env_path("LOCALAPPDATA");
        let program_files_x86 = get_env_path("PROGRAMFILES(X86)");
        let all_users_data = get_env_path("ALLUSERSPROFILE");
        let public = get_env_path("PUBLIC");
        let preferred_languages = get_preferred_languages()?;
        let uninstall_regkeys = get_uninstall_regkeys();

        Ok(Self {
            is_elevated,
            temp,
            appdata,
            local_appdata,
            all_users_data,
            public,
            program_files_x86,
            preferred_languages,
            uninstall_regkeys,
        })
    }
}

fn get_preferred_languages() -> io::Result<Box<[String]>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let desktop_key = hkcu.open_subkey(USER_PROFILE_REG_PATH)?;
    let langs: Vec<String> = desktop_key.get_value("Languages")?;

    Ok(langs.into())
}

fn get_env_path(var: &str) -> PathBuf {
    env::var(var)
        .expect("Variable doesn't exist")
        .into()
}

fn get_uninstall_regkeys() -> Box<[RegKey]> {
    let base_keys = [
        RegKey::predef(HKEY_CURRENT_USER),
        RegKey::predef(HKEY_LOCAL_MACHINE),
    ];

    let bit_prefixes = [r"", r"WOW6432Node"];

    let combinations = base_keys
        .iter()
        .flat_map(|base_key| bit_prefixes.iter().map(move |prefix| (base_key, prefix)));

    let regkeys = combinations.filter_map(|(base_key, prefix)| {
        let path = format!(r"SOFTWARE\{prefix}\Microsoft\Windows\CurrentVersion\Uninstall");
        base_key.open_subkey(path).ok()
    });

    regkeys.collect()
}
