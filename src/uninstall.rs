pub mod chrome_based;
pub mod opera_based;
pub mod edge_based;

pub use chrome_based::get_chrome_based;
pub use opera_based::get_opera_based;
pub use edge_based::get_edge_based;

use crate::{DechromeResult, SystemInfo, UninstallInfo, Uninstaller};

#[derive(Debug)]
pub struct SimpleUninstaller {
    uninstall_info: UninstallInfo,
}

impl SimpleUninstaller {
    pub fn new(uninstall_info: UninstallInfo) -> Self {
        Self { uninstall_info }
    }
}

impl Uninstaller for SimpleUninstaller {
    fn uninstall(self, _info: &SystemInfo) -> DechromeResult<()> {
        self.uninstall_info.invoke()?;
        Ok(())
    }
}
