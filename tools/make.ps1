ry build
cd release 
all2exe  
rm g.exe 
ps2exe g.ps1 gtexe.exe  
cd ..
echo "Moving $env:CARGO_TARGET_DIR/debug/goto.exe"
cp  $env:CARGO_TARGET_DIR/debug/goto.exe release/ 
