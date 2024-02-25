if (!$PSScriptRoot) {
  # gtexe.exe
    $CurrentDIR = Split-Path (Convert-Path ([environment]::GetCommandLineArgs()[0]))
} else {
  # gt.ps1
  $CurrentDIR = $PSScriptRoot
}

$Result = Invoke-Expression "$CurrentDIR\goto.exe $args" 
$Result = $Result -split '#'

if ($Result.length -gt 3) {
  return $Result
}

if ($Result[1] -eq "1") {
  return $Result[2] 
}

if ($Result[0] -eq "1") {
  return
}

Set-Location $Result[2]
