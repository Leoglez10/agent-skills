---
name: qa-visual-loop
description: "Trigger: QA visual, capturas claro y oscuro, harness de QA, probar la app real, ciclo hasta paridad, comparar con referencia. Maneja la app real y repite tareas hasta igualar la referencia."
license: Apache-2.0
metadata:
  author: "Leoglez10"
  version: "1.0"
---

# Skill: Ciclo de QA visual hasta paridad

## Activation Contract

Cargar al verificar cambios de UI en una app de escritorio Tauri, al capturar todas las pantallas en claro y oscuro, o al iterar una app nueva hasta que se parezca a una referencia (por ejemplo Finanzas). Lo usan `app-escritorio-blueprint` y `diseno-sin-slop` en cada tarea visual.

## Hard Rules

- Probar la app real (build debug), nunca solo el navegador: una variable de entorno inyecta un script tras cargar la página y los comandos `qa_log`/`qa_sleep` (solo debug) reportan `PASS`/`FAIL`/`DONE`.
- Siempre sobre una base de datos temporal de demo (`mktemp -d`); nunca datos reales.
- Cada script de QA comprueba el comportamiento observable (DOM, `localStorage`, ruta, medidas) y termina con `DONE`; un `FAIL` o timeout sale con código 1.
- Los selectores no dependen de clases de estilo que cambian (`.primary`); usar roles, `title`, `aria-*` o texto.
- Leer el estado de la app después de que monte (esperar antes de tocar `window.__app`).
- Matar instancias debug viejas antes de correr si la app es de instancia única.
- Una tarea por commit; ninguna se marca hecha sin evidencia en el log del plan.

## Decision Gates

| Situación | Acción |
|---|---|
| Cambio de comportamiento | Script en `qa/<tema>.js` con `assets/qa-template.js` + `run.sh` |
| Cambio visual | `shots.sh <dir>` antes y después; revisar claro y oscuro |
| Igualar una referencia | Correr el ciclo de `references/ciclo.md` hasta que la tabla de paridad esté completa |
| Windows/Linux | `screencapture` es solo macOS: capturar con la herramienta del SO o desde la webview; los scripts PASS/FAIL funcionan igual |
| Se repite el mismo fallo dos veces | Parar, escribir la causa en el log y cambiar de hipótesis |

## Execution Steps

1. Agregar `qa_log`, `qa_sleep` y el hook `APP_DEV_EVAL` (ver `tauri-escritorio/assets/lib.rs`) y un ejemplo `demo_db` que cree datos de demo.
2. Copiar `assets/run.sh`, `assets/shots.sh`, `assets/shots.js`, `assets/winshot.swift` a `<frontend>/qa/`; ajustar nombre del binario, variables y pantallas.
3. Un script por tema con `assets/qa-template.js`.
4. Correr el ciclo de `references/ciclo.md`.
5. Al cerrar: todos los scripts PASS, chequeo de tipos y tests en verde, hoja de capturas revisada.

## Output Contract

```text
Scripts QA: N PASS / M FAIL (lista de FAIL con mensaje)
Capturas: <dir> con <pantalla>-<light|dark>.png · revisadas: sí/no
Paridad: X/Y piezas del inventario · siguiente brecha: <una>
```

## References

- `references/ciclo.md`: el ciclo repetible (auditar, planear, tarea, verificar, comparar) y criterios de parada.
- `assets/run.sh`: corre un script de QA contra la app real.
- `assets/shots.sh` + `assets/shots.js`: captura cada pantalla en claro y oscuro.
- `assets/qa-template.js`: esqueleto de un script con `ok()` que reintenta.
- `assets/winshot.swift`: id de la ventana para `screencapture -l` (macOS).
