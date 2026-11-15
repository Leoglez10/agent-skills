---
name: repo-presentacion
description: "Trigger: presentación del repo, mejorar repo, repo completo, About, topics, social preview, releases, community standards, perfil GitHub. Audita cómo se ve un repo en GitHub y entrega mejoras con comandos gh."
license: Apache-2.0
metadata:
  author: "Leoglez10"
  version: "1.0"
---

# Skill: Presentación del repositorio

## Activation Contract

Cargar cuando el usuario quiera mejorar cómo se ve o se descubre un repositorio en GitHub más allá del README: panel About, topics, imagen de vista previa, releases, archivos de comunidad, plantillas, demo visual o perfil.

Si solo pide el README, usar `readme-completo`. Si la auditoría detecta README ausente o pobre, recomendar `readme-completo` en vez de reescribirlo aquí.

## Hard Rules

- Auditar en modo lectura. No ejecutar `gh repo edit`, `gh release create`, ni crear archivos sin aprobación explícita del usuario.
- Cada propuesta sale de evidencia real del repo (manifests, stack, tags git, archivos existentes). Nada inventado.
- Topics: máximo 20, minúsculas, guiones en vez de espacios, derivados del stack y del propósito.
- Descripción About: una frase, máximo ~120 caracteres, dice qué hace y para quién. Sin emojis de relleno.
- Archivos de comunidad (`LICENSE`, `SECURITY.md`, etc.): no elegir licencia ni contacto de seguridad por el usuario; preguntar.
- Lo que GitHub no permite por API (social preview, repos fijados) se entrega como paso manual con la ruta exacta en la UI.

## Decision Gates

| Situación | Acción |
|---|---|
| Repo sin remoto GitHub o `gh` sin auth | Auditar solo archivos locales; marcar la parte remota como no verificada |
| Proyecto personal o demo pequeña | Priorizar About, topics, demo visual y social preview; omitir CODE_OF_CONDUCT y plantillas |
| Proyecto que acepta contribuciones | Incluir CONTRIBUTING, plantillas de issue/PR, SECURITY y CODE_OF_CONDUCT |
| Hay tags git o versión en manifest sin releases | Proponer release con `--generate-notes` |
| El usuario aprueba aplicar | Ejecutar solo los comandos aprobados y releer el estado con `gh repo view` |

## Execution Steps

1. Detectar `owner/repo` con `gh repo view --json nameWithOwner`. Si falla, ir al gate local.
2. Recolectar estado con los comandos de `references/audit-commands.md`.
3. Evaluar cada área con `references/presentation-checklist.md`.
4. Redactar propuestas concretas: texto exacto, set de topics, archivos a crear y comandos listos.
5. Presentar el reporte y esperar aprobación antes de aplicar.

## Output Contract

```text
Puntaje de presentación: N/10 (áreas OK / total)

Área | Estado | Evidencia | Propuesta
...

Comandos listos para aplicar (requieren tu aprobación):
gh repo edit --description "..." --homepage "..." --add-topic a,b,c
...

Pasos manuales (sin API):
- Social preview: Settings → General → Social preview → Edit
...

Siguiente paso recomendado: <una acción>
```

## References

- `references/audit-commands.md` — comandos `gh`/`git` de lectura para recolectar el estado.
- `references/presentation-checklist.md` — criterios por área y cómo proponer cada mejora.
- `references/topics-and-about.md` — reglas de topics, keywords del manifest y descripción About.
