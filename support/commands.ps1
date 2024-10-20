<# Written by @y00fie (http://yoofie.net) #>

function exportAll([string]$name) {
	$theDate = Get-Date -Format "MM.dd.yyyy - hh.mm.ss tt"
	$export_name = "[" + $theDate + "] $name.zip"

		$compress = @{
			Path             = ".\"
			CompressionLevel = "Fastest"
			DestinationPath  = $export_name
		}
		Compress-Archive @compress -Force
		Write-host "Succesfully exported '$export_name'" -f Green


}

function exportCodeOnly([string]$name) {
	$theDate = Get-Date -Format "MM.dd.yyyy - hh.mm.ss tt"
	$export_name = "[" + $theDate + "] $name.zip"
	$archiveList = ".\src", ".\support", ".\.vscode", ".\.gitignore", ".\justfile", ".\readme.md", ".cargo", "Cargo.toml", ".\support\cbindgen.toml", ".\support\cppbindgen.toml", "rustfmt.toml", "build.rs"
	$all_files_present = 1;
	$missingItem = 'None';

	# Ensure neccessary files/folders are present
	ForEach ($item in $archiveList) {
		
		If (!(Test-Path $item)) {
			Write-host "'$item'" -f Yellow -NoNewline
			Write-host " is missing!" -f Red
			$missingItem = $item
			$all_files_present = false
		}
	}
	If($all_files_present) {
		$compress = @{
			Path             = $archiveList
			CompressionLevel = "Fastest"
			DestinationPath  = $export_name
		}
		Compress-Archive @compress -Force
		Write-host "Succesfully exported '$export_name'" -f Green
	} else {
		Write-host "Failed to export, '$missingItem' is missing!" -f Red
	}
	
	
	# https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.archive/compress-archive?view=powershell-7.3
}

function beyond_compare_files([string]$current_path, [string]$input1, [string]$input2){
	$env:Path += ';C:\Program Files\Beyond Compare 4\'
	#Write-host $env:Path
	$cmd = "BCompare.exe `"$current_path\$input1`" `"$current_path\$input2`"";

	Invoke-Expression "& $cmd"
}

function windowsTerminal() {
	#@%LOCALAPPDATA%\Microsoft\WindowsApps\wt.exe -d %cd%
	Write-host "Launched windows terminal" -f Green	
}

function runDev() {
	$installationPath = .\support\vswhere.exe -prerelease -latest -property installationPath
	Write-host "Visual Studio Location: $installationPath" -f Green
	$vcvars64 = "$installationPath" + "\VC\Auxiliary\Build\vcvars64.bat"
	$vcvars642 = "$installationPath" + "\VC\Auxiliary\Build\"
	$vccmd = "$installationPath" + "\Common7\Tools\LaunchDevCmd.bat"
	Write-host "`t vcvars64 path: $vcvars64" -f Green

	$thePath = replaceSlashes($pwd)
	
	Write-Host "      APPDATA:" + $env:APPDATA
	Write-Host "LOCAL APPDATA:" + $env:LOCALAPPDATA

	$wtt3 = "wt -d $thePath cmd /k `'$vcvars64'"

	Write-Host "The path: $thePath"
	Invoke-Expression "& $wtt3"
}

function stats() {
	If (Test-Path '.\target') {
		
	}
 else {
		mkdir bin
	}
}

function replaceSlashes([string]$inputVal) {
	$pattern = '\\'
	$result = $inputVal -replace $pattern, '/'
	return $result
}

# TEST FUNCTIONS
function Add-Path($Path) {
	$Path = [Environment]::GetEnvironmentVariable("PATH", "Machine") + [IO.Path]::PathSeparator + $Path
	[Environment]::SetEnvironmentVariable( "Path", $Path, "Machine" )
}

function print_path() {
	($env:path).split(';')
}

# From ./support folder, run following pecific function like so:
# powershell -command "& { . .\addPath.ps1; printSomething }"
function printSomething() {
	Write-host "PRINT SOMETHING TEST FUNCTION" -f Blue
	Write-host "USER DOMAIN: $Env:UserDomain"
	Write-host "COMPUTER NAME: $Env:ComputerName"

}

function checkFolder([string]$folderPath){
	If (!(Test-Path ".\$folderPath")) {
		mkdir $folderPath
	} 
}

function testJson([string] $inputJson, [string] $input1) {
	$myJson = Get-Content $inputJson -Raw | ConvertFrom-Json
	$date = Get-Date | Select-Object -Property * | ConvertTo-Json
	Write-Host $date -f Cyan
	$vv = $myJson.version
	$vv2 = $myJson.configurations[1].name
	Write-Host "Version = $vv"
	Write-Host "$vv2"
	Write-Host "Version = $myJson.version"
	$myJson.version | Out-File -FilePath .\$binFolder\www.txt

}

function git_info([string] $inputJson) {
	$myJson = git rev-parse --short HEAD
	$tag = git describe --tags --always
	$branch = git branch --show-current
	$ssize = et -l 1 -ssize-rev .
	Write-Host "$myJson"
	Write-Host "$tag"
	Write-Host "$branch"
	Write-Host "$ssize"
}

function generate_version_info([string] $app_name, [string] $v_major, [string] $v_minor, [string] $v_patch, [string] $v_revision){
	$git_commit = git rev-parse --short HEAD
	$git_branch = git branch --show-current
	$git_tag = git describe --tags --always

	$domain = $Env:UserDomain
	$user = $Env:UserName

	$full_date = Get-Date -UFormat "%A %m/%d/%Y %R %Z"
	$pretty_date = Get-Date
	$date_year = Get-Date -UFormat "%Y"
	$date_month = Get-Date -UFormat "%m"
	$date_day = Get-Date -UFormat "%d"
	$date_hour = Get-Date -UFormat "%H"
	$date_minute = Get-Date -UFormat "%M"
	$date_sec = Get-Date -UFormat "%S"
	
	$pc_name = [System.Security.Principal.WindowsIdentity]::GetCurrent().Name
	$template = @"
/* **************************************
File Name: Generated version Info
File Generated: $pretty_date
*************************************** */
#ifndef _VERSION_INFO_H_
#define _VERSION_INFO_H_

/* **************************************
	#defines
*************************************** */
#define APP_NAME "$app_name"

/* SW Version */
#define RELEASE_VERSION "$v_major.$v_minor.$v_patch.$v_revision"
#define MAJOR_VERSION $v_major
#define MINOR_VERSION $v_minor
#define PATCH_VERSION $v_patch
#define REVISION_VERSION $v_revision

/* Git Tags */
#define GIT_BUILD_VERSION $git_commit
#define GIT_BUILD_VERSION_TAG "$git_tag"
#define GIT_BUILD_BRANCH_NAME "$git_branch"

/* Info about the person who generated this SW*/
#define BUILDER_USER_DOMAIN "$domain"
#define BUILDER_USER_COMPUTER_NAME "$pc_name"
#define C_USER "$user"

/* Timestamps */
#define PROJ_GENERATED_TIMESTAMP "$full_date"
#define PROJ_GENERATED_PRETTY_TIMESTAMP "$pretty_date"

#define BUILDTIME_YEAR $date_year
#define BUILDTIME_MONTH $date_month
#define BUILDTIME_DAY $date_day
#define BUILDTIME_HOUR $date_hour
#define BUILDTIME_MINUTE $date_minute
#define BUILDTIME_SECOND $date_sec

#endif
"@

	$template | Out-File "./src/versionInfo.h"
}

# Reference material
# https://stackoverflow.com/questions/1405750/calling-a-specific-powershell-function-from-the-command-line
# https://stackoverflow.com/questions/12850487/invoke-a-second-script-with-arguments-from-a-script
# https://devblogs.microsoft.com/cppblog/finding-the-visual-c-compiler-tools-in-visual-studio-2017/
# # https://superuser.com/questions/1091344/powershell-to-delete-all-files-with-a-certain-file-extension
