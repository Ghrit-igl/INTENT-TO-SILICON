# AAYU Automated Test Harness
Write-Host "==================================================" -ForegroundColor Cyan
Write-Host "       AAYU INTENT-TO-SILICON TEST RUNNER         " -ForegroundColor Cyan
Write-Host "==================================================" -ForegroundColor Cyan

$binAayuc = if ($env:AAYU_COMPILER_PATH) { $env:AAYU_COMPILER_PATH } else { "$PSScriptRoot\..\bin\aayuc.exe" }
$binVm = if ($env:AAYU_VM_PATH) { $env:AAYU_VM_PATH } else { "$PSScriptRoot\..\bin\aayu-vm.exe" }

if (-not (Test-Path $binAayuc) -or -not (Test-Path $binVm)) {
    Write-Host "[ERROR] Compiler or VM not found in bin/" -ForegroundColor Red
    exit 1
}

$testFiles = Get-ChildItem -Path $PSScriptRoot -Filter "*.aayu" | Where-Object { $_.Name -ne "test_lint_target.aayu" }
$expectedOutput = @{
    "test_01_vars.aayu" = @("AAYU", "1", "2", "0", "Hello from AAYU")
    "test_02_floats.aayu" = @("Area:", "314.159", "Divided:", "90", "Subtracted:", "150")
    "test_03_loops.aayu" = @("Sum of 0..9:", "45", "PASS: Sum is 45")
    "test_04_fibonacci.aayu" = @("fib(20):", "6765", "PASS: fib(20) matches 6765")
    "test_05_signed_numbers.aayu" = @("-5", "-2.5", "-140737488355328", "-5")
    "test_06_functions.aayu" = @("120")
    "test_07_arrays.aayu" = @("20", "null", "3", "A")
    "test_08_dictionaries.aayu" = @("IDENT", "aayu", "null")
    "test_09_globals.aayu" = @("5", "5", "2", "10")
    "test_10_modules.aayu" = @("42")
    "test_suite_proof.aayu" = @("Language: AAYU", "180", "90", "fib(20):", "6765", "TEST PASSED: Fibonacci matches 6765!")
}
$passed = 0
$failed = 0

foreach ($file in $testFiles) {
    $baseName = $file.BaseName
    $aycFile = "$PSScriptRoot\$baseName.ayc"
    
    Write-Host "[TEST] Running $($file.Name)..." -NoNewline
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    
    # 1. Compile
    & $binAayuc $file.FullName -o $aycFile | Out-Null
    if ($LASTEXITCODE -ne 0 -or -not (Test-Path $aycFile)) {
        Write-Host " FAIL (Compilation)" -ForegroundColor Red
        $failed++
        continue
    }
    
    # 2. Execute
    $out = & $binVm $aycFile 2>&1
    $sw.Stop()
    
    if ($LASTEXITCODE -eq 0) {
        if ($expectedOutput.ContainsKey($file.Name)) {
            $actualLines = @($out | ForEach-Object { "$($_)".TrimEnd() })
            $expectedLines = $expectedOutput[$file.Name]
            if (($actualLines -join "`n") -ne ($expectedLines -join "`n")) {
                Write-Host " FAIL (Output mismatch)" -ForegroundColor Red
                Write-Host "Expected: $($expectedLines -join ' | ')" -ForegroundColor Yellow
                Write-Host "Actual:   $($actualLines -join ' | ')" -ForegroundColor Yellow
                $failed++
                Remove-Item $aycFile -Force -ErrorAction SilentlyContinue
                continue
            }

            # The CLI's experimental C emitter is currently gated off. Keep
            # --jit output aligned with the verified VM until it has parity.
            $jitOut = & $binVm $aycFile --jit 2>$null
            if ($LASTEXITCODE -ne 0 -or ((@($jitOut | ForEach-Object { "$($_)".TrimEnd() })) -join "`n") -ne ($expectedLines -join "`n")) {
                Write-Host " FAIL (--jit output mismatch)" -ForegroundColor Red
                $failed++
                Remove-Item $aycFile -Force -ErrorAction SilentlyContinue
                continue
            }
        }
        Write-Host " PASS ($($sw.ElapsedMilliseconds) ms)" -ForegroundColor Green
        $passed++
    } else {
        Write-Host " FAIL (Runtime error)" -ForegroundColor Red
        Write-Host $out
        $failed++
    }
    
    Remove-Item $aycFile -Force -ErrorAction SilentlyContinue
}

# Run Security Suite
$secScript = "$PSScriptRoot\test_malformed_bytecode.ps1"
if (Test-Path $secScript) {
    & powershell -ExecutionPolicy Bypass -File $secScript
    if ($LASTEXITCODE -ne 0) {
        $failed++
    }
}

$selfhostScript = "$PSScriptRoot\run_selfhost_probes.ps1"
if (Test-Path $selfhostScript) {
    & powershell -ExecutionPolicy Bypass -File $selfhostScript
    if ($LASTEXITCODE -ne 0) { $failed++ }
}

# Run Static Linter Verification
$lintTarget = "$PSScriptRoot\test_lint_target.aayu"
$binAayu = "$PSScriptRoot\..\bin\aayu.exe"
if ((Test-Path $lintTarget) -and (Test-Path $binAayu)) {
    Write-Host "[TEST] Running Static Linter on test_lint_target.aayu... " -NoNewline
    $lintOut = & $binAayu lint $lintTarget 2>&1
    if ($lintOut -match "deprecated keyword 'var'") {
        Write-Host "PASS" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "FAIL (Linter did not detect deprecated var)" -ForegroundColor Red
        $failed++
    }
}

# Run Native MCP Server Verification
if (Test-Path $binAayu) {
    Write-Host "[TEST] Running Native MCP Server Verification (--verify)... " -NoNewline
    $mcpOut = & $binAayu mcp --verify 2>&1
    if ($LASTEXITCODE -eq 0 -and $mcpOut -match "9/9 tests passed 100% operational") {
        Write-Host "PASS (9/9 MCP tests verified)" -ForegroundColor Green
        $passed++
    } else {
        Write-Host "FAIL (MCP Verification failed)" -ForegroundColor Red
        Write-Host $mcpOut
        $failed++
    }
}

Write-Host "--------------------------------------------------" -ForegroundColor Cyan
Write-Host "Results: $passed Passed | $failed Failed" -ForegroundColor $(if ($failed -eq 0) { "Green" } else { "Red" })
Write-Host "==================================================" -ForegroundColor Cyan

if ($failed -gt 0) { exit 1 } else { exit 0 }
