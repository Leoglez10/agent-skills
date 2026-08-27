# Reference — Documentación para usuarios

Consultar cuando el proyecto tenga usuarios finales o interfaz de usuario.
No consultar para librerías internas, paquetes o proyectos exclusivamente
técnicos, salvo que exista una audiencia de usuario final.


# 9. Instalación para usuario final

Separar completamente de instalación para desarrolladores.

Debe explicar:

### Requisitos

Ejemplo:

```markdown
- Windows 10/11
- macOS...
- Docker...
```

### Instalación

Paso por paso.

### Primer inicio

Explica:

* configuración inicial;
* permisos;
* directorios;
* primera cuenta;
* importaciones;
* posibles warnings.

No asumas conocimientos técnicos.

---

# 10. Credenciales iniciales

Solamente si existen de verdad.

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

---

# 11. Cómo usar la aplicación

Documenta workflows, no cada botón.

Ejemplo:

```markdown
### Flujo 1: Crear un préstamo

1. ...
2. ...
3. ...

### Flujo 2: Devolver equipo

1. ...
2. ...
```

Prioriza escenarios reales:

```text
"Un usuario quiere..."
"Un administrador necesita..."
"Quiero recuperar..."
```

sobre describir componentes visuales aislados.

---

# 12. Conceptos importantes

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

---

---

# 16. Mantenimiento

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

Si hay releases o instaladores:

Documenta:

1. crear backup;
2. cerrar aplicación;
3. instalar actualización;
4. migraciones;
5. validar;
6. rollback.

Comprueba primero cómo funciona realmente el proceso.

---

# 18. Troubleshooting

Incluye problemas frecuentes verificables.

Formato recomendado:

```markdown
| Problema | Causa probable | Solución |
|---|---|---|
| ... | ... | ... |
```

Nunca inventes soluciones solamente porque "suenan razonables".

---

# 19. GitHub para principiantes

Solo incluir si la audiencia del proyecto lo justifica.

Puede explicar:

* repo;
* clone;
* commit;
* push;
* pull;
* branch;
* PR.

Si el repositorio está orientado exclusivamente a desarrolladores experimentados, esta sección probablemente sobra.

---

