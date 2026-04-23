use crate::filefly_args::UpgradeCommand;
use crate::logger::Logger;

use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self};
use std::path::{Path, PathBuf};
use std::process::Command;

const REPO_OWNER: &str = "theprantadutta";
const REPO_NAME: &str = "filefly";

fn fetch_latest_release() -> Result<Value, Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        REPO_OWNER, REPO_NAME
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "filefly-updater")
        .send()?;

    let json: Value = response.json()?;

    Ok(json)
}

fn download_file(url: &str, path: &Path) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let mut response = client
        .get(url)
        .header("User-Agent", "filefly-updater")
        .send()?;

    let mut file = File::create(path)?;
    io::copy(&mut response, &mut file)?;

    Ok(())
}

#[cfg(windows)]
fn add_to_path(dir: &Path) -> Result<(), Box<dyn Error>> {
    let dir_str = dir.to_str().ok_or("Invalid path")?;

    let read = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "[Environment]::GetEnvironmentVariable('Path', 'User')",
        ])
        .output()?;
    let user_path = String::from_utf8_lossy(&read.stdout).trim().to_string();

    let already_present = user_path
        .split(';')
        .any(|p| p.eq_ignore_ascii_case(dir_str));
    if already_present {
        return Ok(());
    }

    let new_path = if user_path.is_empty() {
        dir_str.to_string()
    } else {
        format!("{};{}", user_path.trim_end_matches(';'), dir_str)
    };

    let ps_cmd = format!(
        "[Environment]::SetEnvironmentVariable('Path', '{}', 'User')",
        new_path.replace('\'', "''")
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-Command", &ps_cmd])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Failed to update User PATH: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}

#[cfg(windows)]
fn stage_existing_binary(binary_path: &Path) -> io::Result<()> {
    if !binary_path.exists() {
        return Ok(());
    }

    let old_path = binary_path.with_extension("exe.old");
    if old_path.exists() {
        let _ = fs::remove_file(&old_path);
    }

    fs::rename(binary_path, &old_path)
}

pub fn handle_upgrade_command(_command: UpgradeCommand) -> Result<(), Box<dyn Error>> {
    let logger = Logger::default();

    logger.info("Fetching the latest release information...");
    let release = fetch_latest_release()?;

    let version = release["tag_name"].as_str().unwrap_or("unknown");
    let binding = vec![];
    let assets = release["assets"].as_array().unwrap_or(&binding);

    let asset = if cfg!(windows) {
        assets
            .iter()
            .find(|a| a["name"].as_str().unwrap_or("").contains("windows"))
    } else {
        assets
            .iter()
            .find(|a| a["name"].as_str().unwrap_or("").contains("unix"))
    };

    let asset = match asset {
        Some(asset) => asset,
        None => {
            logger.error("No compatible asset found for the current OS.");
            return Ok(());
        }
    };

    let download_url = asset["browser_download_url"].as_str().unwrap_or("");
    if download_url.is_empty() {
        logger.error("No download URL found for the asset.");
        return Ok(());
    }

    logger.info("Determining installation path...");
    let install_path = if cfg!(windows) {
        let local_app_data = env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(local_app_data)
            .join("Programs")
            .join("filefly")
    } else {
        PathBuf::from("/usr/local/bin")
    };

    if !install_path.exists() {
        logger.info("Creating installation directory...");
        fs::create_dir_all(&install_path)?;
    }

    let binary_name = if cfg!(windows) {
        "filefly.exe"
    } else {
        "filefly"
    };
    let binary_path = install_path.join(binary_name);

    #[cfg(windows)]
    {
        if binary_path.exists() {
            logger.info("Staging existing binary aside (file-in-use safe)...");
            if let Err(e) = stage_existing_binary(&binary_path) {
                logger.warning(&format!(
                    "Could not move existing binary out of the way: {}",
                    e
                ));
            }
        }
    }

    logger.info(&format!("Downloading Filefly {}...", version));
    download_file(download_url, &binary_path)?;

    #[cfg(not(windows))]
    {
        logger.info("Setting executable permissions...");
        let output = Command::new("chmod")
            .args(["+x", binary_path.to_str().unwrap()])
            .output()?;

        if !output.status.success() {
            logger.error(&format!(
                "Failed to make binary executable: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Ok(());
        }
    }

    #[cfg(windows)]
    {
        logger.info("Adding installation directory to user PATH...");
        if let Err(e) = add_to_path(&install_path) {
            logger.warning(&format!("Could not update user PATH: {}", e));
        }
    }

    logger.success(&format!("Filefly {} installed successfully!", version));
    Ok(())
}
