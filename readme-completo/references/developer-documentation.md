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

---

# 31. Cómo contribuir

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

