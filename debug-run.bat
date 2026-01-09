@echo off
echo ===================================================
echo   MODO DE DEPURACION - EN-CONTROL PRINT SERVICE
echo ===================================================
echo.
echo 1. Deteniendo el servicio de Windows (si esta corriendo)...
sc stop EnControlPrintService >nul 2>&1
timeout /t 2 /nobreak >nul

echo.
echo 2. Iniciando la aplicacion manualmente...
echo    Esta ventana debe permanecer abierta para que funcione.
echo    Intenta imprimir ahora desde la web.
echo.
echo    Si las impresoras aparecen ahora, el problema es de permisos del Servicio.
echo.
echo ---------------------------------------------------
echo  EJECUTANDO (Ctrl+C para salir)...
echo ---------------------------------------------------
target\release\en-control-print-service.exe
pause
