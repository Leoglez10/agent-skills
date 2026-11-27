---
name: tauri-escritorio
description: "Trigger: Tauri, app de escritorio, bandeja, tray, captura rápida, atajo global, actualizaciones, updater, notificaciones. Arma una app Tauri 2 multiplataforma con la base de Finanzas."
license: Apache-2.0
metadata:
  author: "Leoglez10"
  version: "1.0"
---

# Skill: App de escritorio con Tauri 2

## Activation Contract

Cargar al crear o extender una app Tauri 2 (Rust + webview, frontend Svelte/Vite) que deba funcionar como app de escritorio completa: ventanas, menú, bandeja, captura rápida, avisos, inicio automático y actualizaciones firmadas.

## Hard Rules

- Lógica de dominio en un crate Rust aparte (`crates/<app>-core`) que comparten la app y el servidor MCP. Los comandos Tauri son delgados.
- Ventanas declaradas en `tauri.conf.json` con `"create": false` y construidas en `setup`, para poder ajustar opciones en debug (QA sin throttling).
- Ventana principal: `titleBarStyle: "Overlay"`, `hiddenTitle`, semáforos dentro de la sidebar, mínimo 900×600.
- Con bandeja activa, cerrar la ventana solo la oculta (avisos siguen llegando). Salir es explícito.
- Errores de bandeja, menú y atajos se registran, nunca tumban la app.
- Actualizaciones: `tauri-plugin-updater` con `latest.json` firmado en GitHub Releases. La revisión al abrir falla en silencio; la que pide la persona muestra el error.
- Comandos solo de depuración (`qa_log`, `qa_sleep`, evaluación de script) bajo `#[cfg(debug_assertions)]`.
- Datos del usuario en el directorio de datos del SO; QA y capturas usan siempre una base temporal.

## Decision Gates

| Necesidad | Pieza |
|---|---|
| Registrar algo desde cualquier app | Ventana `quick` + atajo global (`tauri-plugin-global-shortcut`) |
| Dato vivo siempre visible | Icono en barra de menús con título y menú (`tray.rs`) |
| Recordatorios | `tauri-plugin-notification` + hilo de avisos |
| Abrir al iniciar sesión | `tauri-plugin-autostart` (LaunchAgent en macOS) |
| Distribución macOS | Cask de Homebrew con `auto_updates true` |
| Una sola instancia | Plugin single-instance; matar instancias debug viejas antes de QA |

## Execution Steps

1. Crear la app (`pnpm create tauri-app`, Svelte + TS) y mover el dominio a `crates/<app>-core`.
2. Copiar `assets/tauri.conf.json` (ventanas main y quick) y ajustar nombre e identificador.
3. Copiar `assets/lib.rs` (setup: ventanas, menú, atajo, bandeja, eventos de ventana, hook de QA).
4. Agregar menú con `registro-comandos` y bandeja según `references/piezas.md`.
5. Agregar `assets/updates.ts` y la tarjeta de actualización en la sidebar.
6. Configurar firma y release siguiendo `references/piezas.md` (sección actualizaciones).
7. `cargo check`, `pnpm build`, y un script de `qa-visual-loop` que abra la app real.

## Output Contract

```text
Ventanas: main, quick (sí/no) · Menú: sí · Bandeja: sí/no · Atajo global: <tecla>
Plugins: updater, process, notification, autostart, global-shortcut, dialog, opener
Build: cargo check OK · pnpm build OK · QA de arranque PASS
```

## References

- `references/piezas.md`: bandeja, captura rápida, avisos, actualizaciones y release, perfiles.
- `assets/tauri.conf.json`: ventanas y bundle.
- `assets/lib.rs`: builder con plugins, setup y eventos (usa `menu.rs` de `registro-comandos` y `assets/tray.rs`).
- `assets/tray.rs`: icono de la barra de menús con dato vivo.
- `assets/updates.ts`: estado de actualización para la UI.
