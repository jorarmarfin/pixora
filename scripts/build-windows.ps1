<#
.SYNOPSIS
  Compilación de release para Windows: pixora.exe + instalador Inno Setup.
.DESCRIPTION
  Debe ejecutarse en Windows (el enlazado MSVC con libvips no se puede hacer desde Linux;
  para eso está el workflow release.yml). Requisitos:
    - Node (.nvmrc), Rust (MSVC), pkg-config (choco install pkgconfiglite), Inno Setup 6
    - libvips `vips-dev-w64-all` descomprimido; ruta en $env:VIPSHOME (o -VipsHome)
    - Ejecutar desde "x64 Native Tools Command Prompt / Developer PowerShell" (dumpbin y lib en PATH)
.EXAMPLE
  pwsh scripts/build-windows.ps1
  pwsh scripts/build-windows.ps1 -SkipTests -SkipWebView2
#>
param(
  [string]$VipsHome = $env:VIPSHOME,
  [switch]$SkipTests,
  [switch]$SkipWebView2
)
$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')

if (-not $IsWindows -and $PSVersionTable.PSVersion.Major -ge 6) { throw 'build-windows.ps1 solo corre en Windows.' }
if (-not $VipsHome -or -not (Test-Path "$VipsHome\bin")) { throw 'Define VIPSHOME (carpeta de vips-dev-w64-all con bin\*.dll).' }
foreach ($c in 'node', 'npm', 'cargo', 'pkg-config') {
  if (-not (Get-Command $c -ErrorAction SilentlyContinue)) { throw "Falta '$c' en PATH." }
}

$env:VIPSHOME = $VipsHome
$env:PKG_CONFIG_PATH = "$VipsHome\lib\pkgconfig"
$env:PATH = "$VipsHome\bin;$env:PATH"

node scripts/check-versions.mjs
if ($LASTEXITCODE -ne 0) { throw 'Versiones inconsistentes.' }
$version = node -p "require('./package.json').version"

# Import libs MSVC de vips/glib/gobject (requiere dumpbin y lib del entorno MSVC).
if (-not (Test-Path "$VipsHome\lib\msvc\vips.lib")) {
  & "$PSScriptRoot\make-vips-implibs.ps1" -VipsHome $VipsHome
}
$env:LIB = "$VipsHome\lib\msvc;$env:LIB"

npm ci
if ($LASTEXITCODE -ne 0) { throw 'npm ci falló' }
if (-not $SkipTests) {
  npm test
  if ($LASTEXITCODE -ne 0) { throw 'npm test falló' }
  Push-Location src-tauri; cargo test; $rc = $LASTEXITCODE; Pop-Location
  if ($rc -ne 0) { throw 'cargo test falló' }
}

npm run tauri build -- --no-bundle
if ($LASTEXITCODE -ne 0) { throw 'tauri build falló' }

& "$PSScriptRoot\package-windows.ps1" -Version $version -VipsHome $VipsHome -SkipWebView2:$SkipWebView2
