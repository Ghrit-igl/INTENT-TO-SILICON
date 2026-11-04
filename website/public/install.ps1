$ErrorActionPreference = 'Stop'

$aayuRoot = Join-Path $env:USERPROFILE '.aayu'
$installBin = Join-Path $aayuRoot 'bin'
$installStd = Join-Path $aayuRoot 'std'
$installExamples = Join-Path $aayuRoot 'examples'
$tempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("aayu-install-" + [guid]::NewGuid().ToString('N'))
$packageDir = $null

try {
    New-Item -ItemType Directory -Force -Path $installBin, $installStd, $installExamples, $tempRoot | Out-Null

    $repoRoot = $PSScriptRoot
    if ($repoRoot -and (Test-Path (Join-Path $repoRoot 'bin\aayuc.exe'))) {
        $packageDir = Join-Path $repoRoot 'bin'
    } else {
        $archive = Join-Path $tempRoot 'aayu-windows.zip'
        $url = 'https://github.com/Ghrit-igl/INTENT-TO-SILICON/releases/latest/download/aayu-windows.zip'
        Invoke-WebRequest -Uri $url -OutFile $archive
        Expand-Archive -LiteralPath $archive -DestinationPath $tempRoot -Force
        $packageDir = $tempRoot
    }

    $vmSource = Join-Path $packageDir 'aayu.exe'
    if (-not (Test-Path $vmSource)) { $vmSource = Join-Path $packageDir 'aayu-vm.exe' }
    $compilerSource = Join-Path $packageDir 'aayuc.exe'
    if (-not (Test-Path $vmSource) -or -not (Test-Path $compilerSource)) {
        throw 'Package must include both the AAYU VM (aayu.exe/aayu-vm.exe) and bootstrap compiler (aayuc.exe).'
    }

    Copy-Item -LiteralPath $vmSource -Destination (Join-Path $installBin 'aayu.exe') -Force
    Copy-Item -LiteralPath $compilerSource -Destination (Join-Path $installBin 'aayuc.exe') -Force
    $vmAlias = Join-Path $packageDir 'aayu-vm.exe'
    if (Test-Path $vmAlias) {
        Copy-Item -LiteralPath $vmAlias -Destination (Join-Path $installBin 'aayu-vm.exe') -Force
    }

    $stdSource = Join-Path $packageDir 'std'
    if (Test-Path $stdSource) {
        Copy-Item -Path (Join-Path $stdSource '*') -Destination $installStd -Recurse -Force
    }
    $examplesSource = Join-Path $packageDir 'examples'
    if (Test-Path $examplesSource) {
        Copy-Item -Path (Join-Path $examplesSource '*') -Destination $installExamples -Recurse -Force
    }

    $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    $pathEntries = @($userPath -split ';' | Where-Object { $_ })
    if (-not ($pathEntries | Where-Object { $_.TrimEnd([char]92) -ieq $installBin.TrimEnd([char]92) })) {
        $newUserPath = (@($pathEntries) + $installBin) -join ';'
        [Environment]::SetEnvironmentVariable('Path', $newUserPath, 'User')
    }
    if (($env:Path -split ';' | Where-Object { $_.TrimEnd([char]92) -ieq $installBin.TrimEnd([char]92) }).Count -eq 0) {
        $env:Path = $env:Path + ';' + $installBin
    }

    & (Join-Path $installBin 'aayu.exe') --version
    Write-Host "AAYU installed in $installBin. The bootstrap compiler is installed beside the VM."
    Write-Host 'Try: aayu run path\to\program.aayu'
} finally {
    if (Test-Path $tempRoot) { Remove-Item -LiteralPath $tempRoot -Recurse -Force }
}
