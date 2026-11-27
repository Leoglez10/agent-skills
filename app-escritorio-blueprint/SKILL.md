---
name: app-escritorio-blueprint
description: "Trigger: app como Finanzas, nueva app de escritorio, blueprint de app, clonar la experiencia, app nativa multiplataforma. Orquesta las skills para construir una app con el diseño y UX de Finanzas."
license: Apache-2.0
metadata:
  author: "Leoglez10"
  version: "1.0"
---

# Skill: Blueprint de app de escritorio

## Activation Contract

Cargar cuando el usuario quiera crear una app nueva (aprendizaje, notas, hábitos, etc.) que se sienta y funcione como Finanzas: mismo diseño, sidebar, ajustes, ⌘K, atajos, deshacer, bandeja, actualizaciones y agentes MCP. Esta skill no escribe código sola: decide el orden y carga las skills hermanas.

## Hard Rules

- La referencia es `references/inventario-finanzas.md`. Cada pieza de ese inventario se traduce al dominio nuevo o se descarta con motivo escrito; nada se omite en silencio.
- No copiar nombres, marca, textos ni datos de Finanzas. Se copia la estructura y el criterio, no la identidad.
- Stack por defecto: Tauri 2 + Svelte 5 + Rust (núcleo en un crate aparte). Otro stack solo si el usuario lo pide; las reglas de diseño y UX se mantienen igual.
- Avanzar de poco en poco: una tarea por commit, con su verificación, en un plan `odd/tasks/<app>.md` o equivalente.
- Ninguna fase se marca hecha sin evidencia: tests en verde, script de QA con PASS o capturas revisadas.

## Decision Gates

| Situación | Acción |
|---|---|
| Proyecto vacío | Recorrer las fases 1 a 7 en orden |
| App existente que "se ve hecha por IA" | Empezar por `diseno-sin-slop` (auditoría) y luego `qa-visual-loop` |
| No hay Rust/Tauri | Aplicar solo `diseno-sin-slop`, `ux-shell-escritorio`, `registro-comandos` y `qa-visual-loop` adaptados |
| La app no maneja datos que un agente deba leer | Saltar `mcp-para-app` y anotarlo en el plan |
| Falta una decisión de producto (pantallas, entidades) | Preguntar una sola cosa y esperar |

## Execution Steps

1. Entrevista corta: dominio, entidades principales (equivalen a "cuentas"), 4 a 6 pantallas, acción principal del día a día (equivale a "Nuevo gasto"), si habrá varios perfiles.
2. Mapear el inventario: tabla `pieza de Finanzas → pieza de la app nueva` usando `references/inventario-finanzas.md`.
3. Escribir el plan con estas fases, una tarea por commit:
   1. Esqueleto Tauri + ventanas → `tauri-escritorio`
   2. Tokens, fuentes y `DESIGN.md` → `diseno-sin-slop`
   3. Shell: sidebar, rutas, diálogos, toast con deshacer → `ux-shell-escritorio`
   4. Registro de comandos, menú nativo, ⌘K, atajos → `registro-comandos`
   5. Pantallas del dominio y Ajustes por pestañas → `ux-shell-escritorio`
   6. Bandeja, captura rápida, avisos, actualizaciones → `tauri-escritorio`
   7. Servidor MCP → `mcp-para-app`
4. Desde la fase 3, instalar el harness de `qa-visual-loop` y correr el ciclo de capturas en cada tarea visual.
5. Al cerrar cada fase: correr la auditoría de `diseno-sin-slop` y comparar las capturas con la tabla de paridad.

## Output Contract

```text
Mapa de paridad: N piezas (traducidas / descartadas con motivo)
Plan: odd/tasks/<app>.md con T1..Tn (fase, skill, verificación)
Fase actual: <n> · Evidencia: <tests / QA PASS / capturas>
Siguiente tarea: <una>
```

## References

- `references/inventario-finanzas.md`: todo lo que tiene Finanzas (pantallas, sidebar, ajustes, comandos, atajos, ventanas), para mapearlo.
- Skills hermanas: `diseno-sin-slop`, `ux-shell-escritorio`, `registro-comandos`, `tauri-escritorio`, `mcp-para-app`, `qa-visual-loop`.
