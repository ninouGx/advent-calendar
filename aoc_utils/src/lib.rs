use std::panic::Location;
use std::path::Path;
use std::fs::{ self, File };
use std::io::Write;
use std::process::Command;

mod direction;
mod position;
mod grid;
mod display;

pub use direction::Direction;
pub use position::Position;
pub use grid::Grid;
pub use display::{ display_grid, display_grid_animated, clear_screen_and_move_cursor };

#[track_caller]
pub fn get_input_for_day(is_test: bool) -> String {
    let caller = Location::caller();
    let file_path = caller.file();

    let day = file_path
        .split('/')
        .last()
        .unwrap()
        .strip_prefix("day")
        .unwrap()
        .strip_suffix(".rs")
        .unwrap();

    let year = file_path.split('/').next().unwrap().strip_prefix("year_").unwrap();

    get_input(year, day, is_test)
}

fn get_input(year: &str, day: &str, is_test: bool) -> String {
    let base_path = format!("year_{}/src/data{}", year, if is_test { "/test" } else { "" });
    let filename = format!("{}/day{}{}_input.txt", base_path, day, if is_test {
        "_test"
    } else {
        ""
    });

    fs::read_to_string(&filename).unwrap_or_else(|_|
        panic!("Unable to read input file: {}", filename)
    )
}

fn create_year_project(year: &str) -> std::io::Result<()> {
    let package_name = format!("year_{}", year);

    // Run cargo new to create the project (automatically added to workspace)
    let status = Command::new("cargo")
        .args(["new", &package_name])
        .status()
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to run cargo new: {}", e)
            )
        })?;

    if !status.success() {
        return Err(
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("cargo new {} failed", package_name)
            )
        );
    }

    println!("Created new project: {}", package_name);

    // Overwrite Cargo.toml with correct dependencies
    let cargo_toml_content = format!(
        r#"[package]
name = "{package_name}"
version = "0.1.0"
edition = "2024"

[dependencies]
progress_timer = {{ git = "https://github.com/ninouGx/progress_timer" }}
aoc_utils = {{ path = "../aoc_utils" }}
"#
    );

    let cargo_toml_path = format!("./{}/Cargo.toml", package_name);
    let mut file = File::create(&cargo_toml_path)?;
    file.write_all(cargo_toml_content.as_bytes())?;
    println!("Configured {}", cargo_toml_path);

    // Delete the placeholder main.rs
    let main_rs_path = format!("./{}/src/main.rs", package_name);
    if Path::new(&main_rs_path).exists() {
        fs::remove_file(&main_rs_path)?;
    }

    Ok(())
}

pub fn create_day_files(year: &str, day: u32) -> std::io::Result<()> {
    let day = format!("{:02}", day);
    let package_name = format!("year_{}", year);
    let year_project_path = format!("./{}", package_name);

    if !Path::new(&year_project_path).exists() {
        create_year_project(year)?;
    }

    fs::create_dir_all(format!("{}/src/data/test", year_project_path))?;
    fs::create_dir_all(format!("{}/src/bin", year_project_path))?;

    let template_path = "templates/day_template.rs";
    let template = fs
        ::read_to_string(&template_path)
        .map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Template file not found at '{}': {}", template_path, e)
            )
        })?;

    // Solution file
    let rs_path = format!("{}/src/bin/day{}.rs", year_project_path, day);
    if !Path::new(&rs_path).exists() {
        let mut file = File::create(&rs_path)?;
        file.write_all(template.as_bytes())?;
        println!("Created {}", rs_path);
    }

    // Input files
    let input_path = format!("{}/src/data/day{}_input.txt", year_project_path, day);
    let test_input_path = format!("{}/src/data/test/day{}_test_input.txt", year_project_path, day);

    if !Path::new(&input_path).exists() {
        File::create(&input_path)?;
        println!("Created {}", input_path);
    }

    if !Path::new(&test_input_path).exists() {
        File::create(&test_input_path)?;
        println!("Created {}", test_input_path);
    }

    Ok(())
}
