# install.ps1 — Windows PowerShell installer for Byle
#
# Usage:
#   iwr -Uri "https://raw.githubusercontent.com/kyal-tech/byle/main/scripts/install.ps1" | iex
#
# Environment variables:
#   $env:BY_VERSION = "v0.8.9"     Version to install (default: latest)
#   $env:BY_PREFIX = "C:\by"       Install directory (default: ~\.by)

param(
    [string]$Version = "v0.8.9",
    [string]$Prefix = ""
)

$Repo = "kyal-tech/byle"

# ─── Detect architecture ────────────────────────────────────

$RuntimeArch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()
if ($RuntimeArch -ne "X64") {
    Write-Host "Error: Windows architecture '$RuntimeArch' is not supported by this release (x64 only)."
    exit 1
}
$Arch = "x64"
$Platform = "windows-$Arch"
$Bundle = "by-windows-$Arch.zip"
$BundleUrl = "https://github.com/$Repo/releases/download/$Version/$Bundle"

Write-Host "Detected: $Platform"

# ─── Determine install prefix ───────────────────────────────

if (-not $Prefix) {
    $env:BY_PREFIX = if ($env:BY_PREFIX) { $env:BY_PREFIX } else { "$env:USERPROFILE\.by" }
} else {
    $env:BY_PREFIX = $Prefix
}

$BinDir = "$env:BY_PREFIX\bin"
$LibDir = "$env:BY_PREFIX\lib"

# ─── Download ───────────────────────────────────────────────

$TmpDir = "$env:TEMP\by-install"
if (Test-Path $TmpDir) { Remove-Item -Recurse -Force $TmpDir }
New-Item -ItemType Directory -Force -Path $TmpDir | Out-Null

$OutFile = "$TmpDir\$Bundle"
Write-Host "Downloading Byle $Version for $Platform..."
Write-Host "  $BundleUrl"

try {
    Invoke-WebRequest -Uri $BundleUrl -OutFile $OutFile -UseBasicParsing
} catch {
    Write-Host "Error: failed to download $Bundle"
    Write-Host "Check that $Version exists at:"
    Write-Host "  https://github.com/$Repo/releases"
    exit 1
}

# ─── Verify checksum ───────────────────────────────────────

$ChecksumUrl = "$BundleUrl.sha256"
$ChecksumFile = "$TmpDir\$Bundle.sha256"
try {
    Invoke-WebRequest -Uri $ChecksumUrl -OutFile $ChecksumFile -UseBasicParsing
    $ExpectedHash = (Get-Content $ChecksumFile -Raw).Trim() -split '\s+' | Select-Object -First 1
    $ActualHash = (Get-FileHash -Path $OutFile -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($ExpectedHash.ToLowerInvariant() -ne $ActualHash) {
        Write-Host "Error: checksum verification failed"
        exit 1
    }
    Write-Host "Checksum verified."
} catch {
    Write-Host "Error: unable to verify bundle checksum"
    exit 1
}

# ─── Extract ─────────────────────────────────────────────────

Write-Host "Extracting..."
try {
    Expand-Archive -Path $OutFile -DestinationPath $TmpDir -Force
} catch {
    Write-Host "Error: failed to extract $Bundle"
    exit 1
}

if (-not (Test-Path "$TmpDir\by.exe")) {
    Write-Host "Error: by.exe not found in archive"
    Get-ChildItem $TmpDir
    exit 1
}

# ─── Install ─────────────────────────────────────────────────

Write-Host "Installing to $env:BY_PREFIX..."

# Create directories
New-Item -ItemType Directory -Force -Path $BinDir | Out-Null
New-Item -ItemType Directory -Force -Path $LibDir | Out-Null

# Copy files
Copy-Item "$TmpDir\by.exe" "$BinDir\by.exe" -Force
if (Test-Path "$TmpDir\libbyc_runtime.a") {
    Copy-Item "$TmpDir\libbyc_runtime.a" "$LibDir\libbyc_runtime.a" -Force
} elseif (Test-Path "$TmpDir\byc_runtime.lib") {
    Copy-Item "$TmpDir\byc_runtime.lib" "$LibDir\byc_runtime.lib" -Force
}

# Clean up by install temp
Remove-Item -Recurse -Force $TmpDir

# ─── Add to PATH ────────────────────────────────────────────

$regPath = "HKCU:\Environment"
$currentPath = (Get-ItemProperty -Path $regPath -Name PATH -ErrorAction SilentlyContinue).PATH
$addedDirs = @()
if ($currentPath -notlike "*$BinDir*") { $addedDirs += $BinDir }
if ($addedDirs.Count -gt 0) {
    $newPath = ($addedDirs -join ';') + ";" + $currentPath
    Set-ItemProperty -Path $regPath -Name PATH -Value $newPath
    Write-Host "  Added to PATH:"
    foreach ($d in $addedDirs) { Write-Host "    $d" }
    try {
        $sig = '[DllImport("user32.dll", SetLastError=true, CharSet=CharSet.Auto)] public static extern IntPtr SendMessageTimeout(IntPtr hWnd, uint Msg, UIntPtr wParam, string lParam, uint fuFlags, uint uTimeout, out UIntPtr lpdwResult);'
        Add-Type -MemberDefinition $sig -Name NativeMethods -Namespace Win32 -ErrorAction Stop | Out-Null
        $result = [UIntPtr]::Zero
        [Win32.NativeMethods]::SendMessageTimeout(0xffff, 0x001a, [UIntPtr]::Zero, "Environment", 2, 5000, [ref]$result) | Out-Null
    } catch { }
} else {
    Write-Host "  PATH already configured"
}
$env:PATH = "$BinDir;$env:PATH"

# ─── Verify ─────────────────────────────────────────────────

Write-Host ""
try {
    $version = & "$BinDir\by.exe" --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        Write-Host "[OK] Byle $Version installed successfully!"
        Write-Host ""
        Write-Host "  Binary:  $BinDir\by.exe"
        Write-Host ""
        Write-Host "  Use now:  by --version"
        Write-Host "  Try:      by run examples\hello.by"
    } else {
        Write-Host "[WARN] Installation completed but verification failed."
    }
} catch {
    Write-Host "[WARN] Installation completed but 'by.exe' not found in PATH."
    Write-Host "   Restart your terminal or add manually:"
    Write-Host "    [Environment]::SetEnvironmentVariable('PATH', ""$BinDir;`$env:PATH"", 'User')"
}
