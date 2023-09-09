mod cli;
mod config;
mod file_manager;
mod github_actions;
mod licenses;
mod package_version;
mod project_generator;
mod project_info;
mod python_files;
mod rust_files;

use std::process::exit;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use cli::ApplicationOrLib;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

use crate::cli::{Args, BooleanChoice, Command, Param};
use crate::config::Config;
use crate::project_generator::generate_project;
use crate::project_info::{get_project_info, ProjectInfo};

fn create(project_info: &ProjectInfo) -> Result<()> {
    generate_project(project_info);
    std::process::Command::new("git")
        .args(["init", &project_info.project_slug])
        .output()
        .expect("Failed to initialize git");

    Ok(())
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Create {
            skip_download_latest_packages,
            default,
        } => {
            let mut project_info = get_project_info(default);
            project_info.download_latest_packages = !skip_download_latest_packages;

            let create_result: Result<()>;
            if let Ok(progress_style) = ProgressStyle::with_template("{spinner:.green} {msg}") {
                let pb = ProgressBar::new_spinner();
                pb.enable_steady_tick(Duration::from_millis(80));
                pb.set_style(progress_style);
                pb.set_message("Generataing Project...");
                create_result = create(&project_info);
                pb.finish_and_clear();
            } else {
                create_result = create(&project_info);
            }

            match create_result {
                Ok(_) => {
                    let success_message = format!(
                        "\nProject created in the {} directory",
                        project_info.project_slug
                    );
                    println!("{}", success_message.green());
                }
                Err(_) => {
                    let error_message =
                        "\nAn Error occurred creating the project. Please try again.".to_string();
                    println!("{}", error_message.red());
                }
            };
        }
        Command::Config(config) => match config.param {
            Param::Creator { value } => Config::save_creator(value),
            Param::CreatorEmail { value } => Config::save_creator_email(value),
            Param::License { value } => Config::save_license(value),
            Param::PythonVersion { value } => Config::save_python_version(value),
            Param::MinPythonVersion { value } => Config::save_min_python_version(value),
            Param::UsePyo3 { value } => match value {
                BooleanChoice::True => Config::save_use_pyo3(true),
                BooleanChoice::False => Config::save_use_pyo3(false),
            },
            Param::ApplicationOrLibrary { value } => match value {
                ApplicationOrLib::Application => Config::save_is_application(true),
                ApplicationOrLib::Lib => Config::save_is_application(false),
            },
            Param::GithubActionPythonTestVersions { value } => {
                Config::save_github_actions_python_test_versions(value)
            }
            Param::MaxLineLength { value } => Config::save_max_line_length(value),
            Param::UseDependabot { value } => match value {
                BooleanChoice::True => Config::save_use_dependabot(true),
                BooleanChoice::False => Config::save_use_dependabot(false),
            },
            Param::UseContinuousDeployment { value } => match value {
                BooleanChoice::True => Config::save_use_continuous_deployment(true),
                BooleanChoice::False => Config::save_use_continuous_deployment(false),
            },
            Param::UseReleaseDrafter { value } => match value {
                BooleanChoice::True => Config::save_use_release_drafter(true),
                BooleanChoice::False => Config::save_use_release_drafter(false),
            },
            Param::UseMultiOsCi { value } => match value {
                BooleanChoice::True => Config::save_use_multi_os_ci(true),
                BooleanChoice::False => Config::save_use_multi_os_ci(false),
            },

            Param::DownloadLatestPackages { value } => match value {
                BooleanChoice::True => Config::save_download_latest_packages(true),
                BooleanChoice::False => Config::save_download_latest_packages(false),
            },
            Param::Reset => {
                if Config::reset().is_err() {
                    let message = "Error resetting config.";
                    println!("{}", message.red());
                    exit(1);
                }
            }
            Param::Show => Config::show(),
        },
    }
}
