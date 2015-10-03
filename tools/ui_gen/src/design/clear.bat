FOR /F "USEBACKQ DELIMS=," %%0 IN (`DIR  /A:D /B/S`) DO IF EXIST "%%0\DEBUG" RMDIR /S /Q  "%%0\DEBUG"
FOR /F "USEBACKQ DELIMS=," %%0 IN (`DIR  /A:D /B/S`) DO IF EXIST "%%0\release" RMDIR /S /Q  "%%0\release"
FOR /F "USEBACKQ DELIMS=," %%0 IN (`DIR  /A:D /B/S`) DO IF EXIST "%%0\GeneratedFiles" RMDIR /S /Q  "%%0\GeneratedFiles"
RMDIR /S /Q ipch
rem RMDIR /S /Q debug
rem RMDIR /S /Q release
del *.sdf
del *.opensdf
del *.aps
attrib  -h *.suo
del *.suo