### JUSTFILE - PROJECT SPECIFIC COMMANDS ##############################
# Written by: @y00fie (http://yoofie.net // https://github.com/yoofie)

# Justfile - https://github.com/casey/just
### SHELL #############################################################
set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]
#set windows-shell := ["cmd.exe", "/c"]

### SETTINGS #########################################################
APP_NAME := "{{project-name}}"

### COMMANDS #########################################################

# Build the project
build: 
	@cargo build

# Clean the project
clean: 
	@cargo clean

# Run the project
run: 
	@cargo run

# Export the project
export:
	@. .\support\commands.ps1; exportAll {{project-name}}

# Calculate stats about your code
stats:
	@Write-host "Code stats for ./src" -f Blue
	@. .\support\commands.ps1; stats
	@tokei .\src --files > bin\code_stats.txt
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
