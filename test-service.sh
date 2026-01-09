#!/bin/bash
# ========================================
# Script de prueba rápida - Servicio Rust
# ========================================

echo ""
echo "========================================"
echo "🧪 PROBANDO SERVICIO RUST"
echo "========================================"
echo ""

echo "📡 1. Probando endpoint /hello"
curl -X GET http://localhost:9100/hello
echo ""
echo ""

echo "📡 2. Probando endpoint /available"
curl -X GET http://localhost:9100/available
echo ""
echo ""

echo "📡 3. Probando endpoint /default"
curl -X GET http://localhost:9100/default
echo ""
echo ""

echo "========================================"
echo "✅ Pruebas completadas"
echo "========================================"
echo ""
echo "Si ves errores de conexión:"
echo "  1. Asegúrate que el servicio esté corriendo: cargo run"
echo "  2. Verifica que esté en el puerto 9100"
echo ""
