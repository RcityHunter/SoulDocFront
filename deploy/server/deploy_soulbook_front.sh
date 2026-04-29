#!/usr/bin/env bash
set -euo pipefail

APP_DIR="/root/soulhub/SoulBookFront"
WEB_ROOT="/srv/soulbook-frontend"
LOCK_FILE="/tmp/soulbook-frontend-deploy.lock"
LOG_FILE="/tmp/soulbook-frontend-deploy.log"

exec > >(tee -a "$LOG_FILE") 2>&1
exec 9>"$LOCK_FILE"

if ! flock -n 9; then
  echo "[$(date -Is)] another frontend deployment is already running"
  exit 0
fi

echo "[$(date -Is)] frontend deployment started"

cd "$APP_DIR"

OLD_REV="$(git rev-parse --short HEAD)"
git fetch origin main
git merge --ff-only origin/main
NEW_REV="$(git rev-parse --short HEAD)"

dx build --release

if [ -d "$APP_DIR/dist" ]; then
  BUILD_DIR="$APP_DIR/dist"
elif [ -d "$APP_DIR/target/dx/soulbook_ui/release/web/public" ]; then
  BUILD_DIR="$APP_DIR/target/dx/soulbook_ui/release/web/public"
else
  BUILD_DIR="$(find "$APP_DIR/target/dx" -type d -path '*/release/web/public' | head -n 1 || true)"
fi

if [ -z "${BUILD_DIR:-}" ] || [ ! -d "$BUILD_DIR" ]; then
  echo "[$(date -Is)] could not locate Dioxus build output"
  exit 1
fi

BACKUP_DIR="/srv/soulbook-frontend.bak.$(date +%Y%m%d%H%M%S)"
if [ -d "$WEB_ROOT" ]; then
  cp -a "$WEB_ROOT" "$BACKUP_DIR"
  echo "[$(date -Is)] backed up frontend to $BACKUP_DIR"
fi

mkdir -p "$WEB_ROOT"
find "$WEB_ROOT" -mindepth 1 -maxdepth 1 -exec rm -rf {} +
cp -a "$BUILD_DIR"/. "$WEB_ROOT"/

nginx -t
systemctl reload nginx

if ! curl -fsS --max-time 15 http://127.0.0.1/docs/ >/dev/null; then
  echo "[$(date -Is)] frontend health check failed"
  if [ -d "$BACKUP_DIR" ]; then
    find "$WEB_ROOT" -mindepth 1 -maxdepth 1 -exec rm -rf {} +
    cp -a "$BACKUP_DIR"/. "$WEB_ROOT"/
    systemctl reload nginx
  fi
  exit 1
fi

echo "[$(date -Is)] frontend deployment succeeded: $OLD_REV -> $NEW_REV"
