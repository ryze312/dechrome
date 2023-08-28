use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};
use std::fs;

use crate::{Uninstaller, DechromeResult, SystemInfo};

const EDGE_PATHS: [&str; 3] = [
    "Microsoft/Edge",
    "Microsoft/EdgeCore",
    "Microsoft/EdgeUpdate",
];

const EDGE_REGKEYS: [&str; 3] = [
    r"SOFTWARE\Microsoft\Active Setup\Installed Components\{9459C573-B17A-45AE-9F64-1857B5D58CEE}",
    r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Microsoft Edge",
    r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall\Microsoft Edge Update"
];

const EDGE_UPDATE_REGKEY: &str = r"SOFTWARE\Microsoft\EdgeUpdate";


pub struct EdgeUninstaller;

#[allow(unused_must_use)] // Supress warnings of for io::Result
impl EdgeUninstaller {
    fn remove_files(info: &SystemInfo) {
        let program_files_x86 = &info.program_files_x86;

        for path in EDGE_PATHS {
            let mut full_path = program_files_x86.clone();
            full_path.push(path);

            fs::remove_dir_all(&full_path);
        }

        let mut link_path = info.public.clone();
        link_path.push("Desktop/Microsoft Edge.lnk");
        fs::remove_file(link_path);

        let mut link_path = info.all_users_data.clone();
        link_path.push("Microsoft/Windows/Start Menu/Programs/Microsoft Edge.lnk");
        fs::remove_file(link_path);
        let mut link_path = info.appdata.clone();
        link_path.push("Microsoft/Internet Explorer/Quick Launch/User Pinned/TaskBar/Microsoft Edge.lnk");
        fs::remove_file(link_path);
    }

    fn remove_regkeys() {
        let hlkm = RegKey::predef(HKEY_LOCAL_MACHINE);
        
        for path in EDGE_REGKEYS {
            hlkm.delete_subkey_all(path);
        }
    }

    fn disable_update() -> DechromeResult<()> {
        let hlkm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let (edge_update_key, _)  = hlkm.create_subkey(EDGE_UPDATE_REGKEY)?;
        edge_update_key.set_value("DoNotUpdateToEdgeWithChromium", &1u32)?; // DWORD is u32

        Ok(())
    }
}

impl Uninstaller for EdgeUninstaller {
    fn uninstall(self, info: &SystemInfo) -> DechromeResult<()> {
        Self::remove_files(info);
        Self::remove_regkeys();
        Self::disable_update()
    }
}

pub fn get_edge_based(info: &SystemInfo) -> Option<EdgeUninstaller> {
    if info.is_elevated {
        Some(EdgeUninstaller {})
    } else {
        None
    }
}
