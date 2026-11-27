# Investigación: cómo se ve una app de escritorio buena y qué delata a la IA

Fuente: rediseño de Finanzas (rama `feat/redesign-no-slop`, PR #26, noviembre 2026).

## Evidencia

| Fuente | Hallazgo | Regla que sale |
|---|---|---|
| Adrian Krebs, estudio de Show HN (https://www.adriankrebs.ch/blog/design-slop/) | 23.5% usa shadcn sin personalizar, 28% usa degradados | Nada de componentes por defecto sin tokens propios; sin degradados |
| Reglas públicas "avoid-ai-design" y "no-slop-ui" | Glass, mayúsculas con tracking, ✨, tarjetas de métricas con icono, pastillas, sombras por todos lados | Lista de prohibidos |
| README de OpenCode Desktop | "built with Electron" | El aspecto viene del diseño, no del framework: no hace falta reescribir en otro stack |
| PLAN.md de Rune (Anomaly) | "Feel is the product", "Same pixels on every platform"; el tema de OpenCode es el de referencia | Fuentes incluidas, un solo tema por tokens, igual en todo SO |
| AGENTS.md de Hex | Solo tokens de tema, una acción Primary por pantalla, sentence case, snapshots claro/oscuro | Una acción primaria; capturas claro/oscuro en cada cambio |

## Decisiones de stack

- Se evaluaron Rune y GPUI: Rune es pre-release y macOS-first, GPUI es pre-1.0; ambos implican reescribir la UI. Se quedó Tauri + Svelte.
- "No tan nativa": una app multiplataforma debe verse igual en todos lados. Por eso fuentes propias y no `system-ui`; la única concesión nativa es la paleta "Sistema" que usa el color de acento del SO.

## Auditoría inicial de Finanzas (para comparar)

Fuente del sistema, sin degradados (bien); `--glass` + `backdrop-filter` en 3 sitios; 5 reglas uppercase; 11 radios distintos; 8 radios pastilla; métricas con icono en cuadro; lema bajo el logo; ✨ en actualizaciones.

## Resultado

Marca solo texto (Funnel Display), UI en Funnel Sans variable 300–800, cifras en DM Mono 400/500 (el archivo 500 se declara 500–900 para que el negrito nunca se sintetice, `letter-spacing: -.03em`). Cuerpo 13px, título 24px. Tablas densas: `th` 8/12, `td` 6/12. Tarjetas con borde, sin sombra; padding 16.

## Lo que aprendimos al migrar

- Un `button.link` global puede pisar `.kpi`: revisar especificidad cuando una celda clicable pierde fondo.
- Leyendas de gráficas se desbordan en anchos medios: que envuelvan bajo la gráfica (`flex-basis`) con elipsis.
- Selectores de QA no deben depender de `.primary`: cambia al degradar botones.
