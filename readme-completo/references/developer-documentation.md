# Reference — Documentación para desarrolladores

Consultar cuando el repositorio vaya a ser usado o modificado por desarrolladores:
stack, requisitos, instalación de dependencias, desarrollo, build, tests, lint,
typecheck, scripts, estructura, arquitectura, CI/CD, releases y contribución.


# 20. Para programadores

Esta sección debe ser precisa.

---

## Stack

Extraer versiones de los archivos reales.

Ejemplo:

```markdown
- Frontend: React 19
- Lenguaje: TypeScript 5
- Desktop: Tauri 2
- Persistencia: SQLite
```

No uses versiones de memoria.

---

## Requisitos

Ejemplo:

```markdown
- Node.js 22+
- Rust stable
- Docker
```

Solo si realmente son necesarios.

---

## Clonar

Generar el path correcto.

Comprueba si existe una subcarpeta adicional después de clonar.

---

## Instalar dependencias

Obtén el package manager del lockfile.

Prioridad aproximada:

```text
bun.lock → bun
pnpm-lock.yaml → pnpm
yarn.lock → yarn
package-lock.json → npm
```

Si soporta varios, puedes documentarlos.

No inventes alternativas.

---

## Desarrollo

Obtén scripts desde el manifiesto real.

Ejemplo:

```bash
npm run dev
```

No asumas que `npm run dev` existe.

---

## Build

Ejemplo:

```bash
npm run build
```

Compruébalo.

### Dónde sale el artefacto

Indica la ruta EXACTA donde queda el instalador, binario o bundle después del
build, para que quien compila sepa qué entregar. Sácala de la configuración real:

| Stack | Dónde mirar | Ejemplo de ruta |
|---|---|---|
| Tauri | `tauri.conf.json` (`bundle`), directorio `target/` | `src-tauri/target/release/bundle/<formato>/` |
| Electron | `electron-builder` (`directories.output`) | `dist/` o `release/` según la config |
| Web / bundler | `build.outDir` (Vite), `distDir` (Next) | `dist/`, `.next/`, `out/` |
| Rust | `Cargo.toml`, perfil `release` | `target/release/<binario>` |

Ejemplo:

```markdown
### Dónde sale el instalador

Después de `npm run tauri build`, el instalador queda en
`src-tauri/target/release/bundle/` (`.msi` en `msi/`, `.exe` en `nsis/`).
```

Reglas:

* nunca inventes la ruta ni los nombres de archivo: confírmalos en la config o ejecutando el build si es seguro;
* los nombres con versión deben escribirse como patrón (`app_<versión>_x64.msi`) salvo que verifiques el valor actual;
* si no puedes comprobar la ruta, márcala como pendiente.

---

## Test

Ejemplo:

```bash
npm test
```

Comprueba:

* framework;
* scripts;
* número de suites si decides mencionarlo.

Evita números específicos que probablemente queden obsoletos salvo que aporten valor.

---

## Typecheck

Ejemplo:

```bash
npx tsc --noEmit
```

Solamente si aplica.

---

## Lint

Si NO existe:

puede ser útil mencionarlo:

```markdown
> El proyecto actualmente no tiene linter configurado.
```

No inventes `npm run lint`.

---

# 21. Scripts disponibles

Cuando haya varios scripts importantes, generar:

```markdown
| Script | Qué hace |
|---|---|
| `npm run dev` | ... |
| `npm test` | ... |
```

Extraerlos directamente del manifiesto.

---

# 22. Configuración importante

Documenta únicamente valores relevantes.

Ejemplos:

* puerto dev;
* HMR;
* identificador de aplicación;
* CSP;
* tamaño de ventana;
* proxy;
* base URL;
* storage;
* límites importantes.

Evita copiar la configuración completa.

El README explica.

Los archivos de configuración contienen el detalle.

---

# 24. Estructura del proyecto

Esta es una de las partes más importantes.

Genera un árbol SELECTIVO.

NO uses automáticamente la salida completa de `tree`.

Mal:

```text
node_modules/
dist/
target/
...
500 archivos
```

Bien:

```text
src/
├── pages/
├── components/
├── hooks/
└── utils/
```

Añade comentarios:

```text
src/
├── pages/       ← Pantallas principales
├── hooks/       ← Lógica compartida
└── utils/       ← Utilidades
```

Destaca archivos arquitectónicamente relevantes.

Ejemplo:

```text
src/hooks/useInventory.ts ← ⭐ Lógica central del dominio
```

---

# Regla del árbol de proyecto

El árbol debe ayudar a responder:

> "Si quiero cambiar X, ¿dónde busco?"

Después del árbol, cuando sea útil, agrega una regla rápida:

```markdown
> 💡 Regla de oro:
> - Pantallas → `src/pages/`
> - Datos → `src/data/`
> - Backend → `server/`
> - Documentación → `docs/`
```

---

# 25. Arquitectura

Para proyectos medianos/grandes, explica:

```text
UI
 ↓
Services
 ↓
Domain
 ↓
Database
```

o:

```text
React
 ↓
Tauri commands
 ↓
Rust
 ↓
SQLite
```

No fuerces una arquitectura formal si el proyecto no la tiene.

Si existe deuda arquitectónica relevante, no la escondas.

---

# 26. CI/CD

Inspecciona `.github/workflows`.

Documenta:

* qué evento dispara el workflow;
* qué construye;
* qué plataforma usa;
* qué artefactos produce;
* si crea Releases.

Ejemplo:

```markdown
Al crear un tag `v*`, GitHub Actions compila...
```

Verifica el trigger real en YAML.

No asumas que un archivo dentro de una carpeta `.github` anidada es ejecutado por GitHub.

---

# 27. Releases

Si hay releases automáticas o manuales, explica:

* esquema de tags;
* build;
* dónde descargar;
* assets;
* prereleases si aplica.

No documentes releases si el proyecto no las usa.

## Procedimiento paso a paso

Cuando el repo tenga un flujo de publicación, documéntalo como procedimiento
numerado. Solo documenta pasos que el repo demuestre (scripts, workflows,
CONTRIBUTING, docs de release):

1. **Validar** antes de publicar: typecheck, tests y build, con los comandos reales del manifiesto.
2. **Subir la versión**: si hay un script de bump, indica cuál; si es manual, lista los archivos que deben quedar sincronizados (manifiestos, config de la app, lockfiles).
3. **Commit + tag + push**: formato real del mensaje y del tag (`v*`, etc.), tal como lo exige el workflow.
4. **Qué hace la CI al ver el tag**: workflow real, qué compila, qué artefactos adjunta y dónde aparecen (Releases).
5. **Precondiciones**: árbol limpio, rama correcta (`main`), credenciales o secretos de CI ya configurados; solo las que el script o el workflow verifican o exigen.
6. **Rollback**: cómo deshacer una publicación mala (borrar tag local y remoto, retirar o marcar la release, `yank`/`unpublish` si es un paquete). Solo si el repo o su plataforma lo permite de forma verificable.
7. **Docs autosellados**: si un script actualiza automáticamente versión o fecha en la documentación (README, manual, PDF), dilo y aclara que no se edita a mano.

Ejemplo:

````markdown
### Publicar una versión nueva

```bash
# 1. Valida antes de publicar
npm run typecheck && npm test && npm run build

# 2. Bump + commit + tag + push (todo en uno)
npm run release -- patch
```

Precondiciones: árbol limpio y rama `main`.
````

Si el repo no tiene scripts de release, documenta únicamente el proceso manual
que sí esté probado y marca el resto como pendiente.

---

# 31. Cómo contribuir

Separa la sección en dos caminos: quien no programa suele aportar más reportando
bien, y no debería tener que leer pasos de Pull Request.

### No programas: reporta (es lo más útil)

Incluye, cuando exista evidencia:

* el botón de reporte dentro de la app, si el código lo tiene (indica dónde está);
* una tabla con los formularios de `.github/ISSUE_TEMPLATE/`:

```markdown
| Quiero... | Formulario |
|---|---|
| Reportar un error | [Bug report](URL-real/issues/new?template=bug_report.yml) |
| Proponer una mejora | [Feature request](URL-real/issues/new?template=feature_request.yml) |
```

* consejos para un buen reporte:
  1. qué hiciste (pasos);
  2. qué esperabas que pasara;
  3. qué pasó en realidad (copia el mensaje de error);
  4. captura de pantalla, sin datos privados;
  5. versión de la app.

No inventes formularios ni nombres de plantilla: toma los reales de
`.github/ISSUE_TEMPLATE/`. Si no existen, enlaza a `/issues/new`.

### Sí programas: Pull Request

Cuando el repo acepte contribuciones:

```markdown
1. Clona.
2. Crea branch.
3. Haz cambios.
4. Ejecuta tests.
5. Abre PR.
```

Si existe `CONTRIBUTING.md`, enlázalo en vez de duplicarlo completamente.

---

# 32. Convención de commits

Solo documentarla si el proyecto realmente la usa o si hay instrucciones explícitas.

Ejemplos:

```text
feat:
fix:
docs:
chore:
refactor:
```

No declares Conventional Commits como regla oficial sin evidencia.

---

# 33. Versionado

Inspecciona el proyecto.

Si usa SemVer:

```markdown
MAJOR.MINOR.PATCH
```

Explica dónde se mantiene la versión.

Si varios archivos necesitan estar sincronizados, menciónalos.

Verifica sus valores actuales antes de escribir la versión actual en el README.

---

