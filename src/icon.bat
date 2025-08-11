@echo off
convert -version >nul 2>&1
if errorlevel 1 (
    echo ERROR: ImageMagick 'convert' not found. Zainstaluj ImageMagick i dodaj do PATH.
    pause
    exit /b 1
)

convert icon.png -define icon:auto-resize=256,128,64,48,32,16 icon.ico

echo 1 ICON "icon.ico" > icon.rc
cargo build --release

pause
