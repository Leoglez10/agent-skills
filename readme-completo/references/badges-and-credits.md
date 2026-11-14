# Reference — Badges, tags y créditos

Consultar cuando el README vaya a llevar badges, cuando el proyecto necesite tags
de descubrimiento (GitHub Topics), o cuando el proyecto derive de otro repositorio
(fork, plantilla, port, código vendorizado, adaptación) o reutilice código de
terceros.

Este archivo NO se consulta para un README mínimo sin badges ni ascendencia externa.

---

# Principio común

> Un badge y un tag son afirmaciones verificables.

Si no puedes comprobar el dato detrás del badge o del tag, no va.

- Badge de CI → debe existir el workflow.
- Badge de versión → debe existir el release o el manifest.
- Badge de licencia → debe existir `LICENSE`.
- Topic `rust` → el proyecto debe estar escrito en Rust.
- Crédito a un repo → debe existir ese repo y la relación debe ser real.

Un badge roto o un topic mentiroso daña más que su ausencia.

---

# 1. Badges

## Qué es un badge

Un badge es una imagen dinámica o estática que declara un hecho del proyecto.
Se renderiza desde un URL, no se sube como asset.

## Estructura

```markdown
[![texto alternativo](URL_DEL_BADGE)](URL_DESTINO)
```

Reglas:

- el badge siempre se envuelve en un link que lleve al dato real (workflow,
  release, licencia);
- el texto alternativo describe el dato, no el color;
- nunca dejes un badge sin destino salvo que no exista destino posible.

## Badge de CI — usa el nativo de GitHub Actions

Es el único patrón que no depende de un tercero:

```text
https://github.com/OWNER/REPO/actions/workflows/<archivo>.yml/badge.svg
https://github.com/OWNER/REPO/actions/workflows/<archivo>.yml/badge.svg?branch=main
```

Verifica antes:

1. que el archivo de workflow exista en `.github/workflows/`;
2. el nombre exacto del archivo;
3. la rama por defecto real (`git symbolic-ref refs/remotes/origin/HEAD` o la
   configuración del repo).

NO pongas badge de build si no hay workflow. NO pongas badge "passing" escrito a
mano.

## Badges de shields.io

Sintaxis general de un badge estático:

```text
https://img.shields.io/badge/<ETIQUETA>-<MENSAJE>-<COLOR>
```

Escapado dentro de la ruta:

| Carácter | Reemplazo |
|---|---|
| espacio | `_` |
| `-` | `--` |
| `_` | `__` |

Endpoints dinámicos habituales (formas aproximadas; si un badge no renderiza,
confirma el parámetro exacto en https://shields.io antes de publicarlo):

```text
https://img.shields.io/github/v/release/OWNER/REPO
https://img.shields.io/github/license/OWNER/REPO
https://img.shields.io/npm/v/PAQUETE
https://img.shields.io/pypi/v/PAQUETE
https://img.shields.io/crates/v/CRATE
```

Parámetros útiles:

- `?label=` para renombrar;
- `?color=` para el color;
- `?logo=<slug>&logoColor=white` para el logo del stack (slugs de simple-icons);
- `?branch=` para fijar la rama.

## Qué badges Sí valen

- estado de CI real;
- última versión / release;
- licencia (si existe el archivo);
- plataformas realmente soportadas (las que declara el instalador o el manifest);
- lenguaje y runtime principales;
- cobertura **solo** si existe un reporte real y accesible;
- descargas o instalaciones **solo** si el paquete existe en ese registro.

## Qué badges NO valen

- "Build passing" sin CI;
- cobertura inventada o desactualizada;
- licencia que el repo no tiene;
- plataformas que nunca se probaron;
- badges decorativos que no informan nada ("Made with ❤", "Awesome");
- cinco badges del mismo dato repetido;
- badges de servicios muertos o de tu propio perfil.

## Cuántos

- 4–10 en el hero;
- el resto, dentro de la sección que corresponde (stack en desarrollo, licencia
  en licencia).

El hero es una tarjeta de identidad, no un tablero.

---

# 2. Tags de la página (GitHub Topics y keywords)

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

---

# 3. Créditos y agradecimientos

## Cuándo esta sección es obligatoria

- el proyecto **es un fork**;
- el proyecto **está basado en** otro repositorio, plantilla, scaffold o boilerplate;
- el proyecto es un **port o adaptación** de otro proyecto;
- se incorporó **código vendorizado** o copiado de terceros;
- se reutilizó de forma sustancial **una skill, un prompt o una arquitectura** de otro proyecto;
- el funcionamiento central depende de un proyecto de terceros que conviene nombrar.

## Cuándo es opcional

- agradecimientos a librerías de uso normal (el `package.json` ya lo dice);
- inspiración general sin código reutilizado.

En ese caso basta un agradecimiento breve, sin convertirlo en un inventario de
dependencias.

## Elementos obligatorios cuando hay ascendencia

1. **Qué se tomó**: código, diseño, plantilla, arquitectura, documentación, skill.
2. **De dónde**: nombre del proyecto y link al repositorio o sitio original.
3. **Quién lo hizo**: autor original, si está declarado.
4. **Qué cambió este repo**: fork con modificaciones, reescritura, port a otro stack.
5. **Licencia del origen**: nombre y compatibilidad con la licencia actual.

Sin estos cinco elementos la sección es decorativa, no cumple su función.

## Reglas legales mínimas

- No borres ni reescribas avisos de copyright del origen.
- Conserva el archivo `LICENSE`, `NOTICE` o equivalente que la licencia original exija.
- Si el origen es Apache-2.0, el `NOTICE` se preserva y se documenta el cambio.
- Si el origen es GPL/AGPL, un derivado no puede relicenciarse a MIT.
- Si el origen no tiene licencia, dilo: no se puede asumir reutilización libre.

Verifica siempre con los archivos reales:

```bash
git remote -v
git log --reverse --format='%an <%ae>' | head
git shortlog -sne
ls LICENSE* NOTICE* COPYING*
```

Y en el manifest: `repository`, `author`, `contributors`, `homepage`, y cualquier
campo de fork o upstream.

## Nombres de sección

Elige uno y sé consistente. En español:

- `Créditos y agradecimientos` (recomendado cuando hay ascendencia);
- `Agradecimientos` (solo gratitud);
- `Créditos` (autoría);
- `Basado en` (cuando el vínculo es el eje del proyecto).

En inglés: `Credits`, `Acknowledgments`, `Based on`, `Built on`.

## Plantilla — fork o proyecto basado en otro

```markdown
## Créditos y agradecimientos

Este proyecto está basado en [Nombre original](https://github.com/owner/repo),
creado por [Autor](https://github.com/autor) y distribuido bajo licencia MIT.

Qué cambió este repositorio:

- nueva interfaz de configuración;
- migración de almacenamiento a SQLite;
- soporte para Windows.

El aviso de copyright original se conserva en [LICENSE](LICENSE).
```

## Plantilla — código o skill reutilizada

```markdown
## Agradecimientos

- [Proyecto A](https://github.com/owner/a) — de aquí proviene la estructura de
  referencia usada en `references/`.
- [Proyecto B](https://example.com) — inspiración para el flujo de instalación.
```

## Errores frecuentes

- presentar como propio un proyecto que es un fork;
- agradecer sin link verificable;
- atribuir a un autor que no confirma el historial de git;
- declarar "inspirado en X" cuando en realidad se copió código de X;
- listar veinte dependencias como si fueran créditos;
- omitir la licencia del origen cuando hubo reutilización.

---

# Checklist de esta referencia

* [ ] ¿Cada badge apunta a un dato que existe (workflow, release, licencia)?
* [ ] ¿El badge de CI usa el nombre real del archivo de workflow y la rama real?
* [ ] ¿Hay un badge por dato, sin repetición?
* [ ] ¿Los topics propuestos son reales, en minúsculas y sin espacios?
* [ ] ¿Los topics se derivan del stack y del propósito, no de tendencias?
* [ ] ¿Se distingue badge (README) de topic (configuración del repo)?
* [ ] ¿Si el proyecto es fork o derivado, existe sección de créditos con los cinco elementos?
* [ ] ¿Se conserva el copyright, `LICENSE` y `NOTICE` del origen?
* [ ] ¿La licencia declarada es compatible con la del origen?
