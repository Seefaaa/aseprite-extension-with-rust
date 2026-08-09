@echo off
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0zip.ps1" %*
exit /b %ERRORLEVEL%
