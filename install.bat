@echo off
set PONY_DIR="C:\Program Files\Ponyfetch\"
set IMAGES_DIR=%PONY_DIR%ponies

:compile_tool
echo Compiling Ponyfetch...
cargo build --release

:createdirectories
echo Creating directories...
if not exist %PONY_DIR% mkdir %PONY_DIR%
if not exist %IMAGES_DIR% mkdir %IMAGES_DIR%

:copyfiles
echo Copying files...
xcopy /s ponies\ %IMAGES_DIR%
copy target\release\ponyfetch.exe %PONY_DIR%

:setpath
echo Setting PATH...
set PATH=%PATH%;%PONY_DIR%
echo Done!