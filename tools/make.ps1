ry build
cd release 
all2exe | Out-Null
rm gt.exe
cp ../build/win/debug/goto.exe .
cd ..

if ($args[0]) {
  Invoke-Expression "gt $args"
}