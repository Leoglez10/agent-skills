#!/usr/bin/env bash
# Drives the real debug app with a QA script on a throwaway demo database; never touches real data.
#   pnpm --dir <frontend> dev &          # vite on :1420
#   <frontend>/qa/run.sh <frontend>/qa/<topic>.js [screenshot.png]
# Scripts report PASS/FAIL lines through the debug-only qa_log command; exits 1 on any FAIL or timeout.
# Adjust: BIN (debug binary path), APP_DB / APP_DEV_EVAL (env vars your app reads), demo_db example.
set -euo pipefail
QA=$(cd "$(dirname "$0")" && pwd); APP="$QA/../src-tauri"
BIN="$APP/../../target/debug/app-desktop"
DB=$(mktemp -d)
trap 'kill "${PID:-0}" 2>/dev/null || true; rm -rf "$DB"' EXIT
(cd "$APP" && cargo build -q && cargo run -q --example demo_db -- "$DB/app.db" >/dev/null)
# __QA__ in a script becomes this folder, so scripts can use qa/fixtures.
APP_DB="$DB/app.db" APP_DEV_EVAL="$(sed "s#__QA__#$QA#g" "$1")" "$BIN" >"$DB/app.log" 2>&1 &
PID=$!
for _ in $(seq 1 120); do grep -q "^QA DONE" "$DB/app.log" && break; sleep 1; done
if [[ -n "${2:-}" ]]; then screencapture -x -o -l"$(swift "$QA/winshot.swift" "$(basename "$BIN")")" "$2" || true; fi
grep "^QA " "$DB/app.log" | sed 's/^QA //'
grep -iE "panic|error" "$DB/app.log" | grep -v "^QA " || true
grep -q "^QA DONE" "$DB/app.log" || { echo "TIMEOUT: the script never finished"; exit 1; }
! grep -q "^QA FAIL" "$DB/app.log"
