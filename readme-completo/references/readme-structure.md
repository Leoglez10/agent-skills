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

