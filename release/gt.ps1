if (!$PSScriptRoot) {
  Write-Host "Goto cannot be used as executable."
}

$res = $( Invoke-Expression "$PSScriptRoot\..\build\win\debug\goto.exe $args" )

$res = $res -split '#'


if ($res.length -gt 3) {
  foreach ($line in $res) {
    Write-Host $line
  }
}

if ($res[1] -eq "0") {
  Set-Location $res[2]
} elseif ($res[1] -eq "1") {
  return $res[2]
}
if ($res[0] -eq "1") {
  Clear-Host
}
