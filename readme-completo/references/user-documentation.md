# Reference — Documentación para usuarios

Consultar cuando el proyecto tenga **usuarios finales** O **operadores o
administradores no técnicos** (personal de oficina, escuela, almacén, soporte).
No consultar para librerías internas, paquetes o proyectos exclusivamente
técnicos cuyo único lector sea un programador.

Regla de uso: este archivo es un **catálogo de piezas**, no una plantilla fija.
Detecta el tipo de proyecto con `repository-inspection.md`, aplica la matriz de
abajo y **toma solo las partes cuya línea `Aplica:` se cumpla**. Quita el resto.
Nunca dejes una sección vacía ni con texto genérico para "rellenar".

## Matriz de aplicabilidad (detecta, luego incluye o quita)

Si una sección va o no en el README lo decide la **Matriz de secciones de
`SKILL.md`**: es la fuente de verdad. Esta tabla no la contradice; solo dice qué
señal buscar en el repo para llenar cada pieza. Las piezas **Casi siempre** se
incluyen salvo evidencia en contra; las **Condicionales** que sean sub-piezas
(advertencia de instalador, recorrido de admin, cierre de sesión) se quitan si
falta su señal. Si la señal es algo que el usuario debería confirmar, marca
`pendiente`.

| Pieza | Nivel | Señal en el repo que la activa |
|---|---|---|
| 9. Instalación usuario final | Casi siempre | Instalador, release, ejecutable, imagen Docker o URL desplegada |
| 9. Advertencia de instalador sin firma | Condicional | Instalador distribuido **y** sin firma de código en config/CI |
| 10. Credenciales iniciales | Condicional | Usuario/clave bootstrap en código, seed o docs |
| 11. Cómo usar (flujos por rol) | Casi siempre | Existe UI o CLI operada por personas |
| 11. Recorrido de admin por pestañas | Condicional | Existe un rol admin con panel de varias secciones |
| 11. Cerrar sesión / expiración | Condicional | Hay login, sesiones o timeout |
| 12. Conceptos importantes | Casi siempre | Alguna regla puede confundir (estados, permisos, público/privado) |
| 13. Importación / exportación | Condicional | Import/export de archivos, migraciones o sincronización |
| 13b. Flujos complejos a fondo | Condicional | Flujo con reglas, estados o varios pasos |
| 13c. Modo prueba / entrenamiento | Condicional | Sandbox, demo, dry-run o modo de práctica |
| 16. Mantenimiento | Condicional | Aplicación operativa con tareas periódicas |
| 17. Actualización | Casi siempre | Hay releases, instaladores o auto-update |
| 17b. Mantener el proyecto limpio | Condicional | Operadores/mantenedores clonan el repo; omitir en distribución solo-binario |
| 18. Troubleshooting | Casi siempre | Hay fallos reproducibles o conocidos |
| 19. GitHub para principiantes | Condicional | Lo mantienen no programadores, estudiantes o un equipo de escuela/oficina |

Casos típicos:

* **App de escritorio interna** (instalador + admin no técnico): casi todo.
* **Web app SaaS pública**: casi todo; 9 se convierte en "cómo entrar o
  registrarse" y 17 en "qué cambia en cada versión".
* **CLI para técnicos**: 11 reducido a comandos; 10 solo si hay credenciales;
  19 solo con evidencia de mantenedores no programadores.
* **Librería/paquete**: no usar este archivo.

---

# 9. Instalación para usuario final

Aplica: siempre que el proyecto se distribuya a personas que no compilan el código.

Separar completamente de instalación para desarrolladores.

Debe explicar:

### Requisitos

Ejemplo:

```markdown
- Windows 10/11
- macOS...
- Docker...
```

Incluye solo requisitos comprobables (manifiesto, config del bundler, docs
oficiales del runtime). Si hay un runtime externo (por ejemplo WebView2,
Java, .NET), enlaza su descarga oficial y di desde qué versión del sistema
viene preinstalado **solo si lo puedes verificar**.

### Instalación

Paso por paso, numerado, sin asumir conocimientos técnicos.

Aplica: cuando existen **dos formas** de conseguir el instalador (página de Releases y copia manual).

Presenta ambas como opciones rotuladas, marcando la recomendada:

```markdown
1. **Consigue el instalador.** Es un archivo que termina en `.exe` / `.msi` / `.dmg` / `.AppImage`.
   - **A) Desde GitHub (recomendado).** Entra a la página de Releases, busca la versión más reciente y, en **Assets**, descarga el archivo.
   - **B) Copia manual** (USB, carpeta compartida): alguien que ya lo tenga te lo pasa.
2. Cópialo a la computadora destino si lo descargaste en otra máquina.
3. Doble clic sobre el instalador y sigue el asistente.
```

Si solo existe una vía, documenta solo esa. No inventes una página de
Releases si el repo no publica releases.

Aplica: solo si el repo **no muestra firma de código** (sin certificado en la config del bundler ni pasos de firma/notarización en CI).

Avisa del warning del sistema operativo antes de que el usuario se asuste:

```markdown
> ⚠️ **Es normal:** el instalador no está firmado con un certificado comercial, así que tu sistema puede mostrar una advertencia.
> - **Windows (SmartScreen, "Windows protegió su PC"):** clic en **Más información** → **Ejecutar de todas formas**.
> - **macOS (Gatekeeper):** clic derecho sobre la app → **Abrir** → confirma **Abrir**.
>
> Descarga el instalador **solo** desde la página oficial de Releases. Si la release publica un checksum SHA-256, comprúebalo antes de abrirlo.
```

Incluye la línea del checksum solo si las releases realmente lo publican; la de
"solo desde Releases" va siempre.

Incluye solo el sistema operativo que el proyecto realmente soporta. Si hay
firma o notarización, **omite el aviso** (un warning falso erosiona confianza).
No digas "es seguro": di por qué aparece el aviso.

Aplica: cuando es verdad que la app funciona sin servidor, sin cuenta y sin configuración extra.

Cierra la instalación con una confirmación explícita:

```markdown
> ✅ Listo. No necesitas internet, no necesitas servidor, no necesitas configurar nada.
```

Úsala solo si el repo lo respalda (app local/offline, base de datos
embebida, sin variables de entorno obligatorias). Si algo sí requiere
configuración, lista esos pasos en su lugar.

### Primer inicio

Aplica: siempre que el primer arranque haga algo o pida algo al usuario.

Explica, en orden:

1. dónde está la app tras instalar (escritorio, menú Inicio, Aplicaciones);
2. qué pasa al abrirla la primera vez (crea base de datos, carga datos de ejemplo, pide cuenta);
3. configuración inicial y permisos solicitados;
4. directorios donde guarda datos;
5. primera cuenta o credenciales (ver sección 10);
6. importaciones iniciales;
7. posibles warnings.

No asumas conocimientos técnicos.

---

# 10. Credenciales iniciales

Aplica: solo si existen de verdad credenciales por defecto o bootstrap.

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

Esta es una de las pocas secciones donde el encabezado puede llevar marcador
de urgencia (`(¡CAMBIAR!)`), porque no cambiarlas expone acceso. Si el
código conserva un acceso de recuperación con la combinación de fábrica aun
después de cambiar la credencial, NO publiques la combinación ni el archivo en
el README: en un repo público eso entrega la puerta trasera de cada
instalación. En el README advierte que existe un acceso de recuperación y que
el equipo debe protegerse físicamente; los detalles van en el reporte final
para el maintainer (o al flujo de `SECURITY.md` si existe).

---

# 11. Cómo usar la aplicación

Aplica: siempre que exista una interfaz usada por personas.

Documenta workflows, no cada botón.

### Un flujo POR ROL

Un flujo por cada rol que el repo distinga (profesor, alumno, operador,
admin, invitado). **El título dice quién y qué quiere lograr.**

```markdown
### Flujo 1: Un operador quiere registrar una salida (Terminal)

![Terminal: catálogo, carrito y pendientes](docs/img/terminal.png)

1. Abre la app y elige **"Soy operador"**.
2. Escribe tu código → Enter.
3. Busca el elemento por nombre o escanéalo.
   - 💡 Si eliges un elemento de tipo A, la app sugiere agregar el accesorio B con un clic.
4. Confirma. El sistema registra fecha y hora.
5. El modal de éxito aparece y **la sesión se cierra sola en 3 segundos**.

> 📋 Esto es **intencional**: la terminal es compartida y no debe quedar una sesión abierta.
```

Reglas del patrón:

* **Pasos numerados**, una acción por paso.
* **Una captura por flujo**, solo si el archivo existe en el repo (`docs/img/`, `assets/`). Sin captura real, no pongas un placeholder: omite la imagen.
* **Tips en línea** con `💡` justo donde ayudan, no en una lista aparte.
* **Notas "esto es intencional"**: cada vez que el código muestre una decisión deliberada que parezca bug (sin PIN para cierto rol, cierre automático de sesión, equipo importado que no aparece para préstamo), explícala en una línea y di el motivo. Verifica el motivo en código o docs; si no lo puedes verificar, no lo expliques.
* Si un flujo tiene estados (`activo`, `vencido`, `cerrado`), lístalos con el umbral exacto (por ejemplo "más de 24 horas") tomado del código.
* Si el flujo tiene reglas largas, déjalo breve y enlaza a "Flujos complejos a fondo" (13b).

Prioriza escenarios reales:

```text
"Un usuario quiere..."
"Un administrador necesita..."
"Quiero recuperar..."
```

sobre describir componentes visuales aislados.

### Recorrido del admin, pestaña por pestaña

Aplica: cuando el rol admin tiene un panel con varias pestañas o secciones.

Una sola lista numerada de acceso y, debajo, **una viñeta por pestaña** con
qué se hace ahí. Las viñetas anidadas cubren filtros, menús de fila y
atajos solo cuando cambian cómo se trabaja:

```markdown
1. Entra a **Administrador** con tu código y tu PIN.
2. Tienes estas pestañas:
   - **Inventario** → alta, edición, detalle; menú de fila (⋮): forzar devolución, eliminar; exportar PDF.
     - 🔎 Tres filtros que se combinan entre sí; el filtro activo se marca en color y hay un botón para limpiarlos.
   - **Categorías** → crear y editar categorías; decidir cuáles se pueden prestar.
   - **Usuarios** → altas, roles, importación desde archivo.
   - **Reportes** → filtrar por fecha, estado o categoría e imprimir.
   - **Configuración** → actualizaciones, respaldos, restauración.
```

Genera la lista **leyendo las pestañas reales del código** (router, menú,
componentes de navegación), no de memoria. Si una pestaña es compleja
(conteos, importaciones), solo nómbrala y enlaza a su sección de 13b.

### Cerrar sesión / expiración

Aplica: cuando hay login o sesión con temporizador.

```markdown
### Flujo N: Cerrar sesión

- En **Terminal** y **Admin** hay un botón **Cerrar sesión** arriba. Úsalo antes de irte.
- En **Terminal** la sesión se cierra automáticamente a las **8 horas**.
```

El tiempo de expiración sale del código. Si no existe, no lo menciones.

---

# 12. Conceptos importantes

Aplica: siempre que exista una regla que pueda confundir al usuario.

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

Aplica: cuando el proyecto lee o escribe archivos, bases externas o migraciones.

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

### Deshacer / revertir

Aplica: siempre que la importación escribe datos.

Di **explícitamente** cuál es el camino de vuelta y dónde se hace. Ejemplo
de fila en la tabla de reglas:

```markdown
| ↩️ Se puede deshacer | Configuración → Respaldos → **Restaurar** el respaldo automático creado al aplicar |
```

Verifica en código que el respaldo previo exista de verdad. Si no existe un
mecanismo de reversión, dilo (`No hay deshacer: haz un respaldo manual antes`)
en lugar de implicarlo.

### Patrón "no es un error"

Aplica: cuando un resultado esperado puede parecer una falla al usuario.

Pon un callout justo después de la regla que lo causa, con el síntoma que
el usuario ve:

```markdown
> ⚠️ **¿Importaste y nadie puede usar los registros nuevos?** No es un error: es intencional. Actívalos en la sección correspondiente.
```

Fórmula: **síntoma que ve el usuario** → "No es un error" → **por qué** →
**qué hacer si quiere otro resultado**. Úsalo solo para comportamientos que
el código realmente implementa así.

---

# 13b. Flujos complejos a fondo

Aplica: cualquier flujo con reglas, estados o varios pasos (importaciones, conteos/auditorías, sincronización, reportes).

Los flujos simples viven en la sección 11. Cuando un flujo tiene reglas que
pueden salir mal, dale su propio `##`, una sección por flujo, con esta
estructura fija (omite un bloque solo si no aplica):

```markdown
## 📋 <Nombre del flujo>

Una frase: qué problema resuelve.

### Dónde está
Ruta de navegación: **Admin ▸ pestaña ▸ botón**.

### Cómo funciona
Explicación breve del modelo mental (3-6 líneas). Qué ve el usuario antes, durante y después.

### Paso a paso
1. ...
2. ...

### Reglas que debes conocer
| Regla | Detalle |
|---|---|
| ... | ... |

### Estados / resultados
| Valor | Significa |
|---|---|
| `S` | ... |
| `N` | ... |
| *(vacío)* | ... |

### Casos borde
- Qué pasa si ...
- Qué pasa si se repite ...

### Tips
- ...

📚 Detalle técnico: [docs/NOMBRE.md](docs/NOMBRE.md)
```

Reglas del patrón:

* **Tabla de reglas**: una fila por regla que cambie el resultado (qué se conserva, qué se sobrescribe, qué nunca ocurre automáticamente).
* **Tabla de estados/resultados**: cada valor que el usuario puede ver en pantalla, en un reporte o en un archivo exportado, y **qué significa cada uno**. Distingue "sin hacer" de "hecho con resultado negativo": **vacío ≠ fallo**. Si un valor vacío se confunde con una falla, añade un callout:

  ```markdown
  > ⚠️ **Por qué importa:** un valor vacío significa "nadie llegó todavía", no "se perdió".
  ```

* **Casos borde**: duplicados, repetir la operación dos veces (¿idempotente?), datos viejos contra nuevos, equivocarse a mitad (¿hay deshacer?), registros que no se pueden resolver solos y quedan listados aparte.
* **Enlace al doc técnico**: si existe `docs/<algo>.md` con el detalle de implementación o formato, enlázalo al final. No enlaces a archivos que no existen.
* Cada fila de cada tabla debe poder rastrearse a código, tests o docs del repo.

### Hardware y periféricos

Aplica: cuando el repo integra lectores, escáneres, impresoras u otros periféricos.

Documéntalo dentro del flujo donde se usa, no en una sección aparte:

* qué dispositivo es y cómo se conecta (teclado emulado, USB, red);
* **qué configuración necesita** y qué hace la app si el dispositivo no está configurado como se esperaba;
* feedback que da la app (sonido, tarjeta de color, destello) y qué significa cada uno;
* qué pasa si hay un campo enfocado frente a si no lo hay.

Ejemplo de explicación honesta: *"La app detecta la ráfaga de teclas por
velocidad, así que funciona aunque el lector no envíe Enter al final"*, solo
si el código implementa esa detección.

---

# 13c. Modo prueba / entrenamiento

Aplica: la app tiene un sandbox, demo, dry-run o modo de práctica.

Documenta, en este orden:

1. **Qué hace**: qué parte del flujo corre completa (tonos, tarjetas, deshacer, validaciones).
2. **Qué NO guarda**: qué escrituras bloquea y cuáles botones quedan deshabilitados con aviso.
3. **Cómo entrar y salir**: ruta o botón exacto.
4. **Indicador visual**: qué se ve mientras está activo (distintivo, barra de color).
5. **Cuándo usarlo**: entrenar a alguien nuevo, probar un cambio.

```markdown
### 🧪 Modo prueba (para entrenar sin miedo)

El botón **"Modo prueba · no guarda nada"** corre el flujo completo; todo se ve y suena igual, pero **ninguna escritura llega a la base de datos**. Mientras está activo aparece un distintivo `Prueba · no se guarda` en la barra superior.

> 💡 Apágalo antes de hacer el trabajo real.
```

Verifica en código **qué** bloquea el modo. No afirmes "no guarda nada" si
alguna operación todavía persiste (logs, caché, preferencias).

---

# 16. Mantenimiento

Aplica: aplicaciones operativas con tareas periódicas.

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

Aplica: siempre que haya releases, instaladores o actualizador automático.

Si hay releases o instaladores:

Documenta:

1. crear backup;
2. cerrar aplicación;
3. instalar actualización;
4. migraciones;
5. validar;
6. rollback.

Comprueba primero cómo funciona realmente el proceso.

### Auto-actualización

Aplica: cuando el repo configura un actualizador (por ejemplo `updater` en la config del bundler, endpoints, clave pública).

Documenta lo que el código demuestra:

* **cuándo busca** (al abrir, cada N horas) y **dónde aparece el aviso** (y dónde no, para no interrumpir tareas críticas);
* que **no descarga ni instala nada sin confirmación**, si es así;
* los pasos del usuario: crear respaldo → confirmar → la app se cierra para instalar → reabrir y ver novedades;
* **actualizaciones firmadas**: solo se instalan paquetes firmados por el repositorio oficial (di esto solo si hay clave pública configurada);
* **buscar a mano**: ruta exacta (Configuración → Actualizaciones → Buscar actualizaciones) y qué muestra (versión instalada);
* **sin internet**: qué dice el panel y que la app sigue funcionando.

```markdown
**La app se actualiza sola. No tienes que bajar nada de GitHub.**

> 🔒 Solo se instalan actualizaciones firmadas por el repositorio oficial.

**Si no hay internet**, el panel lo avisa y la app sigue funcionando normal. Reintenta cuando tengas conexión.
```

### Instalación manual (fallback)

Aplica: siempre que exista auto-actualización; también si solo hay instalación manual.

Si hay actualizador, pon el método manual en un bloque colapsable para no
competir con la ruta principal:

```markdown
<details>
<summary>Instalar a mano (solo si el actualizador no funciona)</summary>

1. **Cierra la app.**
2. **Crea un respaldo** por seguridad.
3. Baja el instalador de Releases e instálalo **encima** (no hace falta desinstalar).
4. Abre la app.

</details>
```

Cierra con un enlace a la sección de respaldo/recuperación si algo sale mal.
Sin actualizador, los pasos manuales van directos, sin `<details>`.

---

# 17b. Mantener el proyecto limpio

Aplica: repos que operadores o mantenedores clonan y compilan. Omitir en distribución solo-binario para usuarios finales.

Documenta solo lo que el `.gitignore`, los scripts y la estructura demuestran:

* **Directorios regenerables** (`node_modules/`, `dist/`, `target/`, `build/`, `.venv/`): se recrean al instalar/compilar, no se suben, y se pueden borrar si pesan mucho.
* **Archivos solo locales** (por ejemplo `*.local.*`, ajustes por máquina, bases de datos reales, `.env`): no se editan ni se suben a Git.
* **Enlace a la guía o script de limpieza**, si existe (`docs/REPO_CLEANUP.md`, `scripts/clean.*`). No inventes uno.

```markdown
- `dist/` y `target/` se **regeneran solos** al compilar y están en `.gitignore`. Si pesan mucho, puedes borrarlos.
- Los archivos de ajustes locales de una computadora **no se editan ni se suben**.

Más detalle en `docs/REPO_CLEANUP.md`.
```

---

# 18. Troubleshooting

Aplica: siempre que existan problemas frecuentes verificables.

Incluye problemas frecuentes verificables.

Formato recomendado:

```markdown
| Problema | Causa probable | Solución |
|---|---|---|
| ... | ... | ... |
```

Nunca inventes soluciones solamente porque "suenan razonables".

Si hay instalador sin firma, la primera fila suele ser el aviso del sistema
operativo (ver sección 9).

---

# 19. GitHub para principiantes

Aplica: proyectos mantenidos por no programadores, estudiantes o un equipo de escuela u oficina.

Si el repositorio está orientado exclusivamente a desarrolladores
experimentados, omite la sección. Si lo mantienen personas que van a
clonar, cambiar y subir cosas, **incluirla es lo esperado**.

Contenido:

### Glosario

Tabla con término, qué es y una analogía cotidiana:

```markdown
| Término | Qué es | Analogía |
|---|---|---|
| **Repositorio** | La carpeta del proyecto en GitHub | Una carpeta compartida con historial |
| **Clone** | Bajar una copia a tu computadora | Fotocopiar el cuaderno completo |
| **Commit** | Guardar un cambio con un mensaje | Una foto del estado del trabajo con una nota |
| **Push** | Subir tus commits a GitHub | Entregar tu copia al equipo |
| **Pull** | Bajar los cambios de otros | Actualizar tu copia con las notas del equipo |
| **Branch** | Una versión paralela del proyecto | Una copia de borrador para probar |
| **PR** | Pedir que revisen tus cambios antes de unirlos | Entregar el borrador al jefe para aprobarlo |
| **`.gitignore`** | Lista de lo que no se sube | La lista de "esto no sale de la oficina" |
```

### Comandos copiables

Pasos numerados con bloques de código listos para copiar, usando el
nombre real del repo y la rama real (`main`, `master`). Cuatro tareas:

1. **Clonar**
2. **Hacer un cambio y subirlo** (`status` → `add` → `commit` → `push`)
3. **Bajar cambios** (`pull`)
4. **Abrir un Pull Request** (crear rama → push → botón de GitHub → revisión)

```bash
git clone https://github.com/<usuario>/<repo>.git
cd <repo>

git status                      # revisa la lista: nada de base de datos, .env ni respaldos
git add <archivo1> <archivo2>   # agrega por nombre lo que cambiaste
git commit -m "feat: descripción corta del cambio"
git push

git pull

git checkout -b feat/nueva-funcion
git push -u origin feat/nueva-funcion
```

Añade una nota con dónde descargar Git si el público no lo tiene instalado.
Usa el shell que corresponda a la audiencia (PowerShell en Windows, bash en macOS/Linux).

### Buenas prácticas

Incluye solo las que apliquen al repo:

* mensajes de commit con la convención que use el proyecto (`feat:`, `fix:`, `docs:`, `chore:`);
* **nunca subas la base de datos real, `.env` ni secretos**; confirma que el `.gitignore` los cubre;
* no subas directorios regenerables (`node_modules/`, `target/`);
* **compila o prueba antes de hacer merge** con el comando real del proyecto.

### CI en dos líneas

Si existen workflows en `.github/workflows/`, explica sin jerga:
qué los dispara (por ejemplo un tag `v*` o ejecución manual), qué hacen
(compilan en una máquina virtual de GitHub y publican el instalador en
Releases) y que, si fallan, GitHub lo marca en rojo. Lee el workflow real;
no describas un CI que no existe.

---

# Reglas de estilo para documentación de usuario

* **Iconos y callouts consistentes en todo el documento**: 💡 para consejos, ⚠️ para riesgos o advertencias, 🚀 para inicio rápido o novedades, ✅ para confirmaciones, 🔒 para seguridad. Mismo significado en cada sección; no mezcles iconos para lo mismo.
* Un icono por encabezado de sección como máximo (📦 instalación, 🚶 uso, 📥 importar, 🧪 prueba), y los mismos en todo el README.
* **Marcadores de urgencia en encabezados** (`¡IMPORTANTE!`, `¡CAMBIAR!`) **solo donde un error pierde datos o expone acceso**: credenciales por defecto, restaurar respaldo que reemplaza la base, subir secretos. En el resto de títulos, no.
* Español neutro con tú, nunca voseo.
* Anti-invención: documenta solo lo que el repo demuestra. Lo que no puedas verificar, omítelo o márcalo como `pendiente`.
