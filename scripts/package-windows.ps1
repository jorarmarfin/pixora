<#
.SYNOPSIS
  Prepara el directorio de distribución de Windows y compila el instalador Inno Setup.
.DESCRIPTION
  Requisitos: `npm run tauri build -- --no-bundle` ya ejecutado, libvips (vips-dev-w64-all) en $env:VIPSHOME,
  Inno Setup 6 (ISCC.exe en PATH o ruta por defecto).
.EXAMPLE
  pwsh scripts/package-windows.ps1 -Version 1.0.0
#>
param(
  [Parameter(Mandatory)][string]$Version,
  [string]$VipsHome = $env:VIPSHOME,
  [switch]$SkipWebView2
)
$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')

$exe = 'src-tauri\target\release\pixora.exe'
if (-not (Test-Path $exe)) { throw "No existe $exe. Ejecuta: npm run tauri build -- --no-bundle" }
if (-not $VipsHome -or -not (Test-Path "$VipsHome\bin")) { throw "VIPSHOME no apunta a libvips (carpeta con bin\*.dll)." }

$dist = 'dist-win'
Remove-Item $dist -Recurse -Force -ErrorAction SilentlyContinue
New-Item $dist -ItemType Directory | Out-Null

Copy-Item $exe $dist
# libvips y todas sus dependencias son DLL junto al exe (arranque sin libvips preinstalado).
Copy-Item "$VipsHome\bin\*.dll" $dist
# Licencias de terceros (libvips es LGPL-2.1+: se distribuye dinámicamente y se acompaña su licencia).
Copy-Item 'docs\third-party.md' $dist -ErrorAction SilentlyContinue
if (Test-Path "$VipsHome\LICENSE") { Copy-Item "$VipsHome\LICENSE" "$dist\LICENSE-libvips.txt" }

if (-not $SkipWebView2) {
  # Bootstrapper oficial (Evergreen); el instalador solo lo ejecuta si WebView2 no está instalado.
  Invoke-WebRequest 'https://go.microsoft.com/fwlink/p/?LinkId=2124703' -OutFile "$dist\MicrosoftEdgeWebview2Setup.exe"
}

$iscc = (Get-Command ISCC.exe -ErrorAction SilentlyContinue)?.Source
if (-not $iscc) { $iscc = "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe" }
if (-not (Test-Path $iscc)) { throw 'No se encontró ISCC.exe (Inno Setup 6).' }

& $iscc "/DAppVersion=$Version" '/DSourceDir=..\dist-win' '/DOutputDir=..\out' 'installer\pixora.iss'
if ($LASTEXITCODE -ne 0) { throw "ISCC falló ($LASTEXITCODE)" }
Get-ChildItem out\Pixora-Setup-*.exe | ForEach-Object { Write-Host "OK: $($_.FullName)" }
