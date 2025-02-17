@echo off
setlocal enabledelayedexpansion

:: Move one level up from the "scripts" directory to the project root
cd ..

:: Check if a module name was provided
if "%1"=="" (
    echo ❌ Error: Please provide a module name.
    echo Usage: scripts\add_module.bat module_name
    exit /b 1
)

set MODULE_NAME=%1
set MODULE_PATH=%CD%\%MODULE_NAME%
set COMMON_PATH=%CD%\common

:: Step 1: Create a new Rust module in the correct location
echo 🚀 Creating new Rust module: %MODULE_NAME%
cargo new "%MODULE_PATH%"

:: Step 2: Add the new module to Cargo.toml workspace members
echo 🔧 Adding %MODULE_NAME% to the workspace members...
(for /f "tokens=*" %%a in (Cargo.toml) do (
    set "line=%%a"
    echo !line! | findstr /C:"members = [" >nul && (
        echo !line!
        echo     "%MODULE_NAME%",
    ) || echo !line!
)) > Cargo_temp.toml
move /Y Cargo_temp.toml Cargo.toml >nul

:: Step 3: Add `common` as a dependency in the new module
echo 🔗 Adding common as a dependency...
echo.>> "%MODULE_PATH%\Cargo.toml"
echo [dependencies]>> "%MODULE_PATH%\Cargo.toml"
echo common = { path = "%COMMON_PATH%" }>> "%MODULE_PATH%\Cargo.toml"

:: Step 4: Confirm success
echo ✅ Module %MODULE_NAME% added successfully in the project root!
exit /b 0
