#!/usr/bin/env bash
set -eu
if command -v bash >/dev/null 2>&1 && [ -n "${BASH_VERSION:-}" ]; then
    set -o pipefail 2>/dev/null || true
fi

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

error() { printf "${RED}%s${NC}\n" "$*"; exit 1; }
info()  { printf "${YELLOW}%s${NC}\n" "$*"; }
ok()    { printf "${GREEN}%s${NC}\n" "$*"; }

# Buscar by en PATH
BY_PATH="$(command -v by 2>/dev/null || true)"
if [ -z "$BY_PATH" ]; then
    error "by no está instalado en el PATH"
fi

# Resolver directorio de instalación
if [ -L "$BY_PATH" ]; then
    BY_REAL="$(readlink -f "$BY_PATH" 2>/dev/null || readlink "$BY_PATH")"
    BY_DIR="$(dirname "$(dirname "$BY_REAL")")"
else
    BY_DIR="$(dirname "$(dirname "$BY_PATH")")"
fi

echo "Se eliminará by de:"
echo "  Binario:  $BY_PATH"
echo "  Directorio: $BY_DIR"
echo ""
echo -n "Continuar? [s/N] "
read -r CONFIRM
if [ "$CONFIRM" != "s" ] && [ "$CONFIRM" != "S" ]; then
    echo "Cancelado."
    exit 0
fi

# Eliminar binarios
rm -f "$BY_DIR/bin/by"
ok "Binarios eliminados: by"

# Eliminar runtime library
if [ -d "$BY_DIR/lib/by" ]; then
    rm -rf "$BY_DIR/lib/by"
    ok "Runtime library eliminada: $BY_DIR/lib/by"
fi

# Limpiar directorios vacíos (solo si no es /usr/local)
if [ "$BY_DIR" != "/usr/local" ]; then
    rmdir "$BY_DIR/bin" 2>/dev/null || true
    rmdir "$BY_DIR/lib" 2>/dev/null || true
    rmdir "$BY_DIR" 2>/dev/null || true
    ok "Directorios eliminados: $BY_DIR"
fi

echo ""
ok "by ha sido eliminado completamente."
echo ""
echo "Si agregaste ~/.by/bin al PATH, elimina esa línea de ~/.zshrc o ~/.bashrc"
