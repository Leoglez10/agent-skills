# Reference — Datos y seguridad

Consultar cuando el proyecto almacene datos, use base de datos, maneje
autenticación, tenga variables sensibles, backups, migraciones, o exponga
servicios o APIs.


# Detección de tipo de proyecto → qué bloques de datos incluir

Antes de escribir las secciones 14, 15 y las nuevas de abajo, clasifica el
proyecto con la evidencia de la inspección (manifiestos, dependencias, código
de acceso a datos, `scripts/`, workflows). Luego toma o quita bloques según la
tabla.

Leyenda: ✅ incluir casi siempre · 🔶 incluir solo si el repo lo prueba · ❌ omitir.

| Bloque | App de escritorio / local-first | App móvil con datos locales | CLI o herramienta con estado local | Web/API con BD propia (servidor) | Web estática / librería / skill sin estado |
|---|---|---|---|---|---|
| 14. Persistencia (ruta, árbol, "no borres") | ✅ | ✅ | ✅ | 🔶 (volúmenes/ruta de BD) | ❌ |
| Qué datos guarda | ✅ | ✅ | 🔶 | ✅ | ❌ |
| 15. Backup y recuperación | ✅ | 🔶 (solo si hay export/import) | 🔶 | ✅ (backup de la BD del operador) | ❌ |
| Respaldo en carpeta sincronizada | ✅ | ❌ | 🔶 | ❌ (usa backups del servidor) | ❌ |
| Varias computadoras | ✅ | ❌ | 🔶 | ❌ (el servidor ya centraliza) | ❌ |
| Scripts para operadores | 🔶 | 🔶 | 🔶 | 🔶 | 🔶 |

Reglas de decisión:

* **Por defecto incluye.** Si la app guarda datos locales que alguien perdería
  al borrar una carpeta, los bloques ✅ y 🔶 de su columna se incluyen casi
  siempre; omite solo con evidencia de que no persiste nada.
* Si el tipo es ambiguo (por ejemplo, app web empaquetada como escritorio),
  clasifica por **dónde viven los datos**: en el equipo del usuario → columna
  local-first; en un servidor → columna web/API.
* Cada subsección empieza con una línea `Aplica: <cuándo>`. Si la condición
  no se cumple, **quita la subsección completa**; no la dejes con "No aplica".
* Dentro de un bloque, quita solo las filas o pasos que el repo no prueba
  (por ejemplo, sin respaldo automático no escribas la subsección de
  respaldo automático).
* No inventes un tipo para justificar un bloque: si no hay evidencia, ❌.

## Marcadores de urgencia en encabezados

Usa etiquetas de urgencia **solo** en secciones donde un error pierde datos
o expone acceso. En el resto, encabezado normal.

```markdown
## 📁 Dónde están guardadas las cosas
## 💾 Respaldo y recuperación (¡IMPORTANTE!)
## 🔐 Seguridad y acceso (¡IMPORTANTE!)
## 🖥️ Trabajar en varias computadoras (¡IMPORTANTE!)
```

* ⚠️ para una advertencia que evita pérdida de datos.
* 🚨 para algo que corrompe datos o expone credenciales.
* No uses estos marcadores en secciones decorativas, de instalación normal
  o de créditos: diluyen la advertencia real.

## Bloques plegables para alternativas

Usa `<details><summary>` para rutas manuales o de respaldo (copiar archivos
sin la app, scripts opcionales, comandos por sistema operativo) y deja visible
la ruta principal, corta.

```markdown
<details>
<summary>Copiar el archivo a mano (sin la app)</summary>

1. Cierra la app.
2. ...

</details>
```

Deja una línea en blanco después de `</summary>` y antes de `</details>` para
que el Markdown interno se renderice. No escondas dentro de un `<details>`
las advertencias ⚠️/🚨: esas siempre quedan visibles.

---

# 14. Persistencia de datos

Aplica: siempre que la app guarde datos locales; omitir solo si no persiste nada.

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

### Ruta por sistema operativo

Aplica: siempre que la app guarde datos locales en el equipo del usuario.

Da una ruta por cada sistema operativo que el proyecto declara soportar.
Obtén cada ruta del código o de la API del framework (por ejemplo, el
directorio de datos de la app que devuelve el framework, el nombre de carpeta
configurado, el identificador de la app). Si solo puedes probar una plataforma,
documenta esa y marca las otras como pendientes.

```markdown
| Sistema | Ruta |
|---|---|
| Windows | `C:\Users\<USUARIO>\AppData\Roaming\<identificador-de-la-app>\` |
| macOS | `~/Library/Application Support/<identificador-de-la-app>/` |
| Linux | `~/.local/share/<identificador-de-la-app>/` |
```

* Reemplaza `<identificador-de-la-app>` por el valor real del repo; no lo
  adivines desde el nombre comercial.
* Explica `<USUARIO>` en una línea para quien no es técnico.
* Si la ruta es configurable por el usuario o por variable de entorno, dilo
  y cita el nombre real de la variable.

### Árbol anotado: qué es crítico, auxiliar y regenerable

Aplica: siempre que la carpeta de datos tenga más de un archivo relevante.

Anota cada archivo o carpeta con su rol. Usa tres categorías y solo las que
el código pruebe:

* **Crítico**: sin esto se pierden datos del usuario.
* **Auxiliar**: lo usa el motor de datos; no se toca a mano mientras la app corre.
* **Regenerable**: la app lo vuelve a crear sola; borrarlo no pierde datos.

```text
<carpeta-de-datos>/
├── app.db            ← CRÍTICO: tus datos
├── app.db-wal        ← AUXILIAR: escritura en curso (no borrar con la app abierta)
├── app.db-shm        ← AUXILIAR: memoria compartida de la base
├── backups/          ← CRÍTICO si es tu única copia: respaldos de la app
├── reports/          ← datos generados por el usuario; trátalos como críticos
└── cache/            ← REGENERABLE: la app lo recrea
```

* Muestra solo archivos que existen en el código o en una instalación real;
  no inventes nombres ni extensiones.
* Si no sabes qué categoría tiene un archivo, déjalo sin etiqueta y no lo
  presentes como borrable.

### Advertencia de pérdida de datos

Aplica: siempre que exista un archivo o carpeta crítica.

Incluye una advertencia explícita, junto al árbol, que nombre qué se pierde
y cómo evitarlo:

```markdown
> ⚠️ **Si borras esa carpeta, pierdes TODO lo que capturaste.**
> Desinstalar la app puede dejar o borrar estos datos: confirma en el código o
> en el instalador cuál de los dos ocurre antes de afirmarlo.
```

* Di qué se pierde en términos del usuario (préstamos, notas, historial), no
  en términos de tablas.
* No afirmes qué hace el desinstalador sin evidencia (configuración del
  instalador). Si no lo sabes, omite esa frase.

### Cómo abrir la carpeta de datos rápido

Aplica: siempre que documentes una ruta de datos.

Prioriza lo que la app ya ofrece; si no existe, da el comando del sistema.

```markdown
**Desde la app:** Configuración → Respaldos → **"Abrir carpeta"**.

<details>
<summary>Sin la app (por sistema operativo)</summary>

* Windows: `Win + R` → escribe `%AppData%\<identificador-de-la-app>` → Enter.
* macOS: Finder → Ir → Ir a la carpeta → pega la ruta.
* Linux: `xdg-open ~/.local/share/<identificador-de-la-app>`

</details>
```

* Menciona el botón o atajo **solo si existe** en el código de la UI.
* No des comandos de sistemas que el proyecto no soporta.

### Qué datos guarda

Aplica: siempre que la app guarde datos locales o tenga base de datos propia.

Tabla pensada para quien no es técnico: no listes tablas SQL, lista lo que la
persona reconoce.

```markdown
| Dato | Qué es | Dónde lo ves en la app |
|---|---|---|
| Clientes | Lista de personas registradas | Pantalla **Clientes** |
| Historial | Operaciones pasadas con fecha | Pestaña **Historial** |
| Configuración | Preferencias de la app | **Ajustes** |
```

* Deriva las filas del esquema real (migraciones, modelos, ORM) y de las
  pantallas reales. Si no puedes ubicar un dato en la UI, deja la celda como
  *pendiente de confirmar*.
* Una fila por concepto que el usuario reconoce, no por tabla técnica.
* Si existe un archivo de esquema o SQL heredado (`schema.sql`,
  `database.sql`) que **no** es la fuente de verdad, dilo y señala dónde vive
  el esquema real:

```markdown
> 🛠 Nota técnica: `database.sql` es una referencia histórica desactualizada.
> El esquema real vive en `<ruta-real-en-el-código>` y se actualiza solo
> cuando se instala una versión nueva (migraciones automáticas).
```

* Solo afirma "migraciones automáticas" si el código las ejecuta al arrancar.
* Si el esquema heredado sí coincide con el código, no escribas la nota.

---

# 15. Backup y recuperación

Obligatoria cuando la pérdida de datos sea relevante.

Aplica: siempre que la app guarde datos locales críticos; omitir solo si no
persiste nada que el usuario no pueda recrear.

Encabezado sugerido en el README: `## 💾 Respaldo y recuperación (¡IMPORTANTE!)`.

Explica:

* crear backup;
* backup automático;
* restaurar;
* qué ocurre durante restore;
* dónde están los backups;
* qué hacer si falla.

Prioriza instrucciones soportadas por la propia aplicación antes que soluciones manuales.

Las subsecciones siguientes detallan cada punto. Quita las que el repo no
pruebe (por ejemplo, sin respaldo automático, quita su subsección y su fila
en la tabla de tipos).

### Resumen en 10 segundos

Aplica: siempre que exista al menos un mecanismo de respaldo.

Abre la sección con un párrafo corto que diga qué pasa solo, dónde queda y qué
tiene que hacer la persona. Solo con hechos probados.

```markdown
**Lo que tienes que saber en 10 segundos:** la app se respalda sola cada
<intervalo> mientras esté abierta y guarda los respaldos en
`<ruta-de-backups>`. Antes de algo grande, crea uno manual con el botón
**"Crear respaldo"**.
```

* Si no hay respaldo automático, dilo con claridad: "esta app **no** se
  respalda sola".
* No prometas "no hay que acordarse de nada" si algo depende de la persona.

### Tipos de respaldo

Aplica: siempre que existan dos o más clases de respaldo (automático, manual,
previo a restaurar, previo a migrar).

Todos suelen vivir en la misma carpeta y se distinguen por el nombre. Obtén
patrón, retención y quién los crea del código.

```markdown
| Tipo | Nombre / patrón del archivo | Quién lo crea | ¿Se borra solo? / retención |
|---|---|---|---|
| 🔄 Automático | `app-auto-AAAA-MM-DD_HH-MM-SS.db` | La app, por intervalo | Sí: se conservan los N más recientes |
| 💾 Manual | `app-backup-AAAA-MM-DD_HH-MM-SS.db` | Tú, con el botón | No: hasta que los borres |
| 🛟 Pre-restauración | `app-pre-restore-AAAA-MM-DD_HH-MM-SS.db` | La app, justo antes de restaurar | No |
```

* Usa los prefijos y el formato de fecha **reales** del código.
* Incluye la fila 🛟 **solo si** la app crea un respaldo de seguridad antes de
  restaurar. Si lo hace, explica que es la red de seguridad contra restaurar
  el archivo equivocado.
* Aclara que la fecha del nombre es la de la computadora y que el formato
  ordena los archivos de más viejo a más nuevo por nombre, solo si es cierto.
* Si hay otros respaldos automáticos (por ejemplo, antes de una importación),
  agrégalos como fila con su evidencia.

### Respaldo automático

Aplica: solo si el código programa respaldos automáticos.

Documenta, con valores del código:

* **intervalo** predeterminado;
* si es **configurable** y dónde (pantalla, archivo de configuración) y con
  qué opciones;
* cada cuánto la app revisa si toca respaldar, si aplica;
* **retención** (cuántos conserva y cuáles borra);
* si se puede apagar y si se recomienda;
* **cómo comprobar que corrió** (carpeta de respaldos, fecha del último
  archivo, indicador en la UI).

```markdown
> ⚠️ **La app tiene que estar abierta.** No hay ningún servicio en segundo
> plano: si la computadora está apagada o la app cerrada, no se genera
> respaldo.
```

* Incluye la advertencia "solo corre si la app está abierta" **únicamente**
  cuando el código lo confirme (temporizador dentro de la app, sin servicio
  ni tarea programada del sistema). Si hay un servicio o tarea programada,
  descríbelo en su lugar.

### Crear un respaldo a mano desde la app

Aplica: solo si la app tiene un botón, comando o menú de respaldo.

```markdown
1. Entra a **<pantalla>** → **<sección>**.
2. Pulsa **"<botón>"**.
3. Se guarda en `<ruta-de-backups>` con el nombre `<patrón>`.
   **"Abrir carpeta"** te lleva ahí directo.
```

* Usa los textos exactos de la UI (búscalos en el código o en las
  traducciones).
* Recomienda cuándo hacerlo (antes de actualizar, importar datos masivos,
  cerrar un ciclo), solo si es coherente con lo que hace la app.

### Copiar el archivo a mano (sin la app)

Aplica: datos locales en un archivo o carpeta que se pueda copiar. Va
plegado como alternativa.

```markdown
<details>
<summary>Copiar el archivo a mano (sin la app)</summary>

1. **Cierra la app** (si está abierta, la copia puede quedar incompleta).
2. Ve a `<ruta-de-datos>`.
3. Copia `<archivo-crítico>` a un lugar seguro (USB, otra carpeta, la nube).
4. Si hay archivos auxiliares (`-wal`, `-shm`), cópialos juntos o cierra la
   app limpiamente para que se integren antes de copiar.

</details>
```

* No des este paso con la app abierta si el motor de datos usa archivos
  auxiliares: sin evidencia de lo contrario, indica cerrar la app.

### Restaurar

Aplica: solo si la app permite restaurar o importar un respaldo.

```markdown
1. Entra a **<pantalla>** → **<sección>**.
2. En la lista de respaldos pulsa **"Restaurar"** en el que quieras, o usa
   **"Importar respaldo"** para traer un archivo de una USB o de la nube.
3. La app valida el archivo, <crea un respaldo previo>, reemplaza la base
   actual y <limpia auxiliares>.
4. Reinicia la app.
```

Describe en "Qué ocurre durante restore" solo los pasos que el código hace:
validar, crear respaldo previo, sobrescribir, limpiar auxiliares, pedir
reinicio.

```markdown
> ⚠️ **Restaurar reemplaza TODA la base, no la mezcla.** Lo capturado
> después de la fecha de ese respaldo se pierde.
```

* Incluye esa advertencia **solo** si el restore sobrescribe (verifícalo en
  el código). Si la app fusiona, explícalo en su lugar.
* **Validación del archivo:** si el código comprueba el formato (por ejemplo,
  encabezado del archivo, versión, integridad), dilo en una línea para el
  usuario ("si eliges un archivo que no es de respaldo, se rechaza sin
  tocar nada"). Sin evidencia, no lo afirmes.
* Si existe el respaldo previo a restaurar, indica cómo volver atrás:
  restaurar ese archivo.

### Dónde están los backups

Aplica: siempre que exista algún respaldo.

Da la ruta por sistema operativo (misma regla que en la ruta de datos) y
aclara si la ruta es configurable. Si los respaldos viven **dentro** de la
carpeta de datos, dilo: borrar esa carpeta borra también los respaldos
(por eso conviene una copia externa).

### Qué hacer si falla

Aplica: siempre que documentes respaldos.

Lista solo fallos con evidencia (mensajes de error del código, validaciones,
logs) y qué hacer:

* el respaldo no aparece → comprobar que la app estuvo abierta, permisos de
  la carpeta, espacio en disco;
* restaurar rechaza el archivo → comprobar que es un respaldo de esta app;
* algo quedó mal tras restaurar → usar el respaldo previo, si existe.

No inventes mensajes de error ni códigos: cita los del código o describe el
síntoma sin comillas.

---

# Respaldo en una carpeta sincronizada en la nube

Aplica: datos locales críticos sin backend propio.

Quita este bloque si hay servidor con sus propios respaldos o si la app no
tiene carpeta de respaldos configurable o fija. No lo presentes como
requisito de la app: es una forma opcional de tener copia fuera del equipo.

Usa el proveedor genérico ("Drive, OneDrive, Dropbox o similar") salvo que el
repo documente uno concreto.

```markdown
**Cómo se configura (una sola vez, por computadora):**

1. Instala el cliente de escritorio de tu servicio (Drive, OneDrive, Dropbox
   o similar) e inicia sesión con la cuenta que corresponda (cuenta de la
   organización si es una app de trabajo, no una personal).
2. Abre sus preferencias → agrega o sincroniza una carpeta.
3. Elige **la carpeta de respaldos**: `<ruta-de-backups>`.
4. Confirma que la sincronización quedó activa.
```

Si la app permite cambiar la carpeta de respaldos desde su configuración,
ofrece la variante: "apunta la carpeta de respaldos de la app a una carpeta
que ya sincroniza el servicio". Sin esa opción en el código, usa la primera.

```markdown
> 🚨 **Nunca sincronices la carpeta de la base de datos viva (`<ruta-de-datos>`).**
> El servicio sube archivos enteros sin saber si la app está escribiendo y
> puede **corromper la base**. Sincroniza solo la subcarpeta de respaldos
> (`<ruta-de-backups>`) o una carpeta aparte.
```

* Escribe 🚨 solo si la base es un archivo vivo (SQLite u otro motor
  embebido con archivos abiertos). Verifícalo.
* Si hay carpetas hermanas que también quieras en la nube (reportes,
  exportaciones), dilo y recuerda agregarlas **por separado**, no la carpeta
  padre.

Documenta también:

* **Qué pasa después:** cada respaldo nuevo se sube solo, normalmente en
  segundos o minutos, según el servicio.
* **Dónde encontrarlos:** ruta en la web del servicio (sección de equipos,
  carpeta sincronizada). Usa el nombre real de la sección solo si lo
  conoces; si no, "en la carpeta sincronizada de tu cuenta".
* **Cómo restaurar desde ahí:** descarga el archivo y úsalo con la función de
  importar de la app (solo si existe).

```markdown
> ⚠️ **Espejo no es archivo histórico.** Si la app borra un respaldo viejo
> (por retención), el servicio también lo borra de la nube. Para conservar uno,
> crea uno **manual** o cópialo a otra carpeta de la nube.
> ⚠️ **Sin internet no hay subida.** El servicio sube todo al volver la
> conexión; el respaldo local se crea igual.
```

* Incluye "espejo ≠ archivo" solo si el servicio y la app borran por
  retención (aplica al modo de sincronización de carpeta espejo). Sin
  retención automática, omítelo.

---

# Trabajar en varias computadoras

Aplica: app local sin servidor que pueda usarse en más de un equipo.

Quita este bloque si hay servidor central o si la app es de un solo equipo
por diseño. Empieza diciendo si la app **fusiona** datos o **reemplaza**:

```markdown
La app no fusiona dos bases: **restaurar reemplaza la base completa**. Por eso
el método es de relevo: una computadora a la vez, como pasarse una estafeta.
```

```markdown
> ⚠️ **Regla no negociable: solo una computadora activa a la vez.**
> Si dos equipos capturan el mismo día, el último en importar borra el
> trabajo del otro y no hay forma de recuperarlo, salvo el respaldo previo.
```

Pasos del relevo (por USB o transferencia de archivos). Usa las funciones
reales de la app (crear respaldo, abrir carpeta, importar):

```markdown
**Ida (equipo A → equipo B):**

1. Instala la app en el equipo B (mismo instalador).
2. En A: **"Crear respaldo"** → **"Abrir carpeta"**.
3. Copia el archivo recién creado a una USB (o envíalo por el medio que uses).
4. En B: **"Importar respaldo"** y elige ese archivo.
5. Reinicia la app en B y trabaja normal.

**Vuelta (equipo B → equipo A):**

6. En B: **"Crear respaldo"** → copia el archivo.
7. En A: **"Importar respaldo"** con ese archivo. Reinicia.

Desde ese momento A vuelve a ser el equipo activo; B queda desactualizado y
no se captura nada ahí hasta el siguiente relevo.
```

* Si la app **sí** fusiona (importar combina registros, resuelve
  conflictos), añade una subsección "Variante con fusión" que explique qué se
  combina, cómo se resuelven duplicados y qué se pierde. **Solo** con
  evidencia en el código. Si no, no la escribas.
* Si hay carpeta sincronizada (bloque anterior), menciona que puede
  reemplazar la USB, pero que la regla de "una computadora activa" sigue.
* Datos que no viajan con el respaldo (reportes, archivos adjuntos en otra
  carpeta): menciónalos solo si el código los guarda fuera del archivo de
  respaldo.

---

# Scripts de utilidad para operadores

Aplica: existen scripts en `scripts/` u otro directorio que un operador use
(respaldo, restauración, migración, diagnóstico).

Quita este bloque si no hay scripts o si son solo de desarrollo (build,
lint), que van en la documentación de desarrollo.

Documenta cada script **que exista**:

```markdown
### Respaldo automatizado con scripts (opcional)

Se ofrecen scripts en `scripts/` para quien prefiera la línea de comandos.
**No son necesarios** si usas la app.

| Script | Para qué sirve | Cómo se ejecuta |
|---|---|---|
| `scripts/<nombre>` | <propósito probado en el código> | `<comando exacto>` |

**Requisitos:** <intérprete y versión, dependencias, permisos>.
```

* Lee cada script antes de describirlo: propósito, argumentos, qué escribe,
  si valida (checksum, integridad), si crea copia previa.
* Marca siempre el bloque como **opcional** si la app cubre lo mismo.
* Enlaza la guía existente (`docs/...`) si la hay; no la dupliques.
* No inventes scripts, flags ni salidas. Si no puedes ejecutar un script,
  descríbelo solo por lo que dice su código y no prometas resultados.

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

