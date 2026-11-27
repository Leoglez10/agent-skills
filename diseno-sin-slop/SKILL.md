---
name: diseno-sin-slop
description: "Trigger: diseño muy IA, quitar AI slop, DESIGN.md, design tokens, rediseño, estilo OpenCode/Hex. Define y audita un sistema de diseño de app de escritorio sin señales de IA."
license: Apache-2.0
metadata:
  author: "Leoglez10"
  version: "1.0"
---

# Skill: Diseño sin slop

## Activation Contract

Cargar al crear el sistema visual de una app de escritorio o web-app, al escribir su `DESIGN.md`, o cuando alguien dice que la app "se ve hecha por IA". Dirección: herramienta seria (OpenCode, Hex, Linear), mismos píxeles en macOS, Windows y Linux.

## Hard Rules

- Colores solo por roles nombrados (`--canvas`, `--surface`, `--ink`, `--muted`, `--brand`…). Ningún hex/rgb en componentes. Sin degradados.
- Dos radios: `--r-control` 6px y `--r-panel` 10px. Círculo solo para avatares y puntos; extremos redondos solo en switches y barras de progreso.
- Espaciado en rejilla de 4px. Cuatro tamaños de texto: 12, 13, 16, 24.
- Fuentes incluidas en el bundle (nunca `system-ui` como cara principal): una sans de UI + una mono con `tabular-nums` para cifras.
- Relleno `--brand` para una sola acción principal por pantalla (más el submit del diálogo abierto).
- Secciones separadas por línea de 1px, no por tarjetas con sombra. Sombra solo en capas flotantes.
- Movimiento solo para explicar un cambio, menos de 200ms.
- Prohibido: glass/`backdrop-filter`, etiquetas en MAYÚSCULAS con tracking, ✨, lema bajo el logo, métricas con icono en cuadro teñido, pastillas como forma por defecto, sombras y brillos decorativos.
- Frases en sentence case en toda la UI.

## Decision Gates

| Situación | Acción |
|---|---|
| Proyecto nuevo | Copiar `assets/tokens.css` y `assets/DESIGN.template.md`, ajustar marca y paletas |
| App existente | Correr la auditoría de `references/auditoria.md` antes de tocar nada; plan por tareas T1..T9 |
| Color con significado (éxito, error, aviso, categoría) | Fijo en todas las paletas; solo superficies y `--brand` cambian por tema |
| Cifra "display" (hero 40+px) | Única excepción permitida a la escala de 4 tamaños; anotarla |
| Blanco sobre color (avatar, knob) | Literal permitido; anotarlo en el log |

## Execution Steps

1. Leer `references/investigacion.md` para fijar dirección y argumentos.
2. Instalar tokens y fuentes (`assets/tokens.css`), con `light-dark()` y `data-theme` para forzar modo.
3. Escribir `DESIGN.md` desde la plantilla, en la raíz del frontend.
4. Migrar pantalla por pantalla en este orden: limpieza rápida → fuentes → una acción primaria → plano y denso → métricas en tira → componentes (sidebar, tablas, diálogos, ⌘K, gráficas) → pantallas.
5. Tras cada tarea: correr `references/auditoria.md` y las capturas de `qa-visual-loop` en claro y oscuro.

## Output Contract

```text
Auditoría: N hallazgos (glass, mayúsculas, radios distintos, pastillas, literales, sombras…) con archivo:línea
Tokens: instalados / faltantes
DESIGN.md: creado / actualizado
Capturas antes/después: claro y oscuro de cada pantalla tocada
```

## References

- `references/investigacion.md`: evidencia (estudio Krebs, reglas de Hex y Rune, OpenCode) y por qué cada regla.
- `references/auditoria.md`: comandos grep para contar señales de slop y plan de migración.
- `assets/tokens.css`: tokens, fuentes, 6 paletas, botones, tira de métricas, diálogo, toast.
- `assets/DESIGN.template.md`: plantilla del `DESIGN.md` del proyecto.
