# Reference — Proyectos de skills y plugins de agente

Consultar cuando el repositorio ES una skill, una colección de skills, o un
plugin para un agente (Claude Code, Cursor, Copilot, agentes MCP, etc.).

Estos repos NO son aplicaciones ni librerías: no se "ejecutan", se **instalan en
un agente** y se **activan por contexto**. Documentarlos como si fueran una app
web produce un README falso (comandos que no existen, `npm start` inventado,
puertos imaginarios).


# Detección

Es un proyecto de skills si encuentras cualquiera de estos:

```text
SKILL.md                      (en el root o en subdirectorios)
skills/<nombre>/SKILL.md
.claude/skills/
agents/*.md                   (subagentes con frontmatter)
commands/*.md                 (slash commands)
hooks/                        (hooks del agente)
plugin.json
.claude-plugin/marketplace.json
AGENTS.md / CLAUDE.md como artefacto publicado, no como config privada
```

Señal fuerte: un archivo `.md` cuyo frontmatter YAML tiene `name` + `description`
y cuyo cuerpo son instrucciones dirigidas al modelo, no a una persona.

Subtipos:

| Subtipo | Señal |
|---|---|
| Skill única | un solo `SKILL.md` en el root |
| Colección de skills | varios directorios, cada uno con su `SKILL.md` |
| Plugin | `plugin.json` o `.claude-plugin/`, agrupa skills + agentes + comandos + hooks |
| Marketplace | `marketplace.json` que lista plugins de terceros |


# Inventario previo a escribir

Para CADA skill del repositorio, lee su frontmatter y extrae literalmente:

* `name`
* `description` (contiene los triggers reales — no los inventes ni los parafrasees)
* `version`, `license`, `author` si existen
* `allowed-tools` / restricciones de herramientas si existen

Y del cuerpo:

* qué hace realmente;
* si usa progressive disclosure (`references/`, `scripts/`, `assets/`);
* tamaño (líneas de `SKILL.md`) — relevante porque el `SKILL.md` se carga en contexto;
* dependencias externas (CLIs, binarios, MCP servers) que la skill asume instalados.

Verifica cada dependencia antes de listarla como requisito.


# Instalación — describe SOLO el método real

Los métodos posibles son distintos entre sí. Detecta cuál aplica en ESTE repo:

| Método | Cuándo | Qué documentar |
|---|---|---|
| Copia manual | no hay instalador | `cp -r <skill> ~/.claude/skills/` y advertir que no se auto-actualiza |
| Symlink | el repo se mantiene con git | `ln -s $(pwd)/<skill> ~/.claude/skills/<skill>` — se actualiza con `git pull` |
| Plugin marketplace | existe `.claude-plugin/marketplace.json` | el comando real de instalación del plugin |
| Instalador propio | existe `install.sh` / `Makefile` | leer el script y documentar lo que hace de verdad |

Documenta también:

* ruta de instalación por scope (usuario vs proyecto) si el agente lo distingue;
* cómo verificar que quedó instalada;
* cómo desinstalar;
* si `SKILL.md` se recarga en caliente o requiere reiniciar la sesión.

NUNCA inventes un comando de instalación. Si no hay instalador, decilo.


# Estructura de README recomendada

Base sobre `readme-structure.md`, con estos ajustes propios del tipo:

1. **Qué es** — una skill/colección de skills, para qué agente, en una frase.
2. **Cómo funciona una skill** (breve) — el agente lee el `description`, decide
   si aplica, y carga las instrucciones. El usuario no la "ejecuta".
3. **Catálogo** — tabla con una fila por skill:

   | Skill | Qué hace | Se activa cuando |
   |---|---|---|

   La columna de activación sale del `description`, textual.
4. **Instalación** — el método real, verificado.
5. **Uso** — cómo se dispara: frase natural, `/comando`, o automático por contexto.
   Incluí un ejemplo de invocación real.
6. **Arquitectura** — árbol del repo y, si aplica, explicación de progressive
   disclosure: qué vive en `SKILL.md` y qué en `references/`, y por qué
   (presupuesto de contexto).
7. **Anatomía de una skill** — sección didáctica; ver abajo.
8. **Cómo crear la tuya** — pasos concretos para extender el repo.
9. **Convenciones y contribución** — naming, formato del frontmatter, dónde va cada cosa.
10. **Limitaciones** — qué no hace, dependencias externas, agentes no soportados.


# Sección didáctica — anatomía de una skill

Este tipo de repo suele existir para que otros aprendan a escribir skills.
Cuando el repo tenga esa intención (colección pública, ejemplos, docs de
convenciones), incluí una sección que explique:

* **Frontmatter**: `name` y `description` son lo único que el agente ve siempre.
  El `description` es el clasificador: ahí van los triggers.
* **Cuerpo**: instrucciones para el modelo, no prosa para humanos. Imperativo,
  reglas explícitas, criterios de decisión.
* **Progressive disclosure**: `SKILL.md` liviano + `references/` cargadas bajo
  demanda. Explicá el costo: todo lo que está en `SKILL.md` ocupa contexto en
  cada activación.
* **Scripts y assets**: cuándo conviene código determinista en vez de instrucciones.
* **Cómo probarla**: activarla en un repo real y verificar el resultado.

Usá los ejemplos REALES del repositorio, no ejemplos genéricos. Si una skill del
repo usa progressive disclosure, mostrala como caso.


# Errores frecuentes a evitar

* Documentar `npm install` / `npm start` en un repo que solo tiene markdown.
* Inventar triggers en vez de citar el `description`.
* Prometer compatibilidad con agentes que el repo nunca menciona.
* Poner badges de CI/build cuando no hay workflows.
* Describir la skill como "ejecutable" — se activa, no se corre.
* Copiar la lista de skills sin verificar que cada directorio exista de verdad.


# Checklist específica

* [ ] ¿Cada skill listada existe como directorio con su `SKILL.md`?
* [ ] ¿Los triggers salen del `description` real?
* [ ] ¿El método de instalación fue verificado?
* [ ] ¿Se aclara la diferencia entre copia y symlink respecto a actualizaciones?
* [ ] ¿El árbol del repo coincide con el filesystem?
* [ ] ¿Las versiones del README coinciden con el frontmatter de cada skill?
* [ ] ¿Se explica cómo agregar una skill nueva?
* [ ] ¿Se nombra el agente destino y su versión mínima si aplica?
