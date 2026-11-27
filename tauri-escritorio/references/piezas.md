# Piezas de escritorio (tomadas de Finanzas)

## Bandeja (icono en la barra de menús)

- Título = dato vivo corto (saldo, racha). Menú: dato (deshabilitado), próximo pendiente (habilitado, abre su pantalla; "⚠︎" si está vencido), separador, Captura rápida (con atajo), Abrir, Salir (`CmdOrCtrl+Q`).
- `refresh(app)` crea o actualiza el icono; la UI llama `refresh_tray` después de cambiar datos para que nunca muestre un valor viejo.
- Se puede apagar en Ajustes › General; apagada, `remove_tray_by_id`.
- `show(app, Some("ruta"))` muestra la ventana y navega con `location.hash`.

## Captura rápida

- Ventana `quick`: sin bordes, siempre encima, fuera del dock, centrada, 640×236, `url: index.html#quick`.
- Atajo global (`Alt+Shift+F`) alterna mostrar/ocultar; si no se puede registrar, se registra en log.
- Al mostrar: `center`, `show`, `set_focus`, emitir `quick-open` para que limpie y enfoque.
- `Focused(false)` → ocultar (tipo Spotlight).
- Al guardar emite `quick-saved` a la ventana principal, que muestra el toast con deshacer y refresca.

## Ventana principal

- `CloseRequested` en `main` con bandeja activa → `prevent_close()` + `hide()`.
- Menús también funcionan con la ventana oculta: `dispatch` la muestra antes de emitir el comando.

## Avisos

- Hilo `spawn_reminders` que revisa pendientes a la hora configurada y manda notificación nativa.
- Ajustes › Avisos tiene "Probar aviso" (`test_notification`) para comprobar permisos.

## Actualizaciones y release

1. `pnpm tauri signer generate -w ~/.tauri/<app>.key`; la pública va en `plugins.updater.pubkey`, la privada en secretos de CI (`TAURI_SIGNING_PRIVATE_KEY`).
2. `plugins.updater.endpoints`: `https://github.com/<owner>/<repo>/releases/latest/download/latest.json`.
3. CI (tauri-action) compila por plataforma, firma y sube el `latest.json` a la release.
4. La UI: `checkUpdates(true)` al abrir; tarjeta en la sidebar; Ajustes › Actualizaciones para buscar a mano.
5. macOS: cask de Homebrew con `auto_updates true` (la app se actualiza sola; brew no la pisa).

## Perfiles

Una base SQLite por perfil en el directorio de datos; el perfil activo en los ajustes. El servidor MCP abre solo perfiles existentes (los crea la app).

## Reportes de fallas

Un Worker (Cloudflare) recibe `{kind, title, body, version, os}` y abre un issue en GitHub. Nunca se envían datos de la base. La app muestra qué se envía.

## Iconos y bundle

`icons/` con 32, 128, 128@2x, `.icns`, `.ico`. `bundle.category`, `shortDescription`, `macOS.minimumSystemVersion: "11.0"`, `mainBinaryName` en minúsculas. El binario MCP se empaqueta junto al ejecutable (`Contents/MacOS` en macOS).
