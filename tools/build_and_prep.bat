@echo off
setlocal

:: ==========================
:: Configuration Variables
:: ==========================
:: Set the project directory relative to this script
set "PROJECT_DIR=.."

:: Set the name of the executable (without extension)
set "EXECUTABLE_NAME=dds-convert-util"

:: Set the destination path where the executable will be placed
set "DEST_PATH=C:\Tools\%EXECUTABLE_NAME%"

:: Set the path to the built executable (relative to project directory)
set "BUILD_EXE_PATH=target\release\%EXECUTABLE_NAME%.exe"

:: ==========================
:: Build the application
:: ==========================
echo ---BUILD--------------------

:: Step 1: Change to the project directory
cd "%PROJECT_DIR%"

:: Step 2: Build the app
echo Building the Rust app...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo Failed to build the Rust app.
    exit /b %ERRORLEVEL%
)
echo.

:: Step 3: Create the destination directory if it doesn't exist
if not exist "%DEST_PATH%" (
    echo Creating directory "%DEST_PATH%"...
    mkdir "%DEST_PATH%"
)
echo.

:: Step 4: Move the built executable to the destination path
echo Moving the built executable to "%DEST_PATH%"...
move /Y "%BUILD_EXE_PATH%" "%DEST_PATH%"
if %ERRORLEVEL% neq 0 (
    echo Failed to move the executable.
    exit /b %ERRORLEVEL%
)
echo.

:: ==========================
:: Add DEST_PATH to the system PATH
:: ==========================
echo Checking if "%DEST_PATH%" is in the system PATH...
set "PATH_FOUND=0"
for %%i in ("%PATH:;=" "%") do (
    if /I "%%~i"=="%DEST_PATH%" (
        set "PATH_FOUND=1"
    )
)

if "%PATH_FOUND%"=="0" (
    echo "%DEST_PATH%" is not in the system PATH.
    echo Attempting to add "%DEST_PATH%" to the user PATH...

    :: Get the current user PATH
    for /F "tokens=2*" %%A in ('reg query "HKCU\Environment" /v Path 2^>nul') do (
        set "CURRENT_USER_PATH=%%B"
    )
    if "%CURRENT_USER_PATH%"=="" (
        set "CURRENT_USER_PATH=%DEST_PATH%"
    ) else (
        set "CURRENT_USER_PATH=%CURRENT_USER_PATH%;%DEST_PATH%"
    )

    :: Add DEST_PATH to the user PATH
    reg add "HKCU\Environment" /v Path /t REG_EXPAND_SZ /d "%CURRENT_USER_PATH%" /f >nul
    if %ERRORLEVEL% neq 0 (
        echo Failed to modify the user PATH.
        echo Please run this script with administrative privileges.
    ) else (
        echo "%DEST_PATH%" has been added to your user PATH. You may need to log off and back on for the changes to take effect.
    )
) else (
    echo "%DEST_PATH%" is already in the system PATH.
)
echo.

echo Setup complete. You can now run "%EXECUTABLE_NAME%" from any location.
endlocal
echo ----------------------------
echo.
