# Reference — Checklist de presentación

Cada área vale 1 punto. Marca `OK`, `Mejorable` o `Falta`, con la evidencia.

## 1. Descripción About

Detalle: `topics-and-about.md`.

- Una frase que responde "qué hace y para quién".
- Mal: "My project". Bien: "CLI que sugiere rutas e historial de zsh sin IA".
- Comando: `gh repo edit --description "..."`.

## 2. Website

- Link a GitHub Pages, deploy, docs o página de la herramienta.
- Si no existe ninguno, marcar `N/A`, no inventar.
- Comando: `gh repo edit --homepage "https://..."`.

## 3. Topics

Detalle: `topics-and-about.md`.

- 5–10 topics útiles: lenguaje, framework, dominio, tipo de herramienta.
- Evitar tendencias sin relación (`ai`, `awesome`) si el código no lo respalda.
- Comando: `gh repo edit --add-topic a,b,c` (y `--remove-topic` para los incorrectos).

## 4. Social preview

- Imagen 1280×640 px (mínimo 640×320), PNG/JPG, <1 MB.
- Contenido: nombre, frase del About y una captura o ícono.
- Sin API: Settings → General → Social preview → Edit → Upload.
- Puedes generar la imagen localmente (HTML + captura) si el usuario lo aprueba.

## 5. Demo visual

- Captura o GIF arriba del README, que muestre el resultado real.
- Guardar en `docs/` o `.github/assets/`, no en la raíz.
- Si falta, recomendar `readme-completo` para integrarlo.

## 6. Licencia

- `LICENSE` en la raíz y detectada por GitHub (`licenseInfo` no nulo).
- Si falta, preguntar cuál; no elegir por el usuario.

## 7. Releases

- Si hay versión en el manifest o tags y no hay releases → proponer:
  `gh release create vX.Y.Z --generate-notes`.
- Sin versión ni tags → `N/A` para proyectos personales; sugerir para librerías.

## 8. Archivos de comunidad

Solo si el proyecto acepta contribuciones:

| Archivo | Ubicación |
|---|---|
| `CONTRIBUTING.md` | raíz o `.github/` |
| `CODE_OF_CONDUCT.md` | raíz o `.github/` |
| `SECURITY.md` | raíz o `.github/` (pedir contacto real) |
| Plantillas de issue | `.github/ISSUE_TEMPLATE/*.yml` |
| Plantilla de PR | `.github/pull_request_template.md` |

Proyecto personal sin colaboradores → `N/A`, no inflar.

## 9. Higiene visible

- Sin basura versionada: `.DS_Store`, `node_modules/`, `.env`, builds.
- `.gitignore` acorde al stack.
- Comando de chequeo: `git ls-files | grep -E '\.DS_Store|node_modules/|\.env$'`.

## 10. Perfil (opcional)

- Repo fijado en el perfil si es de los destacados (manual: perfil → Customize your pins).
- README de perfil en `OWNER/OWNER`.

## Puntaje

`N/10` = áreas OK / áreas aplicables (excluye `N/A`). Reportar el denominador real.
