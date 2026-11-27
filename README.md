<div align="center">

# agent-skills

### Colección de skills para agentes de código con IA

Skills en formato `SKILL.md` (LLM-first) listas para instalar en Claude Code, OpenCode y cualquier agente que soporte el estándar de skills por carpeta.

</div>

> 🇬🇧 **In English:** A collection of LLM-first `SKILL.md` skills for coding agents such as Claude Code, OpenCode and Codex. No build step, no dependencies.

🌐 **Catálogo web:** [leoglez10.github.io/agent-skills](https://leoglez10.github.io/agent-skills/)

---

## ¿Qué es este repo?

Una colección de **skills**: instrucciones especializadas en Markdown con frontmatter que le enseñan a un agente de código (Claude Code, OpenCode, Codex, etc.) a ejecutar una tarea concreta con criterio profesional.

Cada skill vive en su propia carpeta y es autónoma: no hay build, no hay dependencias, no hay runtime. Copias la carpeta y listo.

> 💡 Piénsalo como un manual de instrucciones que se le inyecta al agente justo antes de hacer el trabajo. El agente no "adivina" cómo documentar un proyecto: lee la skill y sigue su sistema.

## Skills disponibles

| Skill | Versión | Licencia | Qué hace | Se activa cuando dices |
|---|---|---|---|---|
| [`readme-completo`](./readme-completo/SKILL.md) | 2.4 | Apache-2.0 | Inspecciona un repositorio real y crea o actualiza un `README.md` profesional, verificable y comprensible para usuarios no técnicos y desarrolladores. | `readme`, `crear readme`, `mejorar readme`, `documentar proyecto`, `documentación principal`, `actualizar readme`, `hacer readme completo` |
| [`repo-presentacion`](./repo-presentacion/SKILL.md) | 1.0 | Apache-2.0 | Audita cómo se ve un repositorio en GitHub más allá del README (About, topics, social preview, releases, archivos de comunidad) y entrega mejoras con comandos `gh` listos, sin aplicarlas sin tu aprobación. | `presentación del repo`, `mejorar repo`, `repo completo`, `About`, `topics`, `social preview`, `releases`, `community standards` |

### Paquete: apps de escritorio al estilo Finanzas

Siete skills que capturan el diseño, la UX y la arquitectura de [Finanzas](https://github.com/Leoglez10/finanzas) para construir otras apps con el mismo nivel. `app-escritorio-blueprint` es la entrada: mapea Finanzas al dominio nuevo y aplica las demás en orden, con un ciclo de capturas que se repite hasta llegar a la paridad.

| Skill | Versión | Qué hace | Se activa cuando dices |
|---|---|---|---|
| [`app-escritorio-blueprint`](./app-escritorio-blueprint/SKILL.md) | 1.0 | Orquesta el paquete: inventario de Finanzas, mapa de paridad y plan por fases | `app como Finanzas`, `nueva app de escritorio`, `blueprint de app` |
| [`diseno-sin-slop`](./diseno-sin-slop/SKILL.md) | 1.0 | Tokens, `DESIGN.md`, lista de prohibidos y auditoría contra el look "hecho por IA" | `diseño muy IA`, `quitar AI slop`, `DESIGN.md`, `design tokens` |
| [`ux-shell-escritorio`](./ux-shell-escritorio/SKILL.md) | 1.0 | Sidebar, Ajustes por pestañas, diálogos, toast con deshacer, textos y estados vacíos | `sidebar`, `ajustes`, `diálogos`, `deshacer` |
| [`registro-comandos`](./registro-comandos/SKILL.md) | 1.0 | Un registro de comandos para menú nativo, ⌘K y tabla de atajos; ⌘ ↔ Ctrl | `paleta de comandos`, `⌘K`, `atajos`, `menú nativo` |
| [`tauri-escritorio`](./tauri-escritorio/SKILL.md) | 1.0 | Tauri 2: ventanas, bandeja, captura rápida, atajo global, avisos, actualizaciones firmadas | `Tauri`, `bandeja`, `captura rápida`, `updater` |
| [`mcp-para-app`](./mcp-para-app/SKILL.md) | 1.0 | Servidor MCP en Rust que comparte el núcleo de la app, con estado "conectado" en Ajustes | `servidor MCP`, `agentes IA`, `rmcp` |
| [`qa-visual-loop`](./qa-visual-loop/SKILL.md) | 1.0 | Maneja la app real con scripts PASS/FAIL y captura cada pantalla en claro y oscuro | `QA visual`, `capturas claro y oscuro`, `paridad` |

#### Empezar una app nueva con el paquete

Abre tu agente dentro de la carpeta del proyecto nuevo y pega este prompt. Cambia la última línea por la descripción de tu app. El agente instala las 7 skills, las lee y te acompaña fase por fase hasta que la app tenga lo mismo que Finanzas.

```text
Quiero crear una app de escritorio nueva con el mismo diseño, UX y funcionamiento que Finanzas, usando el paquete de skills de Leoglez10/agent-skills.

1. Instala las skills del paquete:
   npx skills add Leoglez10/agent-skills --skill app-escritorio-blueprint diseno-sin-slop ux-shell-escritorio registro-comandos tauri-escritorio mcp-para-app qa-visual-loop -y
   Si npx no está disponible, clona https://github.com/Leoglez10/agent-skills y copia esas 7 carpetas completas a tu directorio de skills.
2. Si no ves las skills en esta sesión, lee los SKILL.md directamente desde donde se instalaron. Empieza por app-escritorio-blueprint y su references/inventario-finanzas.md.
3. Hazme la entrevista corta de esa skill, una pregunta a la vez, y espera mis respuestas.
4. Arma el mapa de paridad (pieza de Finanzas → pieza de mi app) y el plan por fases en odd/tasks/[nombre-de-la-app].md.
5. Avanza fase por fase cargando la skill que indica cada una. Un commit por tarea, con su verificación (tests, QA en PASS o capturas).
6. Desde la fase 3, corre el ciclo de qa-visual-loop en cada cambio visual y repite hasta que la tabla de paridad esté completa.
7. No hagas push ni abras PR sin preguntarme.

Mi app: [qué hace, para quién y sus pantallas principales]
```

> 💡 Las skills se cargan al iniciar la sesión del agente. Por eso el paso 2: en la misma sesión donde se instalan, el agente las lee como archivos. En la siguiente sesión se activan solas con sus triggers.

### readme-completo en detalle

Crea o mejora el README principal de cualquier proyecto después de **inspeccionar el repositorio real** (código, configuración, CI, lockfiles), no a partir de suposiciones.

Sus reglas centrales:

- **Principio cero — investigar antes de escribir**: si una afirmación técnica no puede verificarse en el repo, no se presenta como hecho.
- **Source of truth jerárquico**: `package.json` manda sobre una versión escrita a mano; un workflow activo manda sobre una guía vieja de releases.
- **Política contra documentación falsa**: nunca inventa comandos, puertos, variables de entorno, backups, releases ni licencias.
- **Adaptación por tipo de proyecto**: una librería no necesita manual de usuario; una app de escritorio sí necesita backups y troubleshooting.
- **Detección de contradicciones**: README dice React 18, `package.json` dice React 19 → gana el `package.json`.
- **Badges verificables**: cada badge debe respaldarse en un dato comprobable (workflow real, release real, licencia existente). Un badge sin respaldo es una afirmación falsa.
- **Badge ≠ tag**: los badges viven en el README; los topics de GitHub y las keywords del manifest son configuración del repositorio y se entregan como propuesta aparte.
- **Créditos obligatorios por ascendencia**: si el proyecto es un fork, está basado en otro repo, es un port o reutiliza código de terceros, el README debe declarar origen, autor, qué cambió y licencia del origen. `—desde la v2.2—`
- **Matriz de secciones por tipo de proyecto**: detecta si el repo es desktop, web, móvil, API, CLI, librería, infraestructura o skill, y decide qué secciones incluir. Casi todas van por defecto; solo omite una cuando el repositorio demuestra que no aplica, y lo reporta. Incluye flujos por rol, flujos complejos a fondo, respaldo en la nube, trabajo en varias computadoras, guía de GitHub para principiantes y un bloque para IAs. `—desde la v2.4—`

Tipos de proyecto que reconoce: aplicaciones web, desktop y móviles, CLIs, librerías, APIs, backends, monorepos, infraestructura, proyectos educativos y —desde la v2.1— **skills de agente, colecciones de skills y plugins**. Este README fue generado con ella.

## Arquitectura: progressive disclosure

Desde la v2.0, `readme-completo` no carga todo su contenido de una vez.

El problema: una skill monolítica ocupa contexto del agente **siempre**, incluso las partes que ese proyecto no necesita. Documentar una librería no requiere las reglas de backups; documentar una CLI no requiere las de screenshots.

La solución: `SKILL.md` contiene solo el núcleo — reglas fundamentales, clasificación del proyecto, workflow y validación. El conocimiento especializado vive en `references/` y se carga **bajo demanda**.

| Antes (v1.0) | Ahora (v2.2) |
|---|---|
| 1 archivo, 2181 líneas | `SKILL.md` de 575 líneas + 8 referencias |
| 33 KB siempre en contexto | ~15 KB siempre + solo lo que el proyecto pida |

Las referencias y cuándo se consultan:

| Referencia | Se carga cuando |
|---|---|
| [`repository-inspection.md`](./readme-completo/references/repository-inspection.md) | Hay que saber qué archivos leer, detectar el root real o resolver contradicciones |
| [`readme-structure.md`](./readme-completo/references/readme-structure.md) | Ya se clasificó el proyecto y toca elegir secciones |
| [`user-documentation.md`](./readme-completo/references/user-documentation.md) | El proyecto tiene usuarios finales o interfaz gráfica |
| [`developer-documentation.md`](./readme-completo/references/developer-documentation.md) | El repo va a ser usado o modificado por desarrolladores |
| [`skill-projects.md`](./readme-completo/references/skill-projects.md) | El repo **es** una skill, colección de skills o plugin de agente |
| [`data-and-security.md`](./readme-completo/references/data-and-security.md) | Hay datos, base de datos, autenticación, secretos o backups |
| [`badges-and-credits.md`](./readme-completo/references/badges-and-credits.md) | El README va a llevar badges, o deriva de otro repositorio (fork, plantilla, port, código vendorizado) |
| [`writing-style.md`](./readme-completo/references/writing-style.md) | Se está en la redacción final |

> 💡 El patrón es reutilizable: si escribes una skill que crece más allá de unas cientos de líneas, separa el núcleo de lo especializado y documenta en el `SKILL.md` cuándo cargar cada pieza.

## Instalación

### Requisitos

- Un agente de código que soporte skills por carpeta (Claude Code, OpenCode, y otros compatibles con el formato `carpeta/SKILL.md`).

### Instalar una skill

#### Opción 1: con `npx` (recomendada)

Con el CLI abierto de [skills](https://github.com/vercel-labs/skills) (soporta Claude Code, OpenCode, Codex, Cursor y más), un solo comando clona, detecta y instala:

```bash
npx skills add Leoglez10/agent-skills
```

Variantes útiles:

```bash
# Ver qué skills hay sin instalar nada
npx skills add Leoglez10/agent-skills --list

# Instalar solo readme-completo, a un agente específico, sin prompts
npx skills add Leoglez10/agent-skills --skill readme-completo -g -a claude-code -y
```

El CLI detecta tus agentes instalados y te pregunta a cuál instalar (o usa `-a` para elegirlo tú). Por defecto instala por symlink; agrega `--copy` si prefieres una copia física.

#### Opción 2: manual

Clona el repo:

```bash
git clone https://github.com/Leoglez10/agent-skills.git
```

Copia la carpeta completa de la skill a tu directorio de skills:

```bash
cp -r agent-skills/readme-completo ~/.claude/skills/
```

> ⚠️ Copia la carpeta entera, no solo el `SKILL.md`. Sin `references/` al lado, la skill queda incompleta.

Directorios según el agente:

| Agente | Directorio típico |
|---|---|
| OpenCode | `~/.config/opencode/skills/` |
| Claude Code | `~/.claude/skills/` |
| Genérico (agentes estándar) | `~/.agents/skills/` |

El agente detecta la skill automáticamente en la próxima sesión; no requiere configuración adicional.

### Instalar para desarrollo

Si vas a **editar** la skill, un symlink evita mantener dos copias desincronizadas:

```bash
ln -s "$(pwd)/agent-skills/readme-completo" ~/.claude/skills/readme-completo
```

Así editas el repo y el agente ve el cambio al instante.

> ⚠️ Con symlink, actualizar tu máquina y publicar en GitHub pasan a ser cosas separadas. La skill cambia apenas guardas; el commit sigue siendo tu responsabilidad.

## Estructura del repositorio

```text
agent-skills/
├── .github/                  ← Plantillas de issues y PRs, imagen de vista previa
├── docs/                     ← Catálogo web publicado con GitHub Pages
├── readme-completo/
│   ├── SKILL.md              ← ⭐ Núcleo: reglas, workflow, validación
│   └── references/           ← Conocimiento especializado, carga bajo demanda
│       ├── repository-inspection.md
│       ├── readme-structure.md
│       ├── user-documentation.md
│       ├── developer-documentation.md
│       ├── skill-projects.md
│       ├── data-and-security.md
│       ├── badges-and-credits.md
│       └── writing-style.md
├── repo-presentacion/
│   ├── SKILL.md              ← Auditoría de la presentación en GitHub
│   └── references/
│       ├── audit-commands.md
│       ├── presentation-checklist.md
│       └── topics-and-about.md
├── app-escritorio-blueprint/ ← Entrada del paquete de apps de escritorio
│   └── references/inventario-finanzas.md
├── diseno-sin-slop/          ← references/ (investigación, auditoría) + assets/ (tokens.css, DESIGN.template.md)
├── ux-shell-escritorio/      ← references/ (sidebar, ajustes, interacción) + assets/ (undo.ts, dialogs.ts)
├── registro-comandos/        ← assets/ (commands.ts, keys.ts + test, menu.rs)
├── tauri-escritorio/         ← references/piezas.md + assets/ (tauri.conf.json, lib.rs, tray.rs, updates.ts)
├── mcp-para-app/             ← references/agentes.md + assets/ (main.rs, tools.rs)
├── qa-visual-loop/           ← references/ciclo.md + assets/ (run.sh, shots.sh, shots.js, qa-template.js, winshot.swift)
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── LICENSE                   ← Apache-2.0
└── README.md
```

> 💡 Regla de oro:
> - Skill nueva → carpeta nueva con su `SKILL.md`
> - Reglas centrales de una skill → su `SKILL.md`
> - Conocimiento que solo aplica a veces → su `references/`

## Anatomía de una skill

Si nunca escribiste una, esto es lo que tienes que entender antes de tocar un solo archivo.

### El frontmatter es un clasificador, no metadata decorativa

De toda la skill, el agente ve **siempre** dos campos: `name` y `description`. Nada más.

```yaml
---
name: readme-completo
description: "Trigger: readme, README, crear readme... Inspecciona profundamente un repositorio y crea o actualiza un README profesional..."
---
```

El `description` es lo que decide si la skill se activa o se queda dormida. Por eso arranca listando triggers literales: son las palabras que el agente va a matchear contra lo que escribe el usuario. Un `description` vago es una skill que nunca se usa.

### El cuerpo se escribe para el modelo, no para una persona

Esto es lo que más cuesta al principio. El `SKILL.md` no es un tutorial ni un artículo de blog: son **instrucciones ejecutables por un LLM**.

| Prosa para humanos | Instrucción para el modelo |
|---|---|
| "Es importante verificar la información" | "NUNCA inventes comandos, puertos ni variables de entorno" |
| "Conviene adaptar el resultado" | "Si detectas `SKILL.md`, carga `references/skill-projects.md`" |
| "Puedes revisar algunas cosas al final" | Una checklist con casillas concretas |

Imperativo, reglas explícitas, criterios de decisión y tablas de "si pasa X → haz Y". Ambigüedad = resultado impredecible.

### Progressive disclosure: el contexto se paga

Todo lo que esté en el `SKILL.md` ocupa contexto **cada vez que la skill se activa**, use o no esa parte.

`readme-completo` empezó como un archivo de 2181 líneas. Documentar una librería cargaba igual las reglas de backups, de screenshots y de bases de datos: contexto quemado en conocimiento irrelevante para ese proyecto.

La solución fue partirla:

- **`SKILL.md`** → reglas que aplican siempre: qué no inventar, cómo clasificar el proyecto, el workflow, la validación.
- **`references/`** → conocimiento condicional, con una tabla en el `SKILL.md` que dice exactamente cuándo cargar cada archivo.

> 💡 Regla práctica: si una sección de tu skill empieza con "cuando el proyecto sea X...", esa sección es candidata a salir a `references/`.

### Cómo la pruebas

No hay tests unitarios aquí. Se prueba activándola en un repositorio real y verificando el resultado contra los hechos: ¿los comandos existen?, ¿las versiones coinciden?, ¿los paths son reales?

Por eso `readme-completo` termina con una checklist de validación: es su forma de auto-verificarse antes de entregar.

## Agregar una skill nueva

1. Crea una carpeta con el nombre de la skill en kebab-case (`mi-skill/`).
2. Adentro, un `SKILL.md` con frontmatter válido:

```yaml
---
name: mi-skill
description: "Trigger: qué la activa. Qué hace la skill."
license: Apache-2.0
metadata:
  author: "tu-usuario"
  version: "1.0"
---
```

3. El cuerpo del documento describe el sistema de trabajo de la skill: propósito, reglas, flujo y criterios de validación.
4. Si la skill crece, mueves lo especializado a `mi-skill/references/` y dejas en el `SKILL.md` una tabla que indique cuándo cargar cada archivo.

## Contribuir

Lee la [guía de contribución](./CONTRIBUTING.md) y el [código de conducta](./CODE_OF_CONDUCT.md). Para bugs o ideas, abre un [issue](https://github.com/Leoglez10/agent-skills/issues/new/choose) con la plantilla.

## Licencia

[Apache-2.0](./LICENSE). Cada skill lo declara también en su frontmatter.

---

<div align="center">

### Creado por **[Leoglez10](https://github.com/Leoglez10)**

</div>
