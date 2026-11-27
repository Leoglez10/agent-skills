# Agentes: configuración y estado conectado

## Configuración por agente (la ruta es la real del binario junto al ejecutable)

| Agente | Instrucción en la UI | Configuración |
|---|---|---|
| Claude Code | "Ejecuta este comando en una terminal y reinicia Claude Code." | `claude mcp add --scope user <app> -- "<ruta>/<app>-mcp"` |
| Codex | "Pega esto en ~/.codex/config.toml y reinicia Codex." | `[mcp_servers.<app>]` + `command = '<ruta>'` (literal TOML: las barras de Windows no se escapan) |
| OpenCode | "Pega esto en ~/.config/opencode/opencode.json (o une la sección "mcp" con la tuya) y reinicia OpenCode." | `{"mcp": {"<app>": {"type": "local", "command": ["<ruta>"]}}}` |

Ruta del binario desde la app: `std::env::current_exe()?.parent()?.join(format!("<app>-mcp{}", std::env::consts::EXE_SUFFIX))` (en macOS cae en `Contents/MacOS`).

## Latido (para mostrar "conectado")

- Directorio `mcp-clients/` junto a la base de datos.
- Al iniciar, el servidor lee `peer_info().client_info.name`, lo mapea a un agente conocido (`name.contains(slug)`) y escribe `mcp-clients/<slug>` con su PID; repite cada `BEAT` (por ejemplo 60 s) en un `tokio::spawn`.
- La app considera conectado si la fecha de modificación tiene menos de `STALE` (por ejemplo 3 min).
- Tests: configuraciones apuntan al comando (incluida ruta Windows en JSON) y el latido caduca.

## Pantalla Ajustes › Agentes IA

Por agente: nombre, estado (chip `ok` "Conectado" o muted "Sin conectar"), instrucción, bloque de configuración con botón copiar. Ningún dato del usuario sale de la máquina: el servidor corre local por stdio.
