# El ciclo hasta la paridad

Así se rediseñó Finanzas (T0–T9, un commit por tarea, un solo PR final) y así se lleva una app nueva al nivel de la referencia.

## Preparación (una vez)

1. Inventario de la referencia (`app-escritorio-blueprint/references/inventario-finanzas.md`) → tabla de paridad `pieza | equivalente | estado | evidencia`.
2. Capturas de la referencia en claro y oscuro si está disponible (mismo `shots.sh` en su repo).
3. Plan `odd/tasks/<feature>.md`: encabezado (objetivo, rama, entrega, runner), `## Specs` con las frases exactas del usuario, `## Tasks` con una línea por tarea, `## Log` con la petición original como L1.

## Cada vuelta

```text
1. Elegir la siguiente tarea del plan (la brecha de paridad más visible).
2. Capturar ANTES:  qa/shots.sh /tmp/antes
3. Hacer el cambio mínimo de esa tarea (solo tokens; nada fuera de DESIGN.md).
4. Verificar:
   - chequeo de tipos + tests unitarios
   - scripts de QA del área tocada: qa/run.sh qa/<tema>.js
   - auditoría de slop (diseno-sin-slop/references/auditoria.md)
5. Capturar DESPUÉS: qa/shots.sh /tmp/despues  → comparar pantalla por pantalla, claro y oscuro.
   Opcional: montage /tmp/despues/*.png -tile 7x2 -geometry 640x+8+8 hoja.png (ImageMagick)
6. Si algo falla: arreglar dentro de la misma tarea. Un fallo nuevo descubierto fuera del alcance → anotarlo como tarea nueva, no arreglarlo a escondidas.
7. Commit de la tarea (Conventional Commits). Log: qué cambió, comandos corridos y resultado, hallazgos, siguiente tarea.
8. Actualizar la tabla de paridad.
```

## Qué mirar en las capturas

Una sola acción primaria visible · nada de glass, mayúsculas, pastillas ni sombras en tarjetas · cifras en mono alineadas · nada desbordado o cortado a 1280 y a 900 de ancho · contraste en oscuro · sidebar con el mismo orden que la referencia · estados vacíos con guía.

## Parar cuando

- Toda la tabla de paridad está en "hecho" o "descartado con motivo", y
- todos los scripts de QA dan PASS, la auditoría está en cero (con excepciones anotadas) y las capturas están revisadas.

Si una tarea falla dos veces por la misma causa, se detiene esa tarea, se anota la causa y se cambia de hipótesis; no se repite el mismo intento.

## Modo "sin parar"

Si el usuario pide terminar todo de corrido ("sigue con todos sin parar"), se recorre el plan completo con un commit por fase, arreglando lo que falle, y se reporta al final con la evidencia de cada fase. Push y PR siguen siendo decisión del usuario.
