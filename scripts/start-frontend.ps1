$ErrorActionPreference = 'Stop'
$repo = Split-Path $PSScriptRoot -Parent
$trunk = Join-Path $repo '.tools/bin/trunk.exe'
if (!(Test-Path $trunk)) { $trunk = (Get-Command trunk -ErrorAction Stop).Source }
Push-Location (Join-Path $repo 'frontend')
try { & $trunk serve --locked } finally { Pop-Location }
