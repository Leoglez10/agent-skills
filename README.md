<div align="center">

# agent-skills

### Colección de skills para agentes de código con IA

Skills en formato `SKILL.md` (LLM-first) listas para instalar en Claude Code, OpenCode y cualquier agente que soporte el estándar de skills por carpeta.

</div>

> 🇬🇧 **In English:** A collection of LLM-first `SKILL.md` skills for coding agents such as Claude Code, OpenCode and Codex. No build step, no dependencies.

---

## ¿Qué es este repo?

Una colección de **skills**: instrucciones especializadas en Markdown con frontmatter que le enseñan a un agente de código (Claude Code, OpenCode, Codex, etc.) a ejecutar una tarea concreta con criterio profesional.

Cada skill vive en su propia carpeta y es autónoma: no hay build, no hay dependencias, no hay runtime. Copiás la carpeta y listo.

> 💡 Pensalo como un manual de instrucciones que se le inyecta al agente justo antes de hacer el trabajo. El agente no "adivina" cómo documentar un proyecto: lee la skill y sigue su sistema.

## Skills disponibles

| Skill | Versión | Licencia | Qué hace | Se activa cuando decís |
|---|---|---|---|---|
| [`readme-completo`](./readme-completo/SKILL.md) | 2.3 | Apache-2.0 | Inspecciona un repositorio real y crea o actualiza un `README.md` profesional, verificable y comprensible para usuarios no técnicos y desarrolladores. | `readme`, `crear readme`, `mejorar readme`, `documentar proyecto`, `documentación principal`, `actualizar readme`, `hacer readme completo` |
| [`repo-presentacion`](./repo-presentacion/SKILL.md) | 1.0 | Apache-2.0 | Audita cómo se ve un repositorio en GitHub más allá del README (About, topics, social preview, releases, archivos de comunidad) y entrega mejoras con comandos `gh` listos, sin aplicarlas sin tu aprobación. | `presentación del repo`, `mejorar repo`, `repo completo`, `About`, `topics`, `social preview`, `releases`, `community standards` |

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

> 💡 El patrón es reutilizable: si escribís una skill que crece más allá de unas cientos de líneas, separá el núcleo de lo especializado y documentá en el `SKILL.md` cuándo cargar cada pieza.

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

El CLI detecta tus agentes instalados y te pregunta a cuál instalar (o usa `-a` para elegirlo vos). Por defecto instala por symlink; agregá `--copy` si preferís una copia física.

#### Opción 2: manual

Cloná el repo:

```bash
git clone https://github.com/Leoglez10/agent-skills.git
```

Copiá la carpeta completa de la skill a tu directorio de skills:

```bash
cp -r agent-skills/readme-completo ~/.claude/skills/
```

> ⚠️ Copiá la carpeta entera, no solo el `SKILL.md`. Sin `references/` al lado, la skill queda incompleta.

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

Así editás el repo y el agente ve el cambio al instante.

> ⚠️ Con symlink, actualizar tu máquina y publicar en GitHub pasan a ser cosas separadas. La skill cambia apenas guardás; el commit sigue siendo tu responsabilidad.

## Estructura del repositorio

```text
agent-skills/
├── .github/                  ← Plantillas de issues y PRs, imagen de vista previa
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

Si nunca escribiste una, esto es lo que tenés que entender antes de tocar un solo archivo.

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
| "Conviene adaptar el resultado" | "Si detectás `SKILL.md`, cargá `references/skill-projects.md`" |
| "Podés revisar algunas cosas al final" | Una checklist con casillas concretas |

Imperativo, reglas explícitas, criterios de decisión y tablas de "si pasa X → hacé Y". Ambigüedad = resultado impredecible.

### Progressive disclosure: el contexto se paga

Todo lo que esté en el `SKILL.md` ocupa contexto **cada vez que la skill se activa**, use o no esa parte.

`readme-completo` empezó como un archivo de 2181 líneas. Documentar una librería cargaba igual las reglas de backups, de screenshots y de bases de datos: contexto quemado en conocimiento irrelevante para ese proyecto.

La solución fue partirla:

- **`SKILL.md`** → reglas que aplican siempre: qué no inventar, cómo clasificar el proyecto, el workflow, la validación.
- **`references/`** → conocimiento condicional, con una tabla en el `SKILL.md` que dice exactamente cuándo cargar cada archivo.

> 💡 Regla práctica: si una sección de tu skill empieza con "cuando el proyecto sea X...", esa sección es candidata a salir a `references/`.

### Cómo la probás

No hay tests unitarios acá. Se prueba activándola en un repositorio real y verificando el resultado contra los hechos: ¿los comandos existen?, ¿las versiones coinciden?, ¿los paths son reales?

Por eso `readme-completo` termina con una checklist de validación: es su forma de auto-verificarse antes de entregar.

## Agregar una skill nueva

1. Creá una carpeta con el nombre de la skill en kebab-case (`mi-skill/`).
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
4. Si la skill crece, movés lo especializado a `mi-skill/references/` y dejás en el `SKILL.md` una tabla que indique cuándo cargar cada archivo.

## Contribuir

Leé la [guía de contribución](./CONTRIBUTING.md) y el [código de conducta](./CODE_OF_CONDUCT.md). Para bugs o ideas, abrí un [issue](https://github.com/Leoglez10/agent-skills/issues/new/choose) con la plantilla.

## Licencia

[Apache-2.0](./LICENSE). Cada skill lo declara también en su frontmatter.

---

<div align="center">

### Creado por **[Leoglez10](https://github.com/Leoglez10)**

</div>
