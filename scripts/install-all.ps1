<#
PowerShell script to install and build all SDKs and examples in this repo.
Use this on Windows PowerShell (v5.1) or PowerShell Core.

It runs:
- npm install & build for `sdk/typescript` and `examples/node-ts-example`
- cargo build for `sdk/rust`
- anchor build for `programs/zeris-validator` (optional; requires Anchor installed)
- pip install -r requirements.txt for Python deps
#>

Set-StrictMode -Version Latest
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
# Repo root is parent directory of the scripts folder
$root = Resolve-Path (Join-Path $scriptDir "..") | Select-Object -ExpandProperty Path
Push-Location $root

try {
    Write-Host "Installing TypeScript SDK..."
    Push-Location "$root\sdk\typescript"
    npm install
    npm run build
    Pop-Location

    Write-Host "Installing Node example..."
    Push-Location "$root\examples\node-ts-example"
    npm install
    npm run build
    Pop-Location

    Write-Host "Building Rust SDK..."
    Push-Location "$root\sdk\rust"
    cargo build
    Pop-Location

    Write-Host "Building Anchor program (optional; requires anchor CLI)..."
    Push-Location "$root\programs\zeris-validator"
    if (Get-Command anchor -ErrorAction SilentlyContinue) { anchor build } else { Write-Host "anchor not found; skipping" }
    Pop-Location

    Write-Host "Installing Python requirements..."
    python -m pip install -r "$root\requirements.txt"

} finally {
    Pop-Location
}
