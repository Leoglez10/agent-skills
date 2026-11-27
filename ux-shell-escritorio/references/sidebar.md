# Sidebar

Ancho 252px abierta, 62px plegada. 62 centra los iconos exactamente donde estaban: al plegar solo se mueve el borde. Fondo `--sidebar`, borde derecho 1px. Padding superior 44px para dejar sitio a los semáforos de macOS (barra de título overlay); la zona de la marca lleva `data-tauri-drag-region`.

## Orden

| # | Elemento | Regla |
|---|---|---|
| 1 | Marca | Solo el nombre en la fuente display, 24px. Sin logo en caja, sin lema |
| 2 | Botón plegar | Pegado al borde derecho; `aria-expanded`; title con el atajo (`keyLabel('⌘S')`). Oculto en ventanas angostas |
| 3 | "Buscar o ejecutar…" | Abre ⌘K; `kbd` con el atajo a la derecha |
| 4 | Navegación | 5–6 botones `.nav` con icono; `aria-current="page"` en la activa; contador rojo si hay pendientes vencidos |
| 5 | Sección de entidades | Título en sentence case (`.section`), cada fila: avatar/logo 22px, nombre con elipsis, valor en `.num` muted (rojo si negativo) |
| 6 | "+ Nueva …" | `.nav.add` en color de marca |
| 7 | Sección "Cerradas/Archivadas" | Solo si hay elementos |
| 8 | Espaciador | `flex: 1` |
| 9 | Tarjeta de actualización | Estados: disponible (`.btn.sm` "Instalar actualización"), descargando (barra con %), lista ("Reiniciar …"). Plegada: un solo icono en color de marca |
| 10 | Pie | Ajustes + avatar del perfil (abre `ajustes/perfil`) |

## Plegado

- Estado en `localStorage` (`sidebar-collapsed`), alternado por el botón y por el comando `sidebar` (⌘S).
- `matchMedia("(width < 760px)")` pliega sola sin cambiar la preferencia guardada.
- Solo anima el ancho (`grid-template-columns .28s`); los textos se desvanecen en su lugar (opacidad .12s) y el borde los tapa como una cortina.
- Cada fila conserva su texto en el DOM; `title` se pone solo cuando está plegada.
- Contadores pasan a insignia en la esquina del icono.

## Menú contextual en entidades

Clic derecho: "Abrir", "Apariencia…" (icono, color, imagen propia), y acciones del tipo (por ejemplo "Datos de la tarjeta…"). El menú es el mismo helper para toda la app (`showMenu(event, [[label, fn], …])`).

## Verificación (script de QA)

El botón pliega a menos de 70px · la preferencia se guarda · los iconos conservan nombre accesible y tooltip · títulos de sección y marca con opacidad 0 · iconos centrados (±2px) · los iconos plegados navegan · el comando vuelve a abrirla al ancho original.
