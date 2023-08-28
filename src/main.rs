#![windows_subsystem = "windows"]

use dechrome::{install, uninstall};
use dechrome::{SystemInfo, PrepareInstaller, Installer, Uninstaller};


fn main() {
    let info = SystemInfo::build().unwrap();

    let mut firefox_path = info.temp.clone();
    firefox_path.push("firefox_installer.exe");

    println!("Starting...");
    match install::Firefox::prepare(&info, &firefox_path) {
        Ok(firefox) => {
            println!("Installer downloaded. Removing browsers");
            remove_browsers(&info);

            println!("Installing Firefox");
            let res = firefox.install(&info, &firefox_path);
            println!("Result: {res:?}");
        },
        Err(e) => eprintln!("Couldn't download Firefox: {e:?}")
    }
}

fn remove_browsers(info: &SystemInfo) {
    println!("Chrome based:");
    for browser in uninstall::get_chrome_based(info) {
        let res = browser.uninstall(info);
        println!("Result: {res:?}");
    }

    println!("Opera based:");
    for opera in uninstall::get_opera_based(info) {
        let res = opera.uninstall(info);
        println!("Result: {res:?}");
    }

    if let Some(edge) = uninstall::get_edge_based(info) {
        let res = edge.uninstall(info);
        println!("Edge: ");
        println!("Result: {res:?}");
    }
}
