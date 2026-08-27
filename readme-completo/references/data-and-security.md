# Reference — Datos y seguridad

Consultar cuando el proyecto almacene datos, use base de datos, maneje
autenticación, tenga variables sensibles, backups, migraciones, o exponga
servicios o APIs.


# 14. Persistencia de datos

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

---

# 15. Backup y recuperación

Obligatoria cuando la pérdida de datos sea relevante.

Explica:

### Crear backup

### Backup automático

### Restaurar

### Qué ocurre durante restore

### Dónde están los backups

### Qué hacer si falla

Prioriza instrucciones soportadas por la propia aplicación antes que soluciones manuales.

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

