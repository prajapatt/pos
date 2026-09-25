param(
	[string]$Image = "build\bios\chut-os-bios.img"
)

$ErrorActionPreference = "Stop"

$root = (Resolve-Path (Join-Path $PSScriptRoot "..\.."))
$imagePath = Join-Path $root $Image
$qemu = Get-Command qemu-system-x86_64 -ErrorAction SilentlyContinue
if ($null -eq $qemu) {
	throw "qemu-system-x86_64 is required to run the BIOS image."
}
if (-not (Test-Path -LiteralPath $imagePath -PathType Leaf)) {
	throw "BIOS image not found: $imagePath. Run tools/build/build.ps1 first."
}

& $qemu.Source -drive "format=raw,file=$imagePath" -serial stdio -display none
