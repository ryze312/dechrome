use crate::uninstall::SimpleUninstaller;
use crate::utils;
use crate::SystemInfo;

const BROWSERS: [&str; 5] = [
    "Google Chrome",
    "Google Chrome SxS",
    "BraveSoftware Brave-Browser",
    "Vivaldi",
    "YandexBrowser",
];

// Get uninstallers based on Chrome installer, which have an option --force-uninstall for silent execution
pub fn get_chrome_based(info: &SystemInfo) -> Vec<SimpleUninstaller> {
    let uninstall_info = BROWSERS
        .iter()
        .flat_map(|browser| utils::find_browser_uninstall_info(info, browser));

    let uninstallers = uninstall_info.map(|mut info| {
        info.add_arg("--force-uninstall");
        SimpleUninstaller::new(info)
    });

    uninstallers.collect()
}
