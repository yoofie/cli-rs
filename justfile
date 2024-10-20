### JUSTFILE - PROJECT SPECIFIC COMMANDS ##############################
# Written by: @y00fie (http://yoofie.net // https://github.com/yoofie)

# Justfile - https://github.com/casey/just
### SHELL #############################################################
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
#set windows-shell := ["cmd.exe", "/c"]

### SETTINGS #########################################################
APP_NAME := "{{project-name}}"
set ignore-comments := true
### COMMANDS #########################################################

# Build the project (debug)
build: 
	@cargo build

# Build the project (release)
buildr: 
	@cargo build --release

# Clean the project
clean: 
	@cargo clean

# Run the project
run: 
	@cargo run

# Run all unit tests
@test:
	cargo test

# Cargo Fix
@fix:
	cargo fix

# Run spellcheck # https://github.com/crate-ci/typos
@spellcheck:
	typos

# Fix spelling
@fix_spellcheck:
	typos -w

# Help command
@help:
	@. .\target\debug\{{APP_NAME}} --help

### EXPORT #########################################################

# Export the project
export:
	@. .\support\commands.ps1; exportAll {{APP_NAME}}

# Export the project source files only
export-src:
	@. .\support\commands.ps1; exportCodeOnly variable::get("APP_NAME")

### META #########################################################

# Calculate stats about your code
stats:
	@Write-host "Code stats for ./src" -f Blue
	@. .\support\commands.ps1; stats
	@tokei .\src --files > target\code_stats.txt
	@tokei .\src --files
	@Write-host "Code stats data generated: 'target\code_stats.txt'" -f Green

# Vendor the dependencies into the 'vendor' folder
vendor:
	@cargo vendor  --no-delete --versioned-dirs --frozen -v vendor

# Generate Documentation
docs:
	@cargo doc --open

default:
	@just --list --unsorted

# Install some cargo utilities
install_utils:
	cargo install tokei
	cargo install typos-cli

### TEST #########################################################

@compare_results:
	@. .\support\commands.ps1; beyond_compare_files {{justfile_directory()}} 'test/app/GeminiDVC.hex' 'test/app/GeminiDVC_filled.hex'
	@. .\support\commands.ps1; beyond_compare_files {{justfile_directory()}} 'test/app/GeminiDVC.hex' 'test/app/GeminiDVC_filled.hex'
