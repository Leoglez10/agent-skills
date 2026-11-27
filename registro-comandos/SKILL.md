---
name: registro-comandos
description: "Trigger: paleta de comandos, ⌘K, command palette, atajos de teclado, menú nativo, shortcuts. Un solo registro de comandos para menú, ⌘K y la tabla de atajos, con Ctrl en Windows/Linux."
license: Apache-2.0
metadata:
  author: "Leoglez10"
  version: "1.0"
---

# Skill: Registro único de comandos

## Activation Contract

Cargar al agregar comandos, atajos, paleta ⌘K o menú nativo a una app de escritorio (Tauri u otra webview). Cada acción de la app existe una sola vez en un registro, y de ahí salen el menú, la paleta y la pantalla de atajos.

## Hard Rules

- Un registro (`COMMANDS`) con `id`, `label`, `group`, `icon`, `keys`, `run`. El menú nativo usa los mismos `id`.
- Atajos escritos una vez con símbolos Mac (`⌘K`, `⇧⌘N`); en Windows/Linux se muestran y se comparan como `Ctrl+…` con `keyLabel`/`comboOf`.
- En macOS el menú nativo es dueño de los atajos; en Windows/Linux los corre el `keydown` de la webview. Evitar doble ejecución si ambos disparan (ventana de 500ms por `id`).
- Deshacer, rehacer, cortar, copiar, pegar y seleccionar todo se quedan nativos para que los campos de texto funcionen; el ⌘Z de la app solo corre fuera de campos y sin diálogo abierto.
- Un comando con un diálogo abierto no hace nada, salvo la paleta, que cierra el diálogo y se abre.
- La pantalla de atajos se genera del registro; nunca se escribe a mano.
- Labels en sentence case con verbo ("Ir a Resumen", "Nuevo gasto", "Importar…").
- Módulo apagado: su comando lleva a la pestaña de Ajustes que lo activa.

## Decision Gates

| Situación | Acción |
|---|---|
| Acción frecuente | Atajo `⌘<letra>` y entrada en el menú Archivo/Ver |
| Pantalla principal 1..5 | `⌘1`..`⌘5` en el menú Ver |
| Acción rara | Sin atajo; solo en ⌘K |
| Atajo global (desde otras apps) | Plugin global-shortcut, solo uno o dos; ver `tauri-escritorio` |
| Sin Tauri (web pura) | Omitir el menú nativo; el `keydown` corre en todas las plataformas |

## Execution Steps

1. Copiar `assets/keys.ts` y su test `assets/keys.test.ts`; correr el test.
2. Copiar `assets/commands.ts` y reemplazar los comandos por los del dominio.
3. Copiar `assets/menu.rs` en `src-tauri/src/`, repetir los mismos `id` con `CmdOrCtrl+…`; en `setup`: `app.set_menu(menu::build(..))` y `app.on_menu_event(|app, e| menu::dispatch(app, e.id().as_ref()))`.
4. En el componente raíz: `listen("menu", …)` + `keydown` de `assets/commands.ts` (sección App).
5. Paleta ⌘K: lista filtrada de `COMMANDS` con grupo y atajo; luego entidades y frases del dominio (ver `ux-shell-escritorio/references/interaccion.md`).
6. Ajustes › Atajos: `SHORTCUTS` generado.
7. Verificar con QA: cada atajo dispara su comando, la paleta filtra, el menú llega a la ventana.

## Output Contract

```text
Comandos: N (con atajo: M) · grupos: Navegar, Acciones, Editar, Ver, Ventana
Menú nativo: ids alineados (sí/no, lista de diferencias)
Tests keys: PASS · QA comandos: PASS
```

## References

- `assets/commands.ts`: registro, `runCommand`, sidebar plegable, manejo en el componente raíz.
- `assets/keys.ts` / `assets/keys.test.ts`: símbolos Mac ↔ Ctrl.
- `assets/menu.rs`: menú nativo Tauri 2 con los mismos ids.
