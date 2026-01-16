#!/bin/bash
echo "======================================================="
echo "   INSTALADOR DE SERVICIO EN-CONTROL (macOS)"
echo "======================================================="
echo

# Definir rutas
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
SOURCE_EXE="$SCRIPT_DIR/../../target/release/en-control-print-service"
DEST_EXE="/usr/local/bin/en-control-print-service"
PLIST_FILE="com.encontrol.printservice.plist"
DEST_PLIST="$HOME/Library/LaunchAgents/$PLIST_FILE"

# Verificar que existe el ejecutable compilado
if [ ! -f "$SOURCE_EXE" ]; then
    echo "❌ No se encuentra el ejecutable compilado."
    echo "Ejecuta 'cargo build --release' primero."
    exit 1
fi

echo "1. Deteniendo versiones anteriores..."
launchctl unload "$DEST_PLIST" 2>/dev/null || true

echo "2. Copiando ejecutable a $DEST_EXE..."
# Necesitamos sudo para escribir en /usr/local/bin si no tenemos permisos, 
# pero idealmente el usuario debería tener acceso. Usamos cp directo o sudo si falla.
cp "$SOURCE_EXE" "$DEST_EXE" || sudo cp "$SOURCE_EXE" "$DEST_EXE"
chmod +x "$DEST_EXE"

echo "3. Instalando agente Launchd en $DEST_PLIST..."
mkdir -p "$HOME/Library/LaunchAgents"
cp "$SCRIPT_DIR/$PLIST_FILE" "$DEST_PLIST"

echo "4. Cargando servicio..."
launchctl load "$DEST_PLIST"

echo
echo "======================================================="
echo "   INSTALACION COMPLETADA"
echo "======================================================="
echo "El servicio debería estar corriendo en segundo plano."
echo "Logs en: /tmp/en-control-print-service.log"
echo "Prueba: curl http://localhost:9100/hello"
echo
