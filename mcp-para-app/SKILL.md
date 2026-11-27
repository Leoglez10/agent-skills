---
name: mcp-para-app
description: "Trigger: servidor MCP, MCP de mi app, agentes IA, rmcp, conectar Claude/Codex/OpenCode a la app. Expone los datos de una app de escritorio a agentes con un servidor MCP en Rust."
license: Apache-2.0
metadata:
  author: "Leoglez10"
  version: "1.0"
---

# Skill: Servidor MCP para la app

## Activation Contract

Cargar cuando una app de escritorio (Rust/Tauri) deba poder ser leída y operada por agentes (Claude Code, Codex, OpenCode) con un servidor MCP por stdio que comparte el núcleo de la app.

## Hard Rules

- El servidor es un binario aparte (`crates/<app>-mcp`) que usa el mismo crate de dominio que la app. Ninguna regla de negocio se duplica.
- Una tool por cosa que la app hace (cobertura casi total), con nombres `verbo_objeto` en inglés (`list_items`, `add_item`, `delete_item`).
- Cada tool declara `read_only` y `destructive` en sus annotations; borrar es `destructive: true`.
- Descripciones con unidades, formatos (`YYYY-MM-DD`) y efectos secundarios ("Returns the deleted item").
- `instructions` del servidor: el modelo mental del dominio y el flujo correcto para tareas de varios pasos.
- Si hay perfiles: argumento opcional `perfil` en todas las tools; solo se abren perfiles existentes (los crea la app).
- Errores como `CallToolResult::error` con texto legible, nunca panic.
- La app muestra en Ajustes › Agentes IA si cada agente está conectado (latido en archivo) y su configuración copiable con la ruta real del binario.

## Decision Gates

| Situación | Acción |
|---|---|
| Lectura de datos | Tool `read_only: true` |
| Cambio reversible | Tool normal; describir cómo revertir con otra tool |
| Borrado | `destructive: true` y devolver lo borrado |
| Varios pasos típicos (crear cuenta + estilo) | Explicar el flujo en `instructions` del servidor |
| App sin Rust | Mismo diseño con el SDK MCP del lenguaje; mantener annotations y latido |

## Execution Steps

1. Crear el crate con `rmcp = { version = "3", features = ["server", "transport-io"] }` y `tokio`.
2. Copiar `assets/main.rs` y `assets/tools.rs`; escribir specs y `call` por tool sobre el núcleo.
3. Test por tool en `crates/<app>-mcp/tests` con una base temporal.
4. Empaquetar el binario junto al ejecutable de la app (`bundle.externalBin` o copia en el build).
5. Ajustes › Agentes IA: usar `references/agentes.md` para configuración y latido.
6. Probar en un agente real: listar tools, una lectura, una escritura, un deshacer.

## Output Contract

```text
Tools: N (solo lectura: R, destructivas: D) · cobertura vs. acciones de la app: X/Y
Tests: PASS · Binario empaquetado: sí/no
Agentes configurables desde Ajustes: Claude Code, Codex, OpenCode
```

## References

- `references/agentes.md`: configuración por agente, latido y pantalla de Ajustes.
- `assets/main.rs`: servidor rmcp con perfiles y latido.
- `assets/tools.rs`: forma de `ToolSpec`, `specs()` y `call()`.
