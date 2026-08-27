---
name: readme-completo
description: "Trigger: readme, README, crear readme, mejorar readme, documentar proyecto, documentación principal, actualizar readme, hacer readme completo. Inspecciona profundamente un repositorio y crea o actualiza un README profesional, visual, verificable y comprensible tanto para usuarios no técnicos como para desarrolladores."
license: Apache-2.0
metadata:
  author: "Leoglez10"
  version: "1.0"
---

# Skill: README Completo

## Propósito|

Esta skill sirve para crear o actualizar el `README.md` principal de un proyecto después de inspeccionar el repositorio real.

El objetivo NO es simplemente producir un README visualmente atractivo.

El README debe funcionar, cuando aplique, como:

1. Landing page del proyecto.
2. Explicación para alguien que nunca ha visto el software.
3. Guía rápida de inicio.
4. Manual básico de usuario.
5. Manual operativo.
6. Guía de instalación.
7. Guía para desarrolladores.
8. Mapa de arquitectura.
9. Runbook básico de mantenimiento.
10. Punto de entrada hacia documentación más especializada.

El README debe permitir que alguien nuevo entienda:

- qué es el proyecto;
- qué problema resuelve;
- para quién existe;
- qué puede hacer;
- cómo probarlo o instalarlo;
- cómo usarlo;
- dónde guarda los datos;
- qué riesgos o limitaciones tiene;
- cómo recuperarse de errores;
- cómo ejecutar el proyecto;
- cómo está organizado;
- cómo contribuir.

---

# Activation Contract

Usa esta skill cuando el usuario pida:

- crear un README;
- mejorar un README existente;
- hacer el README "más completo";
- documentar un repositorio;
- documentar una aplicación;
- hacer documentación principal para GitHub;
- actualizar el README después de cambios importantes;
- explicar un proyecto desde cero;
- preparar un repositorio para hacerlo público;
- generar documentación para usuarios y desarrolladores.

También puede activarse cuando:

- existe un `README.md` claramente desactualizado;
- se agregaron funcionalidades importantes que el README no menciona;
- cambió el proceso de instalación;
- cambió el stack;
- cambió la estructura del proyecto;
- se agregó CI/CD;
- se agregaron releases;
- existen nuevas variables de entorno;
- cambió la arquitectura;
- se añadieron mecanismos de backup/migración;
- el usuario pide "dejar bien documentado el proyecto".

---

# Principle Zero: investigar antes de escribir

NUNCA escribas el README basándote únicamente en:

- el nombre del repositorio;
- la descripción de GitHub;
- un README antiguo;
- suposiciones sobre el framework;
- convenciones habituales del stack.

Primero inspecciona el repositorio.

La documentación debe describir el proyecto REAL.

## Regla fundamental

> Si una afirmación técnica no puede verificarse en el repositorio, no debe presentarse como un hecho.

Si no puede comprobarse:

- omítela;
- descríbela como pendiente;
- o indícala explícitamente como una suposición que necesita confirmación.

NUNCA inventes:

- comandos;
- puertos;
- credenciales;
- rutas;
- variables de entorno;
- scripts;
- funcionalidades;
- versiones;
- endpoints;
- plataformas soportadas;
- mecanismos de autenticación;
- backups;
- CI;
- releases;
- base de datos;
- licencias;
- requisitos del sistema.

Un README técnicamente falso es peor que uno incompleto.

---

# Source of Truth

Cuando haya contradicciones, usa esta prioridad aproximada:

1. Código ejecutado actualmente.
2. Configuración activa del proyecto.
3. Tests.
4. Scripts de automatización.
5. Workflows de CI/CD.
6. Archivos de dependencias.
7. Configuración de infraestructura.
8. Documentación técnica reciente.
9. README existente.
10. Comentarios históricos.
11. Archivos legacy/archive.

Ejemplos:

- `package.json` manda sobre una versión escrita manualmente en documentación.
- El código real de migraciones manda sobre un `database.sql` histórico.
- Un workflow activo en `.github/workflows/` manda sobre una guía antigua de releases.
- Un `.env.example` manda sobre variables mencionadas únicamente en un documento viejo.

Cuando detectes discrepancias importantes, corrígelas en el README o señálalas.

---

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

# Fase 3 — Clasificar el proyecto

Determina qué estás documentando.

Puede ser:

* aplicación web;
* aplicación desktop;
* aplicación móvil;
* CLI;
* biblioteca;
* API;
* backend;
* monorepo;
* herramienta interna;
* proyecto educativo;
* SaaS;
* proyecto open-source;
* infraestructura;
* librería reusable.

La estructura final del README debe adaptarse al tipo.

NO copies automáticamente todas las secciones de esta skill.

---

# Fase 4 — Identificar audiencias

Un buen README puede tener varias capas.

Identifica qué personas pueden llegar al repositorio.

Ejemplo:

### Usuario final

Quiere saber:

* qué hace;
* cómo instalarlo;
* cómo usarlo;
* qué hacer si falla.

### Administrador

Quiere saber:

* configuración;
* datos;
* backups;
* usuarios;
* permisos;
* mantenimiento.

### Desarrollador nuevo

Quiere saber:

* stack;
* setup;
* comandos;
* estructura;
* flujo de desarrollo.

### Maintainer

Quiere saber:

* arquitectura;
* CI;
* releases;
* migraciones;
* riesgos;
* deuda técnica;
* documentación adicional.

Organiza el README de menor a mayor complejidad técnica.

---

# Arquitectura recomendada del README

No todas las secciones son obligatorias.

Úsalas únicamente cuando tengan sentido.

---

# 1. Hero

El comienzo debe explicar el proyecto inmediatamente.

Formato recomendado:

```html
<div align="center">

<img src="..." alt="..." width="150"/>

# Nombre del Proyecto

### Descripción de una línea

badges...

stack badges...

</div>
```

Debe contener idealmente:

* logo;
* nombre;
* descripción corta;
* estado de CI;
* versión;
* plataforma;
* licencia;
* tecnologías principales.

No satures el hero.

Normalmente 4–10 badges son suficientes.

---

# Política de badges

Usa badges únicamente cuando aporten información.

Buenos ejemplos:

* CI;
* build;
* versión;
* release;
* licencia;
* plataforma;
* lenguaje;
* framework;
* runtime;
* cobertura si existe realmente.

Evita badges decorativos sin utilidad.

NO muestres:

```text
Build Passing
```

si no existe CI real que lo demuestre.

No inventes coverage.

No declares una licencia que el repositorio no tenga.

---

# 2. Explicación de audiencia

Cuando el proyecto tenga usuarios no técnicos, incluye algo parecido a:

```markdown
> Este README está pensado tanto para usuarios finales como para desarrolladores.
> Si solo quieres instalar la aplicación, ve directamente a Instalación.
> Si quieres modificar el código, ve a Desarrollo.
```

Esto evita que una persona normal tenga que atravesar 500 líneas técnicas antes de encontrar cómo instalar el programa.

---

# 3. Quick Start

Cuando el proyecto lo permita, agrega:

```markdown
> 🚀 **¿Tienes prisa?**
>
> 1. Instala...
> 2. Configura...
> 3. Ejecuta...
```

Debe contener únicamente los pasos imprescindibles.

Ideal:

3–5 pasos.

No conviertas Quick Start en otro manual completo.

---

# 4. Tabla de contenidos

Añádela cuando el README sea largo.

Recomendada aproximadamente cuando:

* supera 8–10 secciones;
* tiene manual de usuario;
* tiene documentación técnica extensa.

Comprueba que los anchors funcionen en GitHub.

---

# 5. ¿Qué es?

Explica primero en lenguaje humano.

Evita comenzar con:

> "Es una arquitectura basada en..."

Mejor:

> "Es una aplicación que permite..."

Debe responder:

* qué es;
* para qué sirve;
* qué problema resuelve;
* dónde se utiliza.

Cuando sea útil, utiliza una analogía sencilla:

> "Piensa en ella como..."

La analogía no debe sustituir la explicación técnica.

---

# 6. ¿Para quién es?

Si existen roles, usa tabla:

```markdown
| Rol | Qué puede hacer |
|---|---|
| Usuario | ... |
| Administrador | ... |
```

Especialmente útil si existen:

* usuarios;
* admins;
* operadores;
* profesores;
* estudiantes;
* clientes;
* maintainers.

---

# 7. Vista rápida

Cuando exista interfaz gráfica, incluir screenshots reales.

Ejemplo:

```markdown
| Pantalla | Captura |
|---|---|
| Inicio | ![Inicio](docs/img/inicio.png) |
| Admin | ![Admin](docs/img/admin.png) |
```

## Regla de screenshots

Las imágenes deben:

* existir realmente;
* representar la versión actual;
* tener nombres descriptivos;
* estar almacenadas preferentemente dentro del repo;
* evitar información privada.

Si falta una captura:

```markdown
_(pendiente)_
```

Es mejor que inventarla.

Si existe un procedimiento especial para generar screenshots, documéntalo brevemente.

---

# 8. Funciones principales

Describe funcionalidades verificadas.

Formato recomendado:

```markdown
- ✅ ...
- ✅ ...
- ✅ ...
```

Orden:

1. funcionalidad principal;
2. workflows importantes;
3. automatizaciones;
4. integraciones;
5. seguridad/persistencia;
6. capacidades secundarias.

No confundas:

* funcionalidad existente;
* experimento;
* roadmap.

Si algo está incompleto:

```markdown
- 🧪 Acceso LAN experimental
```

---

# 9. Instalación para usuario final

Separar completamente de instalación para desarrolladores.

Debe explicar:

### Requisitos

Ejemplo:

```markdown
- Windows 10/11
- macOS...
- Docker...
```

### Instalación

Paso por paso.

### Primer inicio

Explica:

* configuración inicial;
* permisos;
* directorios;
* primera cuenta;
* importaciones;
* posibles warnings.

No asumas conocimientos técnicos.

---

# 10. Credenciales iniciales

Solamente si existen de verdad.

Si hay credenciales por defecto:

```markdown
> ⚠️ Cambia estas credenciales inmediatamente.
```

Pero primero revisa si publicarlas sería seguro.

NUNCA publiques:

* credenciales reales de producción;
* API keys;
* secretos;
* tokens;
* contraseñas privadas;
* llaves SSH.

Diferencia entre:

* credencial pública de demo;
* valor bootstrap diseñado para distribuirse;
* secreto real.

Si existe un mecanismo inseguro, dilo claramente.

NO maquilles debilidades de seguridad.

---

# 11. Cómo usar la aplicación

Documenta workflows, no cada botón.

Ejemplo:

```markdown
### Flujo 1: Crear un préstamo

1. ...
2. ...
3. ...

### Flujo 2: Devolver equipo

1. ...
2. ...
```

Prioriza escenarios reales:

```text
"Un usuario quiere..."
"Un administrador necesita..."
"Quiero recuperar..."
```

sobre describir componentes visuales aislados.

---

# 12. Conceptos importantes

Si existe una regla que puede confundir al usuario, explíquela explícitamente.

Ejemplos:

```markdown
> 💡 Concepto clave — Público vs privado
```

```markdown
> ⚠️ Importar registros no significa publicarlos.
```

```markdown
> 💡 Una cuenta local no es una cuenta cloud.
```

Este tipo de explicación evita muchos issues y errores operativos.

---

# 13. Importación / exportación

Si el proyecto trabaja con:

* Excel;
* CSV;
* JSON;
* archivos;
* bases externas;
* migraciones;

documenta:

1. dónde se hace;
2. formato soportado;
3. qué sucede antes de importar;
4. cómo resuelve duplicados;
5. qué modifica;
6. qué conserva;
7. cómo revertir.

Cuando existan reglas complejas, usa una tabla.

---

# 14. Persistencia de datos

Para aplicaciones que almacenan datos localmente, explica dónde.

Ejemplo:

```text
AppData/
├── app.db
└── backups/
```

Debe aclarar:

* qué archivo es crítico;
* qué puede borrarse;
* qué NO debe borrarse;
* si los datos están en cloud o local;
* si distintas instalaciones comparten datos.

No inventes rutas.

Obtén la ruta desde:

* configuración;
* código;
* documentación confiable;
* APIs del framework.

---

# 15. Backup y recuperación

Obligatoria cuando la pérdida de datos sea relevante.

Explica:

### Crear backup

### Backup automático

### Restaurar

### Qué ocurre durante restore

### Dónde están los backups

### Qué hacer si falla

Prioriza instrucciones soportadas por la propia aplicación antes que soluciones manuales.

---

# 16. Mantenimiento

Úsala para aplicaciones operativas.

Puede incluir:

```markdown
| Frecuencia | Tarea |
|---|---|
| Diario | ... |
| Semanal | ... |
| Mensual | ... |
```

Solo recomienda tareas que tengan sentido para el proyecto.

---

# 17. Actualización

Si hay releases o instaladores:

Documenta:

1. crear backup;
2. cerrar aplicación;
3. instalar actualización;
4. migraciones;
5. validar;
6. rollback.

Comprueba primero cómo funciona realmente el proceso.

---

# 18. Troubleshooting

Incluye problemas frecuentes verificables.

Formato recomendado:

```markdown
| Problema | Causa probable | Solución |
|---|---|---|
| ... | ... | ... |
```

Nunca inventes soluciones solamente porque "suenan razonables".

---

# 19. GitHub para principiantes

Solo incluir si la audiencia del proyecto lo justifica.

Puede explicar:

* repo;
* clone;
* commit;
* push;
* pull;
* branch;
* PR.

Si el repositorio está orientado exclusivamente a desarrolladores experimentados, esta sección probablemente sobra.

---

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

# 23. Variables de entorno

Si existen:

```markdown
| Variable | Obligatoria | Descripción |
|---|---|---|
| `DATABASE_URL` | Sí | ... |
```

NUNCA muestres valores reales de secretos.

Usa únicamente:

* placeholders;
* `.env.example`;
* valores públicos.

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

# 28. FAQ

Crear FAQ basándote en:

* comportamiento del producto;
* limitaciones;
* configuración;
* problemas previsibles;
* issues existentes;
* decisiones arquitectónicas.

Preguntas buenas:

```text
¿Necesita internet?
¿Dónde guarda los datos?
¿Funciona en macOS?
¿Cómo recupero mis datos?
¿Por qué no aparece X?
¿Puedo usarlo en varias computadoras?
```

Las respuestas deben existir en el código/documentación.

No inventes FAQs solamente para llenar espacio.

---

# 29. Limitaciones

Esta sección puede estar separada o integrada.

Sé transparente.

Ejemplos:

```markdown
- No hay sincronización cloud.
- No existe build oficial para Linux.
- No hay tests E2E.
- La autenticación actual no está diseñada para Internet público.
```

La documentación profesional reconoce sus límites.

No conviertas el README en marketing.

---

# 30. Seguridad

Si detectas riesgos visibles importantes:

* credenciales hardcodeadas;
* fallback de autenticación;
* CSP desactivado;
* storage inseguro;
* secretos;
* falta de autorización;
* endpoints sin protección;

NO los ignores.

Pero tampoco conviertas el README en un informe de pentesting.

Incluye únicamente advertencias relevantes para operar o desplegar correctamente el sistema.

Nunca publiques secretos al documentarlos.

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

# 34. Documentación relacionada

Si `docs/` contiene documentación valiosa, crea índice:

```markdown
| Documento | Propósito |
|---|---|
| `ENGINEERING_HANDBOOK.md` | ... |
```

No enlaces documentos históricos como si fueran actuales.

Clasifica cuando sea necesario:

* actual;
* experimental;
* legacy;
* archive.

---

# 35. Roadmap

Solo añadir si existe un roadmap real o el usuario lo solicita.

No conviertas TODOs encontrados en código en promesas públicas automáticamente.

---

# 36. Licencia

Comprueba:

```text
LICENSE
package.json
Cargo.toml
pyproject.toml
```

Si no existe licencia, NO pongas automáticamente MIT.

Puedes decir:

```markdown
Actualmente el repositorio no incluye una licencia pública.
```

---

# 37. Créditos

Al final puede incluirse:

```html
<div align="center">

### Diseñado y desarrollado por **Nombre**

GitHub badge...

</div>
```

Incluye:

* autores reales;
* organización;
* institución;
* proyecto;
* links relevantes.

No atribuyas autoría basándote únicamente en el owner de GitHub si existen más contributors relevantes.

---

# 38. Footer

Opcional.

Puede enlazar las secciones más consultadas:

```markdown
**¿Dudas?**
[FAQ](...) ·
[Instalación](...) ·
[Desarrollo](...)
```

Evita frases excesivamente genéricas.

---

# Progressive Disclosure

El orden recomendado es:

```text
VISITANTE
    ↓
qué es
    ↓
qué hace
    ↓
cómo probarlo
    ↓
USUARIO
    ↓
cómo usarlo
    ↓
operación
    ↓
problemas
    ↓
DESARROLLADOR
    ↓
setup
    ↓
arquitectura
    ↓
tests
    ↓
CI/releases
    ↓
MAINTAINER
    ↓
docs
    ↓
contribución
```

No empieces enseñando arquitectura interna a alguien que aún no sabe qué hace el proyecto.

---

# Reglas de redacción

## Escribe para humanos

Preferir:

> "La aplicación guarda los datos en esta computadora."

antes que:

> "La persistencia es ejecutada mediante una instancia SQLite embebida."

Luego puedes añadir el detalle técnico.

---

# Define conceptos

Cuando uses términos como:

* migration;
* SQLite;
* CI;
* PR;
* branch;
* build;
* API;

y la audiencia pueda ser no técnica, explícalos la primera vez.

---

# Evita lenguaje de IA

No escribas frases vacías como:

* "potente solución";
* "robusta herramienta";
* "experiencia fluida";
* "innovadora plataforma";
* "solución integral de última generación".

Describe qué hace concretamente.

---

# Usa ejemplos reales

Preferir:

```text
App Prestamos P15_0.3.0_x64-setup.exe
```

si ese asset existe realmente.

Evita ejemplos inventados que puedan confundirse con nombres reales.

---

# Emojis

Los emojis pueden utilizarse como señalización visual.

Ejemplo:

```text
🎯 propósito
📦 instalación
🚶 uso
💾 backups
🧰 mantenimiento
👨‍💻 desarrollo
🗺 estructura
❓ FAQ
```

No poner emoji en cada oración.

La función es navegación visual, no decoración.

---

# Callouts

Usa callouts de manera consistente.

```markdown
> 💡 Información útil
```

```markdown
> ⚠️ Riesgo o advertencia
```

```markdown
> ✅ Estado correcto
```

```markdown
> 🧪 Funcionalidad experimental
```

No abuses de ellos.

---

# Tablas

Úsalas para información comparable:

* roles;
* estados;
* scripts;
* troubleshooting;
* variables;
* documentación;
* mantenimiento.

No conviertas párrafos largos en tablas innecesariamente.

---

# Código

Los comandos deben poder copiarse.

Usa el lenguaje apropiado:

```bash
npm install
```

```powershell
git clone ...
```

```json
{
  "example": true
}
```

---

# Links

Siempre comprueba:

* paths relativos;
* nombres de archivos;
* anchors;
* URLs del repo;
* workflows;
* issues;
* releases.

No dejes links provenientes de una plantilla anterior.

---

# Política contra documentación falsa

Antes de afirmar:

```text
"funciona offline"
```

encuentra evidencia.

Antes de afirmar:

```text
"soporta Windows 10 y 11"
```

encuentra evidencia.

Antes de afirmar:

```text
"crea backups automáticos"
```

encuentra evidencia.

Antes de afirmar:

```text
"el build se publica automáticamente"
```

revisa el workflow.

Antes de afirmar:

```text
"los datos nunca salen de la computadora"
```

busca:

* fetch;
* axios;
* SDKs;
* telemetry;
* APIs;
* analytics;
* servicios externos.

---

# Revisión de seguridad antes de publicar

Busca accidentalmente en el README:

```text
password
passwd
secret
token
api_key
apikey
private_key
DATABASE_URL
Bearer
Authorization
```

Comprueba que no se haya incluido información sensible.

Si existe un `.env`, NO copies sus valores.

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

# Fase de validación

Antes de terminar el README realiza esta checklist.

## Contenido

* [ ] ¿Explica qué es el proyecto en menos de 30 segundos?
* [ ] ¿Explica para quién es?
* [ ] ¿Describe funcionalidades reales?
* [ ] ¿Existe un camino rápido para empezar?
* [ ] ¿Separa usuario final de desarrollador?
* [ ] ¿Explica las limitaciones importantes?
* [ ] ¿Explica dónde viven los datos cuando aplica?
* [ ] ¿Explica backup/restore cuando aplica?
* [ ] ¿Incluye troubleshooting útil?
* [ ] ¿Incluye estructura del proyecto?

## Técnica

* [ ] ¿Los comandos existen?
* [ ] ¿Los scripts existen?
* [ ] ¿Las versiones coinciden?
* [ ] ¿Los puertos coinciden?
* [ ] ¿Los paths existen?
* [ ] ¿Los workflows existen?
* [ ] ¿Los triggers de CI son correctos?
* [ ] ¿Las plataformas declaradas son correctas?
* [ ] ¿Los nombres de base de datos son correctos?

## Links

* [ ] ¿Los links relativos existen?
* [ ] ¿Los screenshots existen?
* [ ] ¿Los anchors funcionan?
* [ ] ¿Issues apunta al repo correcto?
* [ ] ¿Releases apunta al repo correcto?
* [ ] ¿Los badges apuntan al workflow correcto?

## Seguridad

* [ ] ¿No se expusieron secretos?
* [ ] ¿No se publicaron credenciales privadas?
* [ ] ¿Las credenciales demo están identificadas como demo?
* [ ] ¿Las advertencias importantes están documentadas?
* [ ] ¿No se hacen afirmaciones de seguridad sin comprobar?

## Calidad

* [ ] ¿Puede entenderlo una persona no técnica?
* [ ] ¿Puede hacer setup un desarrollador nuevo?
* [ ] ¿Puede encontrar rápidamente dónde modificar algo?
* [ ] ¿El README evita repetir docs especializadas?
* [ ] ¿Los detalles complejos enlazan a `docs/`?
* [ ] ¿No está relleno con marketing vacío?

---

# Tamaño del README

No existe un tamaño máximo arbitrario.

La regla correcta es:

> El README puede ser largo si cada sección ayuda a completar una tarea real.

Sin embargo:

Cuando un tema necesite demasiados detalles técnicos, mueve el contenido profundo a:

```text
docs/
```

y deja en README:

1. resumen;
2. instrucciones principales;
3. enlace al documento especializado.

README = mapa principal.

`docs/` = profundidad.

---

# Qué NO hacer

NO crear un README compuesto solamente por:

```markdown
# Mi proyecto

## Instalación
npm install

## Ejecutar
npm run dev
```

si el proyecto requiere documentación real.

Tampoco hagas lo contrario:

NO documentes cada función, componente, clase o archivo.

Eso pertenece a documentación técnica específica.

---

# No copiar esta plantilla literalmente

Esta skill define un SISTEMA, no una plantilla rígida.

Para cada proyecto determina qué secciones tienen sentido.

Ejemplo:

Una librería probablemente necesita:

```text
Installation
Quick Start
API
Examples
Compatibility
Contributing
License
```

y NO necesita:

```text
Backup
Manual de usuario
Mantenimiento mensual
Screenshots
```

Una app de escritorio local sí puede necesitarlos.

---

# Adaptación por tipo de proyecto

## Aplicación web

Priorizar:

* demo;
* screenshots;
* funciones;
* env;
* desarrollo;
* deployment;
* arquitectura.

## Desktop

Priorizar:

* descarga;
* plataformas;
* instalación;
* almacenamiento local;
* updates;
* backups;
* troubleshooting.

## API

Priorizar:

* propósito;
* arquitectura;
* auth;
* env;
* endpoints principales;
* ejecución;
* tests;
* deployment;
* OpenAPI.

## CLI

Priorizar:

* instalación;
* usage;
* comandos;
* flags;
* ejemplos;
* configuración;
* shell completion.

## Library

Priorizar:

* installation;
* minimal example;
* API;
* compatibility;
* examples;
* migration guides.

## Proyecto interno

Priorizar:

* usuarios;
* workflows;
* operación;
* backups;
* recuperación;
* mantenimiento;
* ownership.

---

# Workflow de ejecución

Cuando se active la skill:

## Paso 1 — Inspeccionar

Lee:

* README existente;
* manifiestos;
* configuración;
* código central;
* workflows;
* scripts;
* docs;
* estructura.

## Paso 2 — Entender

Determina:

* producto;
* usuarios;
* arquitectura;
* instalación;
* operación;
* desarrollo.

## Paso 3 — Verificar

Contrasta documentación existente contra código.

## Paso 4 — Diseñar

Crea el índice apropiado para ESE proyecto.

## Paso 5 — Redactar

Escribe de menor a mayor complejidad.

## Paso 6 — Añadir visuales

Si existen:

* logo;
* screenshots;
* badges.

No inventarlos.

## Paso 7 — Validar

Ejecuta la checklist de esta skill.

## Paso 8 — Reportar

Describe brevemente:

* qué se añadió;
* qué se corrigió;
* qué no pudo verificarse;
* qué documentación quedó pendiente.

---

# Decision Gates

| Situación                              | Acción                                      |
| -------------------------------------- | ------------------------------------------- |
| No existe README                       | Crear uno desde cero                        |
| README muy corto                       | Investigar repo y expandirlo                |
| README largo pero desactualizado       | Conservar estructura útil y corregir hechos |
| Proyecto pequeño                       | README compacto                             |
| Proyecto complejo                      | README completo + enlaces a docs            |
| No hay logo                            | No inventar uno                             |
| No hay screenshots                     | Omitir o marcar pendiente                   |
| No hay LICENSE                         | No asumir MIT                               |
| No existe CI                           | No poner badge de build                     |
| No existe test script                  | No inventar comando                         |
| Hay funcionalidades experimentales     | Identificarlas como experimentales          |
| Código contradice documentación        | El código/config actual prevalece           |
| Hay posible secreto                    | No incluirlo y advertir al usuario          |
| Hay información imposible de verificar | Omitirla o marcarla pendiente               |

---

# Output Contract

Cuando el usuario pida crear o actualizar el README:

Entrega un `README.md` completo y listo para guardar.

Además, reporta de forma breve:

```text
README actualizado:
- secciones añadidas
- inconsistencias corregidas
- información importante encontrada
- puntos que requieren confirmación
```

Si el usuario pidió modificar directamente el repositorio:

1. inspecciona;
2. genera;
3. actualiza `README.md`;
4. vuelve a leerlo;
5. verifica que no haya links/paths evidentemente rotos;
6. reporta el cambio.

---

# Quality Bar

El resultado debe conseguir que:

### Un visitante pueda decir:

> "Entiendo qué hace."

### Un usuario pueda decir:

> "Sé instalarlo y usarlo."

### Un administrador pueda decir:

> "Sé dónde están los datos y qué hacer si algo falla."

### Un desarrollador pueda decir:

> "Sé levantarlo y sé dónde buscar el código."

### Un maintainer pueda decir:

> "Entiendo cómo se construye, prueba, publica y mantiene."

Si una de estas audiencias aplica al proyecto y el README no puede responderle, todavía no está terminado.

