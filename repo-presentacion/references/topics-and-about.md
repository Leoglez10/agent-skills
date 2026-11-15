# Reference — Topics, keywords y descripción About

Consultar al evaluar las áreas 1 (About) y 3 (Topics) del checklist.

## Diferencia clave

| Elemento | Dónde vive | Para qué sirve |
|---|---|---|
| Badge | README | informar visualmente dentro de la página |
| Topic | Configuración del repo (GitHub) | descubrimiento en búsquedas y topic pages |
| Keyword del manifest | `package.json`, `pyproject.toml`, etc. | indexación de registros y tooling |
| Descripción "About" | Configuración del repo | primera línea que ve un buscador |

Los tres últimos son "tags" de la página. No los confundas con badges y no
declares que los pusiste si no los pusiste: son ajustes del repositorio, no del
Markdown.

## Reglas de GitHub Topics

- máximo 20 topics por repo;
- minúsculas;
- sin espacios: usar `-`;
- sin `#`;
- nombres reales del dominio, no sinónimos acumulados;
- derivados del stack y del propósito real.

Cómo obtenerlos:

1. `keywords` del manifest, si existen;
2. dependencias y stack reales (`package.json`, `pyproject.toml`, `Cargo.toml`,
   `go.mod`);
3. tipo de proyecto (cli, desktop-app, api, monorepo, agent-skill);
4. dominio funcional real (loans, invoicing, backups…), solo si el proyecto lo hace.

Ejemplo de set razonable para una app de escritorio con Tauri y SQLite:

```text
tauri, rust, sqlite, desktop-app, windows, offline-first
```

Ejemplo de lo que NO se hace:

```text
hacktoberfest, awesome, ai, blockchain, web3, python
```

si nada de eso describe el proyecto.

## Keywords del manifest

Si el proyecto publica un paquete, verifica que `keywords` (o el campo equivalente)
exista y coincida con los topics. Si no existe y el proyecto se publica, puedes
proponerlo como ajuste aparte; NO lo inventes dentro del README como si ya
estuviera.

## Descripción de la página

La descripción "About" del repo debe ser una frase concreta, no marketing.
Si está vacía o es genérica, señálalo como pendiente en el reporte final; no la
inventes en el README.
