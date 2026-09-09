param(
    [Parameter(Mandatory = $true)][string]$ArtifactDirectory,
    [Parameter(Mandatory = $true)][string]$Target
)

$ErrorActionPreference = "Stop"
$archive = Join-Path $ArtifactDirectory "stdbr-ffi-$Target.tar.gz"
if (-not (Test-Path $archive -PathType Leaf)) { throw "missing FFI archive: $archive" }

$temporaryDirectory = Join-Path ([System.IO.Path]::GetTempPath()) ("stdbr-ffi-smoke-" + [guid]::NewGuid())
New-Item -ItemType Directory -Path $temporaryDirectory | Out-Null
try {
    tar -xzf $archive -C $temporaryDirectory
    if ($LASTEXITCODE -ne 0) { throw "could not extract $archive" }
    $packageDirectory = Join-Path $temporaryDirectory "stdbr-ffi-$Target"
    foreach ($file in @("stdbr.h", "stdbr_ffi.dll", "stdbr_ffi.lib", "LICENSE")) {
        if (-not (Test-Path (Join-Path $packageDirectory $file) -PathType Leaf)) {
            throw "missing $file in FFI archive"
        }
    }

    $env:PATH = "$packageDirectory;$env:PATH"
    $probe = @'
Add-Type @"
using System.Runtime.InteropServices;
public static class StdbrFfi {
    [DllImport("stdbr_ffi.dll", CallingConvention = CallingConvention.Cdecl)]
    [return: MarshalAs(UnmanagedType.I1)]
    public static extern bool stdbr_cpf_is_valid([MarshalAs(UnmanagedType.LPStr)] string value);
}
"@
if (-not [StdbrFfi]::stdbr_cpf_is_valid("52998224725")) {
    throw "stdbr_cpf_is_valid rejected a valid CPF"
}
'@
    $encodedProbe = [Convert]::ToBase64String([Text.Encoding]::Unicode.GetBytes($probe))
    pwsh -NoLogo -NoProfile -NonInteractive -EncodedCommand $encodedProbe
    if ($LASTEXITCODE -ne 0) { throw "FFI probe failed" }
} finally {
    Remove-Item -Recurse -Force $temporaryDirectory
}
