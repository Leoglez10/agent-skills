---
name: readme-completo
description: "Trigger: readme, README, crear readme, mejorar readme, documentar proyecto, documentación principal, actualizar readme, hacer readme completo. Inspecciona profundamente un repositorio y crea o actualiza un README profesional, visual, verificable y comprensible tanto para usuarios no técnicos como para desarrolladores."
license: Apache-2.0
metadata:
  author: "Leoglez10"
  version: "2.3"
---

# Skill: README Completo

## Propósito

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


# Progressive Disclosure — carga solo lo necesario

Esta skill minimiza el contexto cargado durante la ejecución.

`SKILL.md` contiene únicamente:

- reglas fundamentales;
- criterios de verificación;
- flujo principal de trabajo;
- clasificación del proyecto;
- decisiones sobre qué documentación generar;
- validación final.

La información especializada vive en `references/` y debe consultarse únicamente
cuando sea relevante para el proyecto analizado.

## Regla principal

NO cargues todas las referencias automáticamente.

Primero inspecciona y clasifica el proyecto.

Después consulta únicamente las referencias necesarias.

## Cuándo consultar cada referencia

| Referencia | Consultar cuando |
|---|---|
| `references/repository-inspection.md` | Necesitas saber qué archivos inspeccionar, manifests, lockfiles, configuración, infraestructura, CI/CD, variables de entorno, detección del root real, o cómo tratar un README existente y sus contradicciones |
| `references/readme-structure.md` | Ya clasificaste el proyecto y toca elegir las secciones del README |
| `references/user-documentation.md` | El proyecto tiene usuarios finales o interfaz de usuario |
| `references/developer-documentation.md` | El repositorio va a ser usado o modificado por desarrolladores |
| `references/skill-projects.md` | El repositorio ES una skill, una colección de skills o un plugin de agente (hay `SKILL.md`, `agents/`, `commands/`, `plugin.json` o `.claude-plugin/`) |
| `references/data-and-security.md` | El proyecto almacena datos, usa base de datos, maneja autenticación, secretos, backups, migraciones o expone servicios |
| `references/badges-and-credits.md` | El README va a llevar badges, o el proyecto deriva de otro repositorio (fork, plantilla, port, código vendorizado) o reutiliza código de terceros |
| `references/writing-style.md` | Estás en la redacción final |

`user-documentation.md` NO se consulta para librerías internas, paquetes o
proyectos exclusivamente técnicos, salvo que exista una audiencia de usuario final.

`readme-structure.md` NO es una plantilla obligatoria. Un buen README no es el que
contiene más secciones, sino el que contiene las secciones necesarias para ese
proyecto.

---

# Flujo de decisión

```text
Inspeccionar repositorio
        ↓
Identificar tipo de proyecto
        ↓
Identificar audiencias
        ↓
Identificar capacidades y riesgos
        ↓
Seleccionar referencias necesarias
        ↓
Consultar únicamente esas referencias
        ↓
Diseñar estructura del README
        ↓
Redactar
        ↓
Validar contra el repositorio real
```

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

Para la lista completa de archivos a inspeccionar por stack, y para el inventario
técnico que debes construir antes de escribir, consulta
`references/repository-inspection.md`.

---
# Fase 2 — Clasificar el proyecto

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
* librería reusable;
* skill de agente, colección de skills o plugin.

La estructura final del README debe adaptarse al tipo.

Si detectas `SKILL.md`, `agents/`, `commands/`, `hooks/`, `plugin.json` o
`.claude-plugin/`, el proyecto NO es una aplicación: es un artefacto que se
instala en un agente y se activa por contexto. Carga
`references/skill-projects.md` antes de diseñar la estructura.

NO copies automáticamente todas las secciones descritas en `references/readme-structure.md`.

---

# Fase 3 — Identificar audiencias

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

### Aprendiz (repos educativos y colecciones de skills)

Quiere saber:

* cómo funciona el mecanismo, no solo cómo usarlo;
* por qué el proyecto está estructurado así;
* cómo crear su propia versión;
* qué convenciones seguir.

Esta audiencia aplica cuando el repo existe para enseñar o para ser extendido por
terceros. En ese caso el README necesita una sección didáctica, no solo referencia.

Organiza el README de menor a mayor complejidad técnica.

---

# Workflow de ejecución

Cuando se active la skill:

## Paso 1 — Inspeccionar

Carga `references/repository-inspection.md` y lee:

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

Selecciona las referencias necesarias según la tabla de Progressive Disclosure y consúltalas.

Con `references/readme-structure.md` cargado, crea el índice apropiado para ESE proyecto.

## Paso 5 — Redactar

Carga `references/writing-style.md`.

Escribe de menor a mayor complejidad.

## Paso 6 — Añadir visuales

Si existen:

* logo;
* screenshots;
* badges;

No inventarlos.

Cada badge debe apuntar a un dato comprobable: workflow real, release real,
licencia existente. Un badge sin respaldo es una afirmación falsa.

Fuera de alcance: topics, descripción "About", social preview, releases y
archivos de comunidad son configuración del repositorio, no del README. No los
propongas aquí; si el usuario los pide, usa la skill `repo-presentacion`.

Si el proyecto deriva de otro repositorio (fork, plantilla, scaffold, port,
código vendorizado) o reutiliza código de terceros, la sección de créditos y
agradecimientos es obligatoria, con link al origen, autor, qué cambió este repo y
licencia del origen.

Detalle de sintaxis, catálogo de badges y reglas de atribución:
`references/badges-and-credits.md`.

## Paso 7 — Validar

Ejecuta la checklist de esta skill.

## Paso 8 — Reportar

Describe brevemente:

* qué se añadió;
* qué se corrigió;
* qué no pudo verificarse;
* qué documentación quedó pendiente.

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
* [ ] ¿Existe sección de créditos/agradecimientos cuando el proyecto es fork, derivado o reutiliza código de terceros?

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
* [ ] ¿Cada badge tiene un dato comprobable detrás (workflow, release, licencia)?
* [ ] ¿El badge de CI usa el nombre real del archivo de workflow y la rama real?

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
| El repo contiene `SKILL.md`            | Cargar `references/skill-projects.md`; no documentar comandos de build inexistentes |
| El repo existe para enseñar o extender | Incluir sección didáctica con ejemplos reales del propio repo |
| El proyecto es fork o derivado de otro repo | Sección obligatoria de créditos y agradecimientos con origen, autor, cambios y licencia |
| El proyecto reutiliza código, skill o arquitectura de terceros | Agradecimientos breves con links verificables |
| El usuario pide "poner tags" | Si son badges, van en el README. Si son topics o "About", usar `repo-presentacion` |

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

Badges y créditos:
- badges incluidos y el dato que respalda cada uno
- sección de créditos por ascendencia externa, si aplica
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

