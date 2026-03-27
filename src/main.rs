use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{Select, Text};
use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{fs, thread};
use walkdir::WalkDir;

use crate::utils::dtos::AbiRoot;
use crate::utils::util::print_banner;
use crate::utils::{generate_typescript_content, write_typescript_file};

mod utils;

enum Framework {
    Foundry,
    Hardhat,
}

fn main() -> Result<()> {
    // Print the banner
    print_banner();

    // 1. Detect Framework
    let framework = detect_framework()?;
    // Set folder name and root directory based on detected framework
    let (folder_name, root_dir) = match framework {
        Framework::Foundry => ("Foundry", "out"),
        Framework::Hardhat => ("Hardhat", "artifacts/contracts"),
    };

    println!(
        "{} Detected {} project structure.",
        "✔".green(),
        folder_name
    );

    // Initialize a progress spinner while we search for ABI files
    let pb = ProgressBar::new_spinner();
    // Custom spinner style with more visually appealing ticks
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")?,
    );

    pb.set_message("Scanning for ABI files...");
    pb.enable_steady_tick(Duration::from_millis(80));

    // 2. Find ABI JSON files
    // We filter for .json and ignore common metadata or standard library files
    let abi_files = find_abi_files(Path::new(root_dir))?;

    // Artificial delay for smooth UX.
    thread::sleep(Duration::from_millis(500));
    pb.finish_and_clear();

    if abi_files.is_empty() {
        println!("{} No ABI JSON files found in {}.", "✘".red(), root_dir);
        return Ok(());
    }

    // Extract file names for user selection
    let file_choices: Vec<String> = abi_files.iter().map(|p| p.display().to_string()).collect();

    // 3. Ask user to select a file
    let selection = Select::new(
        &format!("{}", "Which contract do you want to typify?".bold()),
        file_choices,
    )
    .prompt()?;

    // 4. Find the full path of the selected file
    let selected_path = PathBuf::from(&selection);

    // Suggest default output file name by replacing .json with .ts (example default: MyContractAbiTypes.ts)
    let default_output_str = &selected_path
        .file_stem()
        .unwrap_or(std::ffi::OsStr::new("Output"))
        .to_string_lossy()
        .into_owned();

    // Append "AbiTypes.ts" to the default output name
    let default_output = format!("{}AbiTypes.ts", default_output_str);

    // 5. Ask for output file name
    let output_name = Text::new("Enter the output TypeScript file name:")
        .with_default(&default_output)
        .prompt()?;

    // Initialize a progress spinner for the generation process
    let gen_pb = ProgressBar::new_spinner();
    gen_pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")?,
    );
    gen_pb.set_message(format!("Processing {} -> {}...", selection, output_name));
    gen_pb.enable_steady_tick(Duration::from_millis(80));

    // 6. Read the selected ABI JSON file and parse it
    let file_content = fs::read_to_string(&selected_path)?;
    let abi_root: AbiRoot = serde_json::from_str(&file_content)?;

    // Use the selected file stem as the contract name (e.g. Token.json -> Token)
    let contract_name = selected_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Contract");

    // 7. Generate TypeScript content from the ABI
    let ts_code = generate_typescript_content(contract_name, abi_root);

    // 8. Write the generated TypeScript content to a file
    write_typescript_file(&output_name, ts_code)?;

    // Artificial delay for smooth UX.
    thread::sleep(Duration::from_millis(500));
    gen_pb.finish_and_clear();

    // 9. Print success message with the output file name
    println!(
        "\n{} {} {} {} {}",
        "✨".yellow(),
        "Success!".green().bold(),
        "Generated".white(),
        output_name.cyan().underline(),
        "from the forge.".white().italic()
    );

    Ok(())
}

/// Detects whether the current directory is a Foundry or Hardhat project by checking for specific files.
fn detect_framework() -> Result<Framework> {
    // Check for both Foundry and Hardhat indicators first to handle cases where both might be present
    if Path::new("foundry.toml").exists() && Path::new("hardhat.config.ts").exists()
        || Path::new("hardhat.config.js").exists()
    {
        // choices
        let framework_choice: Vec<String> = vec!["Foundry".to_string(), "Hardhat".to_string()];

        // ask user to choose which framework to use for ABI extraction
        let framework_choice = Select::new("Both Foundry and Hardhat project structures detected. Which one do you want to use for ABI extraction?", framework_choice)
            .prompt()?;

        // Return the user's choice as the detected framework
        match framework_choice.as_str() {
            "Foundry" => Ok(Framework::Foundry),
            "Hardhat" => Ok(Framework::Hardhat),
            _ => Err(anyhow::anyhow!("Invalid framework choice")),
        }
    }
    // Check for Foundry indicators first
    else if Path::new("foundry.toml").exists() && Path::new("out").exists() {
        Ok(Framework::Foundry)
    }
    // Check for Hardhat indicators next
    else if Path::new("hardhat.config.ts").exists()
        || Path::new("hardhat.config.js").exists() && Path::new("artifacts").exists()
    {
        Ok(Framework::Hardhat)
    }
    // If neither is detected, return an error
    else {
        Err(anyhow::anyhow!(
            "Could not detect Foundry or Hardhat project. Please run in the project root."
        ))
    }
}

/// Recursively searches the specified directory for ABI JSON files, filtering out common metadata and standard library files.
fn find_abi_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
            let file_name = path
                .file_name()
                .unwrap_or(std::ffi::OsStr::new(""))
                .to_string_lossy();

            // Check if the file is part of forge-std (common standard library for Foundry)
            let is_forge_std = path.components().any(|c| c.as_os_str() == "forge-std");

            // Filter out internal build files or standard libraries
            if !file_name.ends_with(".metadata.json") && !is_forge_std {
                files.push(path.to_path_buf());
            }
        }
    }
    Ok(files)
}
