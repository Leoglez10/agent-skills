# Reference — Inspección del repositorio

Consultar cuando necesites saber QUÉ archivos leer, cómo detectar el root real,
cómo construir el inventario técnico, o cómo tratar un README existente,
contradicciones e información histórica.


# Fase 1 — Reconocimiento del repositorio

Antes de redactar, identifica el root real del proyecto.

Algunos repositorios tienen estructuras como:

```text
repo/
├── README.md
└── app/
    ├── package.json
    └── src/
```

No asumas que el código está directamente en el root.

---

# Archivos que debes inspeccionar

No todos existirán.

## Metadatos

Busca:

```text
README.md
LICENSE
CHANGELOG.md
CONTRIBUTING.md
CODE_OF_CONDUCT.md
SECURITY.md
```

## JavaScript / TypeScript

```text
package.json
package-lock.json
bun.lock
bun.lockb
pnpm-lock.yaml
yarn.lock
tsconfig.json
vite.config.*
next.config.*
astro.config.*
```

## Python

```text
pyproject.toml
requirements.txt
Pipfile
poetry.lock
setup.py
```

## Rust

```text
Cargo.toml
Cargo.lock
```

## Go

```text
go.mod
go.sum
```

## Desktop

```text
src-tauri/
electron.*
forge.config.*
```

## Mobile

```text
capacitor.config.*
android/
ios/
pubspec.yaml
```

## Infraestructura

```text
Dockerfile
docker-compose.yml
compose.yml
.github/workflows/
.vercel/
netlify.toml
fly.toml
render.yaml
```

## Variables de entorno

```text
.env.example
.env.template
config.example.*
```

NUNCA publiques valores reales de `.env`.

## Documentación

Inspecciona:

```text
docs/
documentation/
wiki/
specs/
openspec/
adr/
architecture/
scripts/
```

---

# Fase 2 — Construir un inventario técnico

Antes de escribir el README, construye mentalmente una ficha del proyecto.

Debe contener, cuando sea posible:

```text
Nombre:
Descripción:
Problema que resuelve:
Tipo de proyecto:
Usuarios:
Plataformas:
Frontend:
Backend:
Runtime:
Base de datos:
Persistencia:
Servicios externos:
Autenticación:
Variables de entorno:
Instalación:
Modo desarrollo:
Build:
Tests:
Lint:
Typecheck:
CI:
CD:
Releases:
Datos importantes:
Backups:
Migraciones:
Limitaciones:
Seguridad:
Licencia:
Autor/es:
```

No necesariamente muestres esta ficha.

Sirve para evitar contradicciones durante la escritura.

---


---

# README existente

Cuando el usuario pide mejorar un README:

NO lo sustituyas ciegamente.

Primero clasifica su información como:

```text
✅ correcta y útil
🔄 correcta pero desactualizada
⚠️ dudosa
❌ falsa
🗃 histórica
➕ falta documentar
```

Conserva la información específica del dominio que siga siendo verdadera.

---

# Detección de contradicciones

Busca inconsistencias como:

```text
README: React 18
package.json: React 19
```

```text
README: puerto 3000
vite.config.ts: 1770
```

```text
README: npm
repo: bun.lock únicamente
```

```text
README: CI en push
workflow: tags v*
```

```text
README: PostgreSQL
código actual: SQLite
```

Corrige el README usando la fuente de verdad.

---

# Información histórica

Si encuentras código o documentación legacy:

No la presentes como actual.

Usa términos claros:

```text
legacy
histórico
experimental
archivado
referencia
no es fuente de verdad
```

---

