# Inventario de Finanzas (referencia de paridad)

Lo que la app nueva debe igualar. Columna "Equivale a" sirve para traducirlo a otro dominio (ejemplo: una app de aprendizaje).

## Ventanas

| Pieza | Comportamiento | Ejemplo en app de aprendizaje |
|---|---|---|
| Ventana principal | 1280×820, mínimo 900×600, barra de título overlay (semáforos dentro de la sidebar), título oculto | Igual |
| Ventana "quick" | 640×236, sin bordes, siempre encima, fuera del dock, se oculta al perder foco (tipo Spotlight) | Capturar una nota o tarjeta rápida |
| Icono en la barra de menús | Muestra un dato vivo (saldo) como título, el próximo pendiente, captura rápida, abrir, salir. Con bandeja activa, cerrar la ventana solo la oculta | Racha del día / próxima repaso |
| Atajo global | ⌥⇧F abre o cierra la captura rápida desde cualquier app | Igual |

## Sidebar (de arriba a abajo)

1. Marca (solo texto en la fuente display, sin logo en caja ni lema) + botón plegar (⌘S). Zona arrastrable.
2. Botón "Buscar o ejecutar…" con `kbd ⌘K`.
3. Navegación principal: 5–6 pantallas con icono; contador en la que tiene pendientes (vencidos).
4. Sección "Cuentas": cada entidad con avatar/logo, nombre y valor en mono; clic derecho = menú contextual (Abrir, Apariencia…, datos extra).
5. "+ Nueva cuenta".
6. Sección "Cerradas" solo si hay.
7. Espaciador.
8. Tarjeta de actualización (disponible → descargando con barra → lista para reiniciar).
9. Pie: Ajustes + avatar del perfil (abre Ajustes › Perfil).

Plegada: 62px, solo iconos centrados, texto como tooltip y para lectores de pantalla; ventanas angostas (<760px) se pliegan solas; la preferencia se guarda.

## Pantallas

Resumen (inicio con guía de primeros pasos), Movimientos (tabla densa, búsqueda ⌘F), Presupuesto, Próximos pagos, Deudas (opcional, se activa en Ajustes), Deseos/metas, página por cuenta, Ajustes.

Patrón: título de página 24px, una sola acción primaria arriba a la derecha, tira de métricas sin iconos, secciones separadas por líneas.

## Ajustes (pestañas)

| Pestaña | Contenido |
|---|---|
| Perfil | Tu perfil, lista de perfiles (crear, renombrar, eliminar con confirmación) |
| General | Apariencia (Sistema / Claro / Oscuro + 6 paletas con vista previa), Sistema (iniciar al arrancar, icono en barra) |
| Avisos | Recordatorios con hora, botón "probar aviso" |
| Deudas (módulo opcional) | Activar la página |
| Datos | Importar CSV/Excel/PDF, ruta de la base de datos |
| Agentes IA | Estado conectado por agente, configuración copiable del servidor MCP |
| Actualizaciones | Versión, buscar, instalar, notas |
| Fallas y mejoras | Formulario que abre un issue (solo texto, versión y SO; nunca datos) |
| Atajos | Tabla generada del registro de comandos |

## Comandos y atajos

| Grupo | Comandos (atajo) |
|---|---|
| Navegar | Ir a pantalla 1–5 (⌘1–⌘5), Ajustes (⌘,) |
| Acciones | Nuevo (⌘N), Nuevo alterno (⇧⌘N), Captura rápida (⌥⇧F), crear entidades, Importar (⌘I) |
| Editar | Buscar (⌘F), Deshacer último cambio (⌘Z) |
| Ver | Barra lateral (⌘S), Claro/oscuro (⇧⌘L) |
| Ventana | Paleta de comandos (⌘K) |

⌘K además entiende lenguaje corto ("uber 120 ayer" registra, "editar gym" abre para editar), cambia perfil y tema.

## Interacción

- Nada pregunta "¿estás seguro?": todo cambio muestra un toast con "Deshacer" (6 s) y ⌘Z.
- Una sola excepción: eliminar un perfil completo (irreversible) pide confirmación.
- Diálogos nativos `<dialog>`; Enter guarda, Esc cierra; un atajo de menú no abre un diálogo encima de otro.
- Errores en lenguaje humano, con la siguiente acción ("Registra un ingreso o baja alguna categoría").
- Estados vacíos con guía de primeros pasos que se marcan al cumplirse.

## Fuera de la UI

Núcleo en crate Rust compartido por la app y el servidor MCP · perfiles = una base de datos por perfil · actualizaciones firmadas desde GitHub Releases · cask de Homebrew · harness de QA que maneja la app real sobre una base de demo.
