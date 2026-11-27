# Interacción con la persona

## Deshacer en vez de confirmar

- Tras un cambio exitoso: `remember("Borraste <cosa>", () => api.restaurar(...))`. El toast dura 6 s con acción y 2.4 s sin ella.
- ⌘Z de la app corre en la webview solo cuando ningún campo de texto se está editando; en campos manda el deshacer nativo.
- `undo()` sin nada: toast "Nada que deshacer". Si el revert falla: "No se pudo deshacer: <motivo>".

## Diálogos

- Estructura: `.dlg-h` (título 16px, botón cerrar ghost) · `.dlg-b` (scroll) · `.dlg-f` (pie con acciones, primaria a la derecha).
- Enter envía; Esc cierra y devuelve el foco al disparador.
- Un atajo de menú con un diálogo abierto no hace nada (salvo ⌘K, que cierra el diálogo y abre la paleta).
- El mismo diálogo crea y edita: abrir con la fila (`openEntry(kind, row)`) o con un borrador prellenado (`draft`).
- Lo que tiene nombre en una tabla abre su edición al hacer clic (`button.edit-name`: texto hasta hover/foco).

## Paleta ⌘K más allá de comandos

- Filtra comandos por texto normalizado (sin acentos, minúsculas).
- Busca entidades por nombre y ofrece "Editar <tipo>: <nombre>"; si el texto empieza con "editar", esos resultados van primero.
- Interpreta frases cortas del dominio con un parser propio y muestra lo que entendió antes de guardar ("uber 120 ayer" → gasto de 120, Uber, ayer).
- Cambia perfil y tema desde ahí.

## Captura rápida

Ventana aparte tipo Spotlight: una línea grande, vista previa en chips de lo que se entendió, pie con ayuda corta. Enter guarda y se oculta; Esc o perder foco la oculta.

## Textos

- Sentence case. Verbo + objeto ("Nuevo gasto", "Programar pago"). Puntos suspensivos cuando abre algo más ("Importar…").
- Plurales correctos ("1 cuenta abierta", "3 cuentas abiertas").
- Errores dicen qué pasó y qué hacer: "Asignaste $500 más de lo que tienes. Registra un ingreso o baja alguna categoría".
- Fallos de red en lenguaje humano: "Sin conexión: no se pudo enviar el reporte".
- Comprobaciones automáticas en segundo plano (actualizaciones al abrir) fallan en silencio; las que pide la persona muestran el error.

## Estados vacíos y primeros pasos

La pantalla de inicio muestra la guía (ejemplo: crear cuenta → registrar ingreso → asignarlo) con pasos que se marcan solos cuando el dato existe. Tablas vacías: sin filas fantasma; una frase y la acción para empezar.

## Densidad y columnas

Ocultar columnas que no aportan (la columna "Cuenta" con una sola cuenta abierta). Nombres largos en una línea con elipsis. Porcentajes y cifras no se parten (`nowrap`).

## Accesibilidad mínima

Nombre accesible en cada botón de icono · `aria-current` en navegación · `role="status"` en avisos vivos · foco visible con `--brand` · `prefers-reduced-motion` apaga transiciones · contraste revisado en claro y oscuro.
