@echo off
echo ========================================
echo PROBANDO EN-CONTROL PRINT SERVICE
echo ========================================
echo.

echo [1/4] Test: Health Check (/hello)
echo.
curl http://localhost:9100/hello
echo.
echo.

echo [2/4] Test: Lista de Impresoras (/available)
echo.
curl http://localhost:9100/available
echo.
echo.

echo [3/4] Test: Impresora por Defecto (/default)
echo.
curl http://localhost:9100/default
echo.
echo.

echo [4/4] Test: Verificar Actualizaciones (/update)
echo.
curl http://localhost:9100/update
echo.
echo.

echo ========================================
echo PRUEBAS COMPLETADAS
echo ========================================
pause
