# This is a PowerShell script to execute a Rust Cargo command
# Replace 'YourCargoManifestPath' with the actual path to your Cargo.toml file

$CargoManifestPath = Join-Path (Get-Item $PSScriptRoot).FullName "YourCargoManifestPath"
$TargetDir = "C:\tmp\codecrafters-http-server-target" # Replace with your desired target directory

& cargo run --quiet --release --target-dir=$TargetDir --manifest-path $CargoManifestPath -- $args
