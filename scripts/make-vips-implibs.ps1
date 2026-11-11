<#
.SYNOPSIS
  Genera las import libs MSVC (.lib) que necesita el crate `libvips` a partir de las DLL de MXE.
.DESCRIPTION
  El build.rs del crate enlaza `vips`, `glib-2.0` y `gobject-2.0`. Los binarios `vips-dev-w64-all`
  traen DLL pero no .lib para MSVC. Requiere el entorno de MSVC (dumpbin y lib en PATH).
  Escribe en <VipsHome>\lib\msvc y exporta esa ruta en LIB (GITHUB_ENV si existe).
#>
param([string]$VipsHome = $env:VIPSHOME)
$ErrorActionPreference = 'Stop'
if (-not $VipsHome) { throw 'VIPSHOME no definido' }
$out = Join-Path $VipsHome 'lib\msvc'
New-Item $out -ItemType Directory -Force | Out-Null

$map = @{
  'libvips-42.dll'        = 'vips'
  'libglib-2.0-0.dll'     = 'glib-2.0'
  'libgobject-2.0-0.dll'  = 'gobject-2.0'
}
foreach ($dll in $map.Keys) {
  $path = Join-Path $VipsHome "bin\$dll"
  if (-not (Test-Path $path)) { throw "No existe $path" }
  $name = $map[$dll]
  $exports = & dumpbin /exports $path |
    ForEach-Object { if ($_ -match '^\s+\d+\s+[0-9A-Fa-f]+\s+[0-9A-Fa-f]{8}\s+(\S+)') { $Matches[1] } }
  if (-not $exports) { throw "Sin exportaciones en $dll" }
  $def = Join-Path $out "$name.def"
  @("LIBRARY $dll", 'EXPORTS') + $exports | Set-Content $def -Encoding ascii
  & lib /nologo /def:$def /machine:x64 "/out:$out\$name.lib"
  if ($LASTEXITCODE -ne 0) { throw "lib.exe falló para $name" }
  Write-Host "OK $name.lib ($($exports.Count) símbolos)"
}
if ($env:GITHUB_ENV) { "LIB=$out;$env:LIB" | Out-File -Append $env:GITHUB_ENV -Encoding utf8 }
