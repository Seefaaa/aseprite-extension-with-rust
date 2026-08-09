$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$rootDir = Split-Path -Parent $scriptDir
$luaDir = Join-Path $rootDir "lua"
$distDir = Join-Path $rootDir "dist"
$zipPath = Join-Path $distDir "ext.zip"
$artifact = "test.dll"

Push-Location $rootDir
try {
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}
finally {
    Pop-Location
}

if (-not (Test-Path $luaDir)) {
    Write-Error "$luaDir not found"
    exit 1
}

Copy-Item (Join-Path $rootDir "target/release/$artifact") (Join-Path $luaDir $artifact) -Force

New-Item -ItemType Directory -Path $distDir -Force | Out-Null

if (Test-Path $zipPath) {
    Remove-Item $zipPath -Force
}

Compress-Archive -Path (Join-Path $luaDir "*") -DestinationPath $zipPath

Write-Host "created $zipPath"
