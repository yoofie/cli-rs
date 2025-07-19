### JUSTFILE - PROJECT SPECIFIC COMMANDS ##############################################
# Written by: @y00fie (http://yoofie.net // https://github.com/yoofie)


# Justfile - https://github.com/casey/just
######################################################################################
### SHELL ############################################################################
######################################################################################
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
#set windows-shell := ["cmd.exe", "/c"]

set export
set ignore-comments := true

PYTHON_DEFAULT := if os() == "windows" { "python" } else { "python3" }
PYTHON := env_var_or_default("PYTHON", PYTHON_DEFAULT)


######################################################################################
### SETTINGS #########################################################################
######################################################################################
APP_NAME := "{{project-name}}"

### COMMANDS #########################################################################

# Build the project
@build: 
	cargo build

# Build the project
@buildr: 
	cargo build --release

# Clean the project
@clean: 
	cargo clean

# Run the project
@run $ARGS: 
	cargo run {{ARGS}}

# Run all unit tests
@test:
	cargo test

# Help command
@help:
	. .\target\debug\{{APP_NAME}} --help

# Generate Documentation
@docs:
	cargo doc --open

@native_libs:
	cargo rustc -q -- --print=native-static-libs

### EXTRAS ###########################################################################

# Ignore chmod file permission changes
@ignore_permssions:
	git config --global core.fileMode false

### EXPORT ###########################################################################

# Export the project
@export:
	. .\support\commands.ps1; exportAll {{APP_NAME}}

# Export the project source files only
@export-src:
	. .\support\commands.ps1; exportCodeOnly {{APP_NAME}}

### CODE UTILS #######################################################################

# Cargo Fix
@fix:
	cargo fix

# Run spellcheck # https://github.com/crate-ci/typos
@spellcheck:
	typos

# Fix spelling
@fix_spellcheck:
	typos -w

# Calculate stats about your code
@stats:
	. .\support\projcmd.ps1; stats
	tokei .\cli\src .\toolbox\src .\c_api\src --files > target\code_stats.txt
	tokei .\cli\src .\toolbox\src .\c_api\src --files

### META #############################################################################

# Vendor the dependencies into the 'vendor' folder
@vendor:
	cargo vendor  --no-delete --versioned-dirs --frozen -v vendor

@default:
	just --list --unsorted

# Install some cargo utilities
install_utils:
	cargo install tokei
	cargo install typos-cli
	cargo install just

### TEST #########################################################

@compare_results:
	. .\support\commands.ps1; beyond_compare_files {{justfile_directory()}} 'test/app/GeminiDVC.hex' 'test/app/GeminiDVC_filled.hex'
	. .\support\commands.ps1; beyond_compare_files {{justfile_directory()}} 'test/app/GeminiDVC.hex' 'test/app/GeminiDVC_filled.hex'

### ALIASES #########################################################################

# Alternative names for the commands above
alias b := build
alias r := buildr
alias release := buildr
alias doc := docs
alias dox := docs
