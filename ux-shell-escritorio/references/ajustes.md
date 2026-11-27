# Ajustes

Layout `.settings`: lista de pestañas de 210px a la izquierda (sticky) y contenido a la derecha; menos de 1000px la lista pasa a fila horizontal con scroll. Cada pestaña es una ruta (`#ajustes/general`) para que comandos, avisos y la bandeja puedan abrir una pestaña concreta.

Cada cambio se guarda al momento (sin botón "Guardar"), y lo que se pueda revertir muestra toast con deshacer.

## Pestañas de referencia

| Pestaña | Secciones (h2) | Reglas |
|---|---|---|
| Perfil | Tu perfil · Tus perfiles | Avatar grande con color; crear, renombrar, cambiar, eliminar. Eliminar es la única confirmación de la app: nombra el perfil y lo que se borra |
| General | Apariencia · Sistema | Modo Sistema/Claro/Oscuro en `.seg`; paletas como tarjetas que se pintan a sí mismas con `data-palette`; iniciar al arrancar; icono en la barra de menús |
| Avisos | Avisos de … | Activar, hora, "Probar aviso" que manda una notificación real |
| Módulos opcionales | Uno por módulo | Switch que muestra u oculta la pantalla y su entrada en la sidebar |
| Datos | Importar · Base de datos | Importar archivos con vista previa; ruta de la base visible y copiable |
| Agentes IA | Agentes MCP | Por agente: conectado (latido reciente) o no, instrucciones y configuración JSON copiable con la ruta real del binario |
| Actualizaciones | Actualizaciones | Versión actual, buscar, instalar, notas de la versión, estado de error legible |
| Fallas y mejoras | Reporta una falla o propón una mejora | Tipo (falla/mejora), título, descripción. Envía solo el texto, la versión y el SO; nunca datos del usuario. Dice qué se envía antes de enviar |
| Atajos | Atajos de teclado | Tabla generada del registro de comandos (no escrita a mano) + Enter/Esc |

Descartar las que no apliquen y anotarlo en el plan.

## Apariencia: detalles

- La elección se guarda en los ajustes de la app (no en `localStorage`) para que la ventana de captura rápida y la bandeja usen el mismo tema.
- `data-theme` en `<html>` fuerza claro u oscuro; sin él manda el sistema.
- Paleta "Sistema" usa el color de acento del SO (ver `tokens.css`).
