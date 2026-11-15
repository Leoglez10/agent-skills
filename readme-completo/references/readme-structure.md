# Reference — Estructura del README

Consultar DESPUÉS de clasificar el proyecto, para elegir únicamente las secciones
que tengan sentido. NO es una plantilla obligatoria: un buen README no es el que
tiene más secciones, sino el que tiene las necesarias para ese proyecto.


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

Usa el set obligatorio de 8–12 badges en dos filas (ver `references/badges-and-credits.md`); no agregues más en el hero.

## Línea en inglés (cuando el README no está en inglés)

Justo debajo del hero (después del `</div>`), agrega un resumen de una línea en
inglés para quien llegue desde fuera:

```markdown
> 🇬🇧 **In English:** Desktop app (stack) for <what it does> at <who uses it>. Docs below are in Spanish, the language of its users.
```

Reglas:

* una sola línea, sin traducir todo el README;
* stack, función y usuarios deben salir del repo real;
* omítela solo si el README ya está en inglés.

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

> Un badge es una afirmación verificable: si no puedes comprobar el dato detrás,
> no va.

## Distinción importante: badge vs tag

* **Badge** → vive en el README y se ve dentro de la página.
* **Tag / topic** → vive en la configuración del repositorio (GitHub Topics,
  `keywords` del manifest, descripción "About") y sirve para descubrimiento.

No son intercambiables. Esta skill solo maneja badges; los topics son de la
skill `repo-presentacion`. No declares topics que no se configuraron.

Para sintaxis de badges y errores frecuentes, consulta
`references/badges-and-credits.md`.

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

# 2b. Bloque para IAs

Aplica cuando el README es largo (> ~400 líneas) o el proyecto probablemente se
consulte mediante asistentes de IA (usuarios no técnicos que le preguntan a un
chat sobre el repo).

Plantilla:

```markdown
<details>
<summary>🤖 ¿Eres una IA leyendo esto?</summary>

**Cómo explicar este proyecto (regla principal).** Quien pregunta es, por defecto,
<perfil real de la audiencia>. Entonces:

- Explica en lenguaje sencillo: nada de <términos técnicos a evitar>. Habla de <conceptos de la interfaz>.
- Da pasos concretos y cortos.
- Si la respuesta solo se entiende con detalle técnico, **pregunta primero** si la persona quiere la versión técnica.
- Pasa a modo técnico solo si la persona lo pide o habla en términos de código.

**Dónde está cada cosa**

- Historial de cambios → [CHANGELOG.md](CHANGELOG.md)
- Instalador / descargas → [Releases](URL-real)
- Manual completo → [docs/MANUAL.md](docs/MANUAL.md)
- Guías técnicas → [Documentación relacionada](#documentación-relacionada)

**Resumen del proyecto (3 líneas):** qué es · para quién · con qué está hecho.

</details>
```

Reglas:

* solo rutas y links reales: verifica que cada archivo exista;
* la regla de tono es plegable por defecto (`<details>`); no la dejes abierta si el README ya es largo;
* el resumen de 3 líneas sale del repo, no de suposiciones;
* ajusta la regla de tono al perfil real: si la audiencia es técnica, la regla cambia.

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

Formato recomendado: nombre corto en negrita + beneficio en una línea.

```markdown
- **Dictado por voz**: el evaluador habla y la IA llena el campo correcto.
- **Guardado automático**: cada cambio se guarda en la nube.
```

Con más de 6 funciones, agrúpalas bajo encabezados `###` (no texto suelto):

```markdown
### Captura

- **...**: ...

### Reportes

- **...**: ...
```

No pongas ✅ en cada línea: todo lo listado ya está verificado, así que la marca
no distingue nada. Usa emoji solo para lo que tiene otro estado.

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
- 🧪 **Acceso LAN**: experimental.
- 🚧 **Exportar a Excel**: parcial, solo estudios cerrados.
```

---

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

## Resumen en lenguaje simple

Si existe `LICENSE`, agrega debajo del enlace un resumen corto para personas no
técnicas, con tres partes:

```markdown
## Licencia y uso

Ver **[LICENSE](LICENSE)**.

En corto:

- **Qué puedes hacer:** usarlo, instalarlo, estudiarlo y adaptarlo (según lo que permita la licencia).
- **Qué no puedes hacer:** (restricciones reales, p. ej. venderlo o redistribuirlo en un producto comercial).
- **Sin garantía:** el software se entrega tal cual; la responsabilidad de operarlo y respaldar los datos es de quien lo usa.
```

Reglas:

* deriva cada punto SOLO del texto real de `LICENSE`; no de la licencia "típica" del nombre;
* si el `LICENSE` es una licencia estándar (MIT, Apache, GPL), resume sus condiciones reales (atribución, copyleft, aviso de copyright);
* el resumen no sustituye al texto legal: siempre enlaza `LICENSE`;
* si no estás seguro de una condición, déjala fuera del resumen.

---

# 37. Créditos y agradecimientos

Dos casos distintos. No los mezcles.

## Caso A — autoría propia

Al final puede incluirse:

```html
<div align="center">

<img src="https://github.com/usuario.png" alt="Nombre" width="96"/>

### Diseñado y desarrollado por **Nombre**

GitHub badge... · Issues badge...

🏫 **Institución / organización**: ...

🎯 **Propósito**: ...

🛠 **Stack**: ...

</div>

> 📬 ¿Encontraste un error? Abre un [issue](URL-real-de-issues/new/choose).
```

Reglas del bloque:

* avatar: `https://github.com/<usuario>.png` solo de un autor verificado (historial de git, manifest, owner confirmado); sin autor verificado, omite la imagen;
* institución/organización: solo si el repo o el usuario la indican;
* propósito: una línea, tomada de la descripción real del proyecto;
* stack: una línea con lo que muestran los manifiestos;
* CTA de issues: enlaza el repo correcto; si hay formularios en `.github/ISSUE_TEMPLATE/`, usa `/issues/new/choose`.

Incluye:

* autores reales;
* organización;
* institución;
* proyecto;
* links relevantes.

No atribuyas autoría basándote únicamente en el owner de GitHub si existen más contributors relevantes.

## Caso B — el proyecto deriva de otro repositorio

Obligatorio cuando el proyecto es un **fork**, está **basado en** otro repo,
plantilla o scaffold, es un **port o adaptación**, o incorpora código vendorizado.

La sección debe responder cinco cosas:

1. qué se tomó;
2. de dónde (link real);
3. quién lo hizo;
4. qué cambió este repositorio;
5. bajo qué licencia estaba el origen.

Ejemplo:

```markdown
## Créditos y agradecimientos

Este proyecto está basado en [Nombre original](https://github.com/owner/repo),
creado por [Autor](https://github.com/autor) bajo licencia MIT.

Qué cambió este repositorio: nueva interfaz de configuración, migración a SQLite
y soporte para Windows. El aviso de copyright original se conserva en
[LICENSE](LICENSE).
```

Reglas duras:

* no borres avisos de copyright del origen;
* conserva `LICENSE`, `NOTICE` o el archivo equivalente que la licencia exija;
* no relicencies a MIT un derivado de GPL/AGPL;
* no presentes como propio un trabajo ajeno;
* no declares "inspirado en" si en realidad se copió código.

Verifica la ascendencia con evidencia real (`git remote -v`, historial de autoría,
`LICENSE`/`NOTICE`, campos de fork en el manifest) antes de escribir la sección:
consulta `references/badges-and-credits.md`.

## Agradecimientos técnicos

Cuando el proyecto reutilice de forma sustancial una skill, una arquitectura o un
proyecto de terceros, escribe una lista breve de agradecimientos con links
verificables. No conviertas la sección en un inventario de dependencias: eso ya
está en el manifest.

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

# Marcas de urgencia en títulos

Usa marcas de urgencia en el título SOLO en secciones donde ignorar el aviso puede
causar pérdida de datos o riesgo de acceso:

```markdown
## 💾 Respaldo y recuperación (¡IMPORTANTE!)

### Credenciales por defecto (¡CAMBIAR!)
```

Reglas:

* máximo 2–3 en todo el README; si todo es urgente, nada lo es;
* nunca en secciones informativas o decorativas;
* la advertencia detrás del título debe ser real y verificable.

---

# Bloques plegables `<details>`

Usa `<details>` para rutas alternativas o de respaldo que no todos necesitan
(instalación manual, plan B si la vía principal falla, scripts opcionales):

```markdown
<details>
<summary>Si el instalador no funciona: instalación manual</summary>

Pasos reales...

</details>
```

Reglas:

* línea en blanco después de `<summary>` y antes de `</details>` para que el markdown interno se renderice;
* el camino principal NUNCA va plegado;
* no escondas advertencias de pérdida de datos dentro de un `<details>`.

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

La fuente de verdad para incluir u omitir secciones es la Matriz de secciones de SKILL.md; esta sección solo da matices.

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

## Skill / Plugin

Priorizar:

* qué hace y cuándo se activa;
* instalación en el agente;
* cómo invocarla (ejemplos reales);
* estructura (`SKILL.md`, `references/`, `agents/`, `commands/`);
* cómo crear o extender una propia;
* compatibilidad con agentes.

Carga `references/skill-projects.md` para el detalle.

---

