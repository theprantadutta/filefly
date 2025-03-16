use crate::filefly_args::UpgradeCommand;
use crate::logger::Logger;

use serde_json::Value;
use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self};
use std::path::PathBuf;
use std::process::Command;

// GitHub repository details
const REPO_OWNER: &str = "theprantadutta";
const REPO_NAME: &str = "filefly";

// Function to fetch the latest release from GitHub
fn fetch_latest_release() -> Result<Value, Box<dyn Error>> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        REPO_OWNER, REPO_NAME
    );
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "filefly-updater") // Required by GitHub API
        .send()?;

    let json: Value = response.json()?;

    Ok(json)
}

// Function to download a file from a given URL and save it to the specified path
fn download_file(url: &str, path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let mut response = client
        .get(url)
        .header("User-Agent", "filefly-updater")
        .send()?;

    let mut file = File::create(path)?;
    io::copy(&mut response, &mut file)?;

    Ok(())
}

// Function to add a directory to the user's PATH (Windows only)
fn add_to_path(dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let path_env = env::var("Path").or_else(|_| env::var("PATH"))?;
    let dir_str = dir.to_str().ok_or("Invalid path")?;

    if !path_env.split(";").any(|p| p == dir_str) {
        let new_path = format!("{};{}", path_env, dir_str);
        env::set_var("Path", &new_path);
        // Persist the change for the user
        Command::new("setx").args(&["Path", &new_path]).output()?;
    }
    Ok(())
}

// Function to handle the upgrade command
pub fn handle_upgrade_command(_command: UpgradeCommand) -> Result<(), Box<dyn Error>> {
    // Start the logger
    let logger = Logger::default();

    logger.info("Fetching the latest release information...");
    let release = fetch_latest_release()?;

    // Get the version and assets
    let version = release["tag_name"].as_str().unwrap_or("unknown");
    let binding = vec![];
    let assets = release["assets"].as_array().unwrap_or(&binding);

    // Determine the appropriate asset based on the OS
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

    // Ensure installation directory exists
    if !install_path.exists() {
        logger.info("Creating installation directory...");
        fs::create_dir_all(&install_path)?;
    }

    // Define the full path to the binary
    let binary_name = if cfg!(windows) {
        "filefly.exe"
    } else {
        "filefly"
    };
    let binary_path = install_path.join(binary_name);

    // Download the binary
    logger.info(&format!("Downloading Filefly {}...", version));
    download_file(download_url, &binary_path)?;

    // Make the binary executable (Unix only)
    if !cfg!(windows) {
        logger.info("Setting executable permissions...");
        let output = Command::new("chmod")
            .args(&["+x", binary_path.to_str().unwrap()])
            .output()?;

        if !output.status.success() {
            logger.error(&format!(
                "Failed to make binary executable: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
            return Ok(());
        }
    }

    // Add to PATH (Windows only)
    if cfg!(windows) {
        logger.info("Adding installation directory to system PATH...");
        add_to_path(&install_path)?;
    }

    logger.success(&format!("Filefly {} installed successfully!", version));
    Ok(())
}
