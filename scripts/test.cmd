:: Main entry point
@setlocal
@set ERRORS=0
@set BUILDS_LOG="%TEMP%\bugsalot-builds-list.txt"
@echo Channel    Config     Platform   Result>%BUILDS_LOG%
@echo ---------------------------------------->>%BUILDS_LOG%
@call :build %*
@if "%ERRORS%" == "0" goto :all-builds-succeeded
@goto :some-builds-failed

:all-builds-succeeded
@echo.
@echo.
@type %BUILDS_LOG%
@echo.
@echo.
@echo Build succeeded!
@echo.
@echo.
@endlocal && exit /b 0

:some-builds-failed
@echo.
@echo.
@type %BUILDS_LOG%
@echo.
@echo.
@echo Build failed!
@echo.
@echo.
@endlocal && exit /b 1



:build
@setlocal

:: Parameters

@set "CHANNEL=%~1"
:: stable
:: beta
:: nightly
@if not defined CHANNEL set CHANNEL=*

@set "CONFIG=%~2"
:: debug
:: release
@if not defined CONFIG set CONFIG=*

@set "PLATFORM=%~3"
:: windows
:: android
:: linux (WSL)
:: wasm
@if not defined PLATFORM set PLATFORM=*

:: Handle wildcards

@if not "%CHANNEL%" == "*" goto :skip-channel-wildcard
  @call :build stable  "%CONFIG%" "%PLATFORM%"
  @call :build beta    "%CONFIG%" "%PLATFORM%"
  @call :build nightly "%CONFIG%" "%PLATFORM%"
  @endlocal && set ERRORS=%ERRORS%&& exit /b 0
:skip-channel-wildcard

@if not "%CONFIG%" == "*" goto :skip-config-wildcard
  @call :build "%CHANNEL%" debug   "%PLATFORM%"
  @call :build "%CHANNEL%" release "%PLATFORM%"
  @endlocal && set ERRORS=%ERRORS%&& exit /b 0
:skip-config-wildcard

@if not "%PLATFORM%" == "*" goto :skip-platform-wildcard
  @call :build "%CHANNEL%" "%CONFIG%" windows
  @call :build "%CHANNEL%" "%CONFIG%" android
  @call :build "%CHANNEL%" "%CONFIG%" linux
  @call :build "%CHANNEL%" "%CONFIG%" wasm
  @endlocal && set ERRORS=%ERRORS%&& exit /b 0
:skip-platform-wildcard

:: If we got this far, CHANNEL, CONFIG, and PLATFORM are all non-wildcards.

@set "PAD=                      "
@set "PAD_CHANNEL=%CHANNEL%%PAD%"
@set "PAD_CONFIG=%CONFIG%%PAD%"
@set "PAD_PLATFORM=%PLATFORM%%PAD%"
@set "PAD_CHANNEL=%PAD_CHANNEL:~0,10%"
@set "PAD_CONFIG=%PAD_CONFIG:~0,10%"
@set "PAD_PLATFORM=%PAD_PLATFORM:~0,10%"

:: Skip some builds due to earlier errors, non-bugsalot bugs, being too lazy to install the beta toolchain, etc.

@if not "%ERRORS%" == "0" goto :build-one-skipped
@if /I "%CHANNEL%"  == "beta"    echo Skipping %CHANNEL% %CONFIG% %PLATFORM%: Beta toolchain&& goto :build-one-skipped
@if /I "%PLATFORM%" == "android" echo Skipping %CHANNEL% %CONFIG% %PLATFORM%: Build not fully configured&& goto :build-one-skipped
@if /I "%PLATFORM%" == "linux" if defined CI echo Skipping %CHANNEL% %CONFIG% %PLATFORM%: Appveyor doesn't have WSL installed&& goto :build-one-skipped

:: Parameters -> Settings

@rustup toolchain list | findstr default | findstr x86_64 && set NATIVE_ARCH=x86_64|| set NATIVE_ARCH=i686
@set SHELL_PREFIX= 
@set SHELL_POSTFIX= 
@set CARGO_TEST_COMMAND=test
@set CARGO_FLAGS= 

@if /i "%CONFIG%" == "debug"     echo.>NUL
@if /i "%CONFIG%" == "release"   set CARGO_FLAGS=%CARGO_FLAGS% --release

@if /i "%PLATFORM%" == "windows" echo.>NUL
:: Infer --target from toolchain

@if /i "%PLATFORM%" == "android" set CARGO_FLAGS=%CARGO_FLAGS% --target=%NATIVE_ARCH%-linux-android

@if /i "%PLATFORM%" == "linux"   set "SHELL_PREFIX=%WINDIR%\System32\bash --login -c '"
@if /i "%PLATFORM%" == "linux"   set "SHELL_POSTFIX='"
:: Infer --target from toolchain

@if /i "%PLATFORM%" == "wasm"    set CARGO_FLAGS=%CARGO_FLAGS% --target=wasm32-unknown-unknown
@if /i "%PLATFORM%" == "wasm"    set CARGO_TEST_COMMAND=web test
@if /i "%PLATFORM%" == "wasm" if /i "%CHANNEL%" == "nightly" echo WARNING: nightly cargo web test is broken, just building... && set CARGO_TEST_COMMAND=build

@if /i NOT "%PLATFORM%" == "wasm" goto :skip-add-chrome-to-path
@where cargo-web >NUL 2>NUL || cargo install cargo-web
@where chrome >NUL 2>NUL && goto :skip-add-chrome-to-path
@if exist "%ProgramFiles(x86)%\Google\Chrome\Application\chrome.exe" set PATH=%ProgramFiles(x86)%\Google\Chrome\Application\;%PATH% && goto :skip-add-chrome-to-path
@if exist      "%ProgramFiles%\Google\Chrome\Application\chrome.exe" set      PATH=%ProgramFiles%\Google\Chrome\Application\;%PATH% && goto :skip-add-chrome-to-path
:skip-add-chrome-to-path

:: Actually build

%SHELL_PREFIX%cargo +%CHANNEL% %CARGO_TEST_COMMAND% %CARGO_FLAGS%%SHELL_POSTFIX% || goto :build-one-error
%SHELL_PREFIX%cargo +%CHANNEL% build --examples %CARGO_FLAGS%%SHELL_POSTFIX%     || goto :build-one-error
@goto :build-one-successful

:: Exit from :build

:build-one-skipped
@echo %PAD_CHANNEL% %PAD_CONFIG% %PAD_PLATFORM% skipped>>%BUILDS_LOG%
@endlocal && set ERRORS=%ERRORS%&& exit /b 0

:build-one-successful
@echo %PAD_CHANNEL% %PAD_CONFIG% %PAD_PLATFORM% ok>>%BUILDS_LOG%
@endlocal && set ERRORS=%ERRORS%&& exit /b 0

:build-one-error
@echo %PAD_CHANNEL% %PAD_CONFIG% %PAD_PLATFORM% ERRORS>>%BUILDS_LOG%
@endlocal && set /A ERRORS=%ERRORS% + 1&& exit /b 1
