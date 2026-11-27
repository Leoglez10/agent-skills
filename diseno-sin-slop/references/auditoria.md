# Auditoría de slop

Correr desde la raíz del frontend (`SRC=src`). Cada conteo va al log del plan con archivo:línea.

```bash
SRC=src
rg -n "backdrop-filter|--glass" $SRC                         # glass: debe ser 0
rg -n "text-transform:\s*uppercase" $SRC                     # mayúsculas: 0
rg -n "letter-spacing:\s*\.?[0-9]" $SRC | rg -v mono          # tracking fuera de la mono
rg -on "border-radius:\s*[^;]+" $SRC | sort -t: -k3 -u        # radios distintos: solo tokens, 50%, 99px en switch/progreso
rg -n "border-radius:\s*(99|999|9999)px" $SRC                 # pastillas
rg -n "#[0-9a-fA-F]{3,8}\b|rgb\(" $SRC --glob '!**/tokens.css' --glob '!**/app.css'   # literales en componentes
rg -n "linear-gradient|radial-gradient" $SRC                  # degradados: 0
rg -n "box-shadow" $SRC | rg -v "shadow-float"                # sombras no flotantes
rg -n "sparkle|✨" $SRC                                         # ✨: 0
rg -n "font-size:\s*[0-9]" $SRC | rg -v "var\(--text"         # tamaños fuera de escala
rg -n "class=\"[^\"]*\bprimary\b" $SRC                        # revisar: una por pantalla
rg -n "transition|animation" $SRC                             # revisar duración < 200ms y propósito
```

Revisión a ojo (con capturas): métricas con icono en cuadro, lema bajo el logo, tarjetas apiladas con sombra, botones primarios repetidos por fila.

## Plan de migración (una tarea por commit)

| Tarea | Qué |
|---|---|
| T1 | `DESIGN.md` + tokens nuevos sin usar (ningún cambio visible) |
| T2 | Limpieza rápida: glass, mayúsculas, ✨, lema, radios → 2 |
| T3 | Fuentes incluidas (UI + mono) |
| T4 | Una acción primaria por pantalla |
| T5 | Plano y denso: sin sombras en tarjetas, tablas compactas |
| T6 | Métricas como tira: números grandes sin icono, celdas separadas por 1px |
| T7 | Componentes: sidebar → tablas → diálogos → ⌘K → gráficas; tamaños a la escala |
| T8 | Pantalla por pantalla |
| T9 | Harness de capturas claro/oscuro (puede adelantarse para revisar T7/T8) |

Cierre: auditoría en cero (salvo excepciones anotadas), todos los scripts de QA en PASS, chequeo de tipos sin errores, capturas revisadas.
