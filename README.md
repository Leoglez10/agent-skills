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
| [`readme-completo`](./readme-completo/SKILL.md) | 1.0 | Apache-2.0 | Inspecciona un repositorio real y crea o actualiza un `README.md` profesional, verificable y comprensible para usuarios no técnicos y desarrolladores. |

### readme-completo en detalle

Crea o mejora el README principal de cualquier proyecto después de **inspeccionar el repositorio real** (código, configuración, CI, lockfiles), no a partir de suposiciones.

Sus reglas centrales:

- **Principio cero — investigar antes de escribir**: si una afirmación técnica no puede verificarse en el repo, no se presenta como hecho.
- **Source of truth jerárquico**: `package.json` manda sobre una versión escrita a mano; un workflow activo manda sobre una guía vieja de releases.
- **Política contra documentación falsa**: nunca inventa comandos, puertos, variables de entorno, backups, releases ni licencias.
- **Adaptación por tipo de proyecto**: una librería no necesita manual de usuario; una app de escritorio sí necesita backups y troubleshooting.
- **Detección de contradicciones**: README dice React 18, `package.json` dice React 19 → gana el `package.json`.

Se activa con: "readme", "crear readme", "mejorar readme", "documentar proyecto", "hacer readme completo".

## Instalación

### Requisitos

- Un agente de código que soporte skills por carpeta (Claude Code, OpenCode, y otros compatibles con el formato `carpeta/SKILL.md`).

### Instalar una skill

Cloná el repo y copiá la carpeta de la skill que quieras a tu directorio de skills:

```bash
git clone https://github.com/Leoglez10/agent-skills.git
```

Luego copiá `readme-completo/` al directorio de skills de tu agente, por ejemplo:

| Agente | Directorio típico |
|---|---|
| OpenCode | `~/.config/opencode/skills/` |
| Claude Code | `~/.claude/skills/` |
| Genérico (agentes estándar) | `~/.agents/skills/` |

El agente detecta la skill automáticamente en la próxima sesión; no requiere configuración adicional.

## Estructura del repositorio

```text
agent-skills/
├── readme-completo/
│   └── SKILL.md        ← ⭐ La skill completa (sistema de documentación)
├── .atl/
│   └── skill-registry.md  ← Índice local auto-generado (no es contenido del repo)
└── README.md
```

> 💡 Regla de oro: una skill nueva = una carpeta nueva con su `SKILL.md` adentro.

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
