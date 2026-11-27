---
name: ux-shell-escritorio
description: "Trigger: sidebar, barra lateral, ajustes, settings, diálogos, toast, deshacer, primeros pasos, UX de app de escritorio. Arma el shell y las interacciones de una app al estilo Finanzas."
license: Apache-2.0
metadata:
  author: "Leoglez10"
  version: "1.0"
---

# Skill: Shell y UX de app de escritorio

## Activation Contract

Cargar al construir o revisar la estructura de una app de escritorio: sidebar, rutas, páginas, Ajustes, diálogos, toasts, estados vacíos y textos. Usar con `diseno-sin-slop` (estilos) y `registro-comandos` (atajos y ⌘K).

## Hard Rules

- Sidebar en este orden: marca (texto) + plegar → "Buscar o ejecutar…" con ⌘K → navegación → secciones de entidades con valor en mono → "+ Nueva …" → espaciador → tarjeta de actualización → Ajustes + avatar. Detalle en `references/sidebar.md`.
- Plegable a iconos con ⌘S; se guarda en `localStorage`; menos de 760px se pliega sola. Texto plegado sigue como `title` y texto accesible.
- Página: título 24px, como máximo una acción primaria arriba a la derecha, tira de métricas, secciones divididas por líneas.
- Nunca preguntar "¿estás seguro?" para cambios reversibles: aplicar, mostrar toast con "Deshacer" (6 s) y apilar el revert para ⌘Z. Solo lo irreversible (borrar un perfil o base completa) confirma, nombrando lo que se pierde.
- Diálogos con `<dialog>` nativo y `showModal()`, abiertos desde un estado central (`assets/dialogs.ts`); Enter guarda, Esc cierra; no se apilan.
- Ajustes en pestañas con ruta propia (`ajustes/<pestaña>`), lista a la izquierda; ver `references/ajustes.md`.
- Textos en sentence case, verbos concretos, errores con la siguiente acción. Ver `references/interaccion.md`.
- Toda la UI con nombre accesible; foco visible; respeta `prefers-reduced-motion`.

## Decision Gates

| Situación | Acción |
|---|---|
| Entidades del dominio (cuentas, cursos, proyectos) | Sección propia en la sidebar con avatar, nombre, valor y menú contextual |
| Módulo que no todos usan | Apagado por defecto, se activa en Ajustes; su comando abre ese switch mientras está apagado |
| Varios espacios de datos (personal, trabajo) | Perfiles: una base por perfil, avatar en el pie de la sidebar |
| Pantalla vacía | Guía de primeros pasos que se marcan solos al cumplirse |
| Acción que se repite por fila | `.btn` o `.btn.ghost`, nunca primaria |

## Execution Steps

1. Copiar `assets/undo.ts` y `assets/dialogs.ts`; adaptar los tipos al dominio.
2. Montar el shell (`.app` = sidebar + `main`) con rutas por hash (`#resumen`, `#ajustes/general`).
3. Construir la sidebar siguiendo `references/sidebar.md`.
4. Crear cada pantalla con el patrón de página; la primera trae la guía de primeros pasos.
5. Crear Ajustes con las pestañas de `references/ajustes.md` que apliquen.
6. Repasar textos y estados con la lista de `references/interaccion.md`.
7. Verificar con un script de `qa-visual-loop` (plegar sidebar, deshacer, Esc en diálogo) y capturas claro/oscuro.

## Output Contract

```text
Shell: sidebar (secciones), rutas, páginas creadas
Ajustes: pestañas incluidas / descartadas con motivo
Deshacer: acciones cubiertas / pendientes
QA: scripts PASS · capturas revisadas
```

## References

- `references/sidebar.md`: anatomía, plegado, menú contextual, tarjeta de actualización.
- `references/ajustes.md`: pestañas, contenido y reglas de cada una.
- `references/interaccion.md`: deshacer, diálogos, textos, estados vacíos, accesibilidad.
- `assets/undo.ts`: toast + pila de deshacer.
- `assets/dialogs.ts`: estado central de diálogos.
