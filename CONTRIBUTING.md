# Contribuir

Gracias por querer mejorar estas skills. Este repo no tiene build ni dependencias: cada skill es una carpeta con su `SKILL.md`.

## Antes de empezar

- Para un bug o una idea, abrí primero un [issue](https://github.com/Leoglez10/agent-skills/issues/new/choose).
- Para un cambio chico (typo, link roto), un Pull Request directo está bien.

## Flujo

1. Hacé fork y creá un branch: `git checkout -b feat/mi-cambio`.
2. Hacé el cambio en la skill correspondiente.
3. Probá la skill instalándola localmente:
   ```bash
   ln -s "$(pwd)/<skill>" ~/.claude/skills/<skill>
   ```
   y pedile al agente una tarea que la active.
4. Abrí el Pull Request usando la plantilla.

## Reglas para skills

- `description` en una sola línea, entre comillas, con las palabras de activación primero (máximo 250 caracteres).
- Frontmatter completo: `name`, `description`, `license`, `metadata.author`, `metadata.version`.
- `SKILL.md` corto e imperativo; el detalle va en `references/` y se carga bajo demanda.
- Subí `metadata.version` cuando cambie el comportamiento de la skill.
- Si agregás una skill nueva, registrala en la tabla y en el árbol del `README.md`.

## Commits

Usá [Conventional Commits](https://www.conventionalcommits.org/): `feat(readme-completo): ...`, `fix(repo-presentacion): ...`, `docs: ...`.

## Licencia

Al contribuir aceptás que tu aporte se publique bajo la licencia [Apache-2.0](./LICENSE) del repositorio.
