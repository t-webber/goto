ry build
cd release 
all2exe -noOutput -noConsole
rm g.exe 
rm gtexe.exe
ps2exe gtexe.ps1 gtexe.exe -noConsole
cp ../.build/win/debug/goto.exe .
cd ..
