<div align="center">

# leo-agent-skills

### Colección de skills para agentes de código con IA

Skills en formato `SKILL.md` (LLM-first) listas para instalar en Claude Code, OpenCode y cualquier agente que soporte el estándar de skills por carpeta.

</div>

---

## ¿Qué es este repo?

Una colección de **skills**: instrucciones especializadas en Markdown con frontmatter que le enseñan a un agente de código (Claude Code, OpenCode, Codex, etc.) a ejecutar una tarea concreta con criterio profesional.

Cada skill vive en su propia carpeta y es autónoma: no hay build, no hay dependencias, no hay runtime. Copiás la carpeta y listo.

> 💡 Pensalo como un manual de instrucciones que se le inyecta al agente justo antes de hacer el trabajo. El agente no "adivina" cómo documentar un proyecto: lee la skill y sigue su sistema.

## Skills disponibles

| Skill | Versión | Licencia | Qué hace |
|---|---|---|---|
| [`readme-completo`](./readme-completo/SKILL.md) | 2.0 | Apache-2.0 | Inspecciona un repositorio real y crea o actualiza un `README.md` profesional, verificable y comprensible para usuarios no técnicos y desarrolladores. |

### readme-completo en detalle

Crea o mejora el README principal de cualquier proyecto después de **inspeccionar el repositorio real** (código, configuración, CI, lockfiles), no a partir de suposiciones.

Sus reglas centrales:

- **Principio cero — investigar antes de escribir**: si una afirmación técnica no puede verificarse en el repo, no se presenta como hecho.
- **Source of truth jerárquico**: `package.json` manda sobre una versión escrita a mano; un workflow activo manda sobre una guía vieja de releases.
- **Política contra documentación falsa**: nunca inventa comandos, puertos, variables de entorno, backups, releases ni licencias.
- **Adaptación por tipo de proyecto**: una librería no necesita manual de usuario; una app de escritorio sí necesita backups y troubleshooting.
- **Detección de contradicciones**: README dice React 18, `package.json` dice React 19 → gana el `package.json`.

Se activa con: "readme", "crear readme", "mejorar readme", "documentar proyecto", "hacer readme completo".

## Arquitectura: progressive disclosure

Desde la v2.0, `readme-completo` no carga todo su contenido de una vez.

El problema: una skill monolítica ocupa contexto del agente **siempre**, incluso las partes que ese proyecto no necesita. Documentar una librería no requiere las reglas de backups; documentar una CLI no requiere las de screenshots.

La solución: `SKILL.md` contiene solo el núcleo — reglas fundamentales, clasificación del proyecto, workflow y validación. El conocimiento especializado vive en `references/` y se carga **bajo demanda**.

| Antes (v1.0) | Después (v2.0) |
|---|---|
| 1 archivo, 2181 líneas | `SKILL.md` de 524 líneas + 6 referencias |
| 33 KB siempre en contexto | 13 KB siempre + lo que el proyecto pida |

Las referencias y cuándo se consultan:

| Referencia | Se carga cuando |
|---|---|
| [`repository-inspection.md`](./readme-completo/references/repository-inspection.md) | Hay que saber qué archivos leer, detectar el root real o resolver contradicciones |
| [`readme-structure.md`](./readme-completo/references/readme-structure.md) | Ya se clasificó el proyecto y toca elegir secciones |
| [`user-documentation.md`](./readme-completo/references/user-documentation.md) | El proyecto tiene usuarios finales o interfaz gráfica |
| [`developer-documentation.md`](./readme-completo/references/developer-documentation.md) | El repo va a ser usado o modificado por desarrolladores |
| [`data-and-security.md`](./readme-completo/references/data-and-security.md) | Hay datos, base de datos, autenticación, secretos o backups |
| [`writing-style.md`](./readme-completo/references/writing-style.md) | Se está en la redacción final |

> 💡 El patrón es reutilizable: si escribís una skill que crece más allá de unas cientos de líneas, separá el núcleo de lo especializado y documentá en el `SKILL.md` cuándo cargar cada pieza.

## Instalación

### Requisitos

- Un agente de código que soporte skills por carpeta (Claude Code, OpenCode, y otros compatibles con el formato `carpeta/SKILL.md`).

### Instalar una skill

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
├── readme-completo/
│   ├── SKILL.md              ← ⭐ Núcleo: reglas, workflow, validación
│   └── references/           ← Conocimiento especializado, carga bajo demanda
│       ├── repository-inspection.md
│       ├── readme-structure.md
│       ├── user-documentation.md
│       ├── developer-documentation.md
│       ├── data-and-security.md
│       └── writing-style.md
├── .atl/
│   └── skill-registry.md     ← Índice local auto-generado (no es contenido del repo)
└── README.md
```

> 💡 Regla de oro:
> - Skill nueva → carpeta nueva con su `SKILL.md`
> - Reglas centrales de una skill → su `SKILL.md`
> - Conocimiento que solo aplica a veces → su `references/`

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

1. Fork o clone del repo.
2. Creá un branch.
3. Agregá o mejorá una skill.
4. Abrí un Pull Request.

## Licencia

La skill `readme-completo` declara **Apache-2.0** en su frontmatter.

> ⚠️ El repositorio todavía no incluye un archivo `LICENSE` a nivel raíz. Si vas a redistribuir las skills, agregalo.

---

<div align="center">

### Creado por **[Leoglez10](https://github.com/Leoglez10)**

</div>
