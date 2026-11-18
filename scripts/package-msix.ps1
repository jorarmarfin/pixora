<#
.SYNOPSIS
  Empaqueta Pixora como MSIX (Microsoft Store) a partir de dist-win.
.DESCRIPTION
  Requisitos: haber ejecutado antes scripts/package-windows.ps1 (genera dist-win con pixora.exe + DLL de libvips)
  y Windows SDK (makeappx.exe / signtool.exe). Usa installer/msix/AppxManifest.xml y los iconos de src-tauri/icons.
  Para subir a la Store NO hace falta firmar (Microsoft firma el paquete). Para instalarlo localmente sí:
  pasar -PfxPath con un certificado cuyo Subject sea exactamente el Publisher del manifiesto.
.EXAMPLE
  pwsh scripts/package-msix.ps1 -Version 1.0.0
  pwsh scripts/package-msix.ps1 -Version 1.0.0 -PfxPath test.pfx -PfxPassword 1234
#>
param(
  [Parameter(Mandatory)][string]$Version,
  [string]$PfxPath,
  [string]$PfxPassword
)
$ErrorActionPreference = 'Stop'
Set-Location (Join-Path $PSScriptRoot '..')

if ($Version -notmatch '^\d+\.\d+\.\d+$') { throw "Versión inválida '$Version' (usar X.Y.Z)." }
if (-not (Test-Path 'dist-win\pixora.exe')) { throw 'No existe dist-win\pixora.exe. Ejecuta antes scripts/package-windows.ps1.' }

function Find-SdkTool([string]$name) {
  $cmd = Get-Command $name -ErrorAction SilentlyContinue
  if ($cmd) { return $cmd.Source }
  $kits = "${env:ProgramFiles(x86)}\Windows Kits\10\bin"
  $found = Get-ChildItem $kits -Recurse -Filter $name -ErrorAction SilentlyContinue |
    Where-Object { $_.FullName -match '\\x64\\' } | Sort-Object FullName -Descending | Select-Object -First 1
  if (-not $found) { throw "No se encontró $name (instala el Windows SDK)." }
  $found.FullName
}

$stage = 'dist-msix'
Remove-Item $stage -Recurse -Force -ErrorAction SilentlyContinue
New-Item "$stage\Assets" -ItemType Directory | Out-Null

# Contenido de la app (sin el bootstrapper de WebView2: en MSIX no aplica).
Get-ChildItem dist-win -Exclude 'MicrosoftEdgeWebview2Setup.exe' | Copy-Item -Destination $stage -Recurse

# Logos exigidos por el manifiesto (ya generados para Tauri).
foreach ($f in 'StoreLogo.png', 'Square44x44Logo.png', 'Square150x150Logo.png') {
  $src = "src-tauri\icons\$f"
  if (-not (Test-Path $src)) { throw "Falta $src (ejecuta scripts/generate-icons.sh)." }
  Copy-Item $src "$stage\Assets\$f"
}

# Manifiesto: la versión MSIX tiene 4 partes y la última debe ser 0 para la Store.
(Get-Content 'installer\msix\AppxManifest.xml' -Raw -Encoding UTF8).Replace('@VERSION@', "$Version.0") |
  Set-Content "$stage\AppxManifest.xml" -Encoding UTF8 -NoNewline

New-Item out -ItemType Directory -Force | Out-Null
$msix = "out\Pixora-$Version-x64.msix"
Remove-Item $msix -ErrorAction SilentlyContinue
& (Find-SdkTool 'makeappx.exe') pack /d $stage /p $msix /o
if ($LASTEXITCODE -ne 0) { throw "makeappx falló ($LASTEXITCODE)" }

if ($PfxPath) {
  $sign = @('sign', '/fd', 'SHA256', '/f', $PfxPath)
  if ($PfxPassword) { $sign += @('/p', $PfxPassword) }
  & (Find-SdkTool 'signtool.exe') @sign $msix
  if ($LASTEXITCODE -ne 0) { throw "signtool falló ($LASTEXITCODE)" }
}
Write-Host "OK: $((Resolve-Path $msix).Path)"
