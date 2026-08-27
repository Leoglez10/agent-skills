# Reference — Estilo de redacción

Consultar durante la redacción final: lenguaje claro, badges, callouts, tablas,
screenshots, emojis, tono, ejemplos y eliminación de lenguaje genérico de IA.


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

