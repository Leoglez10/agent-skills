# Reference — Comandos de auditoría

Todos son de solo lectura. Reemplaza `OWNER/REPO`.

## Panel About

```bash
gh repo view OWNER/REPO --json description,homepageUrl,repositoryTopics,licenseInfo,usesCustomOpenGraphImage,openGraphImageUrl
```

- `description` vacío o genérico → proponer.
- `homepageUrl` vacío y hay GitHub Pages, deploy o docs → proponer link.
- `repositoryTopics` vacío → proponer set.
- `usesCustomOpenGraphImage: false` → falta social preview.

## Community Standards

```bash
gh api repos/OWNER/REPO/community/profile --jq '{health: .health_percentage, files: .files | map_values(. != null)}'
```

Detecta `readme`, `license`, `contributing`, `code_of_conduct`, `issue_template`, `pull_request_template`. `SECURITY.md` no sale aquí:

```bash
gh api repos/OWNER/REPO/contents/SECURITY.md --silent && echo yes || ls .github/SECURITY.md SECURITY.md docs/SECURITY.md 2>/dev/null
```

## Releases y versiones

```bash
gh release list -R OWNER/REPO --limit 5
git tag --sort=-v:refname | head -5
```

Comparar con la versión del manifest (`package.json`, `pyproject.toml`, `Cargo.toml`, frontmatter `metadata.version`).

## Demo visual

```bash
grep -nE '!\[|<img|\.gif|\.mp4|\.webm' README.md
ls docs/ assets/ .github/ 2>/dev/null | grep -iE 'png|jpg|gif|svg|webp|mp4'
```

## Perfil del dueño

```bash
gh api graphql -f query='{ user(login:"OWNER") { pinnedItems(first:6){ nodes { ... on Repository { name } } } } }'
gh repo view OWNER/OWNER --json name 2>/dev/null   # README de perfil
```

Solo aplica si el usuario quiere mejorar también su perfil.
