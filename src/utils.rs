use std::{fs::File, io::Write, path::Path};
use ureq;

use crate::{DechromeError, DechromeResult, SystemInfo, UninstallInfo};

pub fn fetch_file(url: &str, path: &Path) -> DechromeResult<()> {
    if path.exists() {
        return Ok(());
    }

    let response = ureq::get(url).call()?;
    let length = response
        .header("Content-Length")
        .ok_or(DechromeError::ContentLengthError)?;
    let length = length
        .parse()
        .map_err(|_| DechromeError::ContentLengthError)?;

    let mut buffer = Vec::with_capacity(length);
    response.into_reader().read_to_end(&mut buffer)?;

    let mut file = File::create(path)?;
    file.write_all(&buffer)?;

    Ok(())
}

pub fn try_fetch_multiple<I: IntoIterator<Item = String>>(
    urls: I,
    path: &Path,
) -> DechromeResult<()> {
    for url in urls {
        let res = fetch_file(&url, path);

        if let Err(DechromeError::FileNotFound) = res {
            continue;
        }

        if res.is_ok() {
            return Ok(());
        }

        res?;
    }

    Err(DechromeError::FileNotFound)
}

pub fn parse_shell(input: &str) -> DechromeResult<(String, Vec<String>)> {
    let mut in_quotes = false;

    let input = input.trim();
    let input_split = input.split(|char| match char {
        '"' => {
            in_quotes = !in_quotes;
            false
        }
        ' ' => !in_quotes,
        _ => false,
    });

    let mut parsed = input_split.map(|split| split.replace('"', ""));
    let command = parsed.next().ok_or(DechromeError::ExecutablePathNotFound)?;
    let args = parsed.collect();

    if in_quotes {
        // Has to run after .collect because that's when everything runs
        return Err(DechromeError::MismatchedQuotes);
    }

    Ok((command, args))
}

pub fn find_browser_uninstall_info<'a>(
    info: &'a SystemInfo,
    browser: &'a str,
) -> impl Iterator<Item = UninstallInfo> + 'a {
    let regkeys = info
        .uninstall_regkeys
        .iter()
        .filter_map(move |regkey| regkey.open_subkey(browser).ok());

    regkeys.filter_map(|browser_key| {
        UninstallInfo::try_from(browser_key).ok()
    })
}

pub fn find_browser_uninstall_info_starting_with<'a>(
    info: &'a SystemInfo,
    starts_with: &'a str,
) -> impl Iterator<Item = UninstallInfo> + 'a {
    let regkeys = info.uninstall_regkeys.iter().flat_map(move |regkey| {
        regkey
            .enum_keys()
            .filter_map(|res| res.ok()) // Filter out successful ones
            .filter(move |regkey| regkey.starts_with(starts_with)) // Get only matched
            .filter_map(|regname| regkey.open_subkey(regname).ok()) // Try to open them
    });

    regkeys.filter_map(|browser_key| {
        UninstallInfo::try_from(browser_key).ok()
    })
}
