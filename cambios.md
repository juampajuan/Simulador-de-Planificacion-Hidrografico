## Simulacion
- [x] recuerdro de zona en los PNG.
- [x] Cambiar la cobertura a un valor medio, para no spoilear.
- [x] Lib que genere imagen superpuesta de simulacion con cobertura arriba.
- [x] Porque hardcodeado esas datos? (Los 10cm o 200k hz)
	- Revisar el angulo
		Mismo transductos = diametro.
			- Osea esto en parametro de CONFIG.
	- Cambias SOLO la frecuencia.
- [x] Al no usar sensor inercial la medicion deberia venir mas profunda porque tiene mas trayecto que recorrer.
	10, 15, 20%.

## Logger
- [x] Generar la struct
- [x] Generar la logica de thread
- [] Agregarlo en todos lados
	1. Ya agregado en la respuesta del las requests.
	2. simulation.rs: cache hit/miss de matrix y de path, calculo de recorrido nuevo (Debug), simulacion completada (Info), error al registrar el intento en la DB (Error), imagen de cobertura generada (Debug).
	3. simulations (lib.rs), Cubre: create_depth_matrix, create_path, run_simulation (puntos de medicion, mediciones tomadas, si se aplicaron errores, interpolacion completada), create_path_image, create_simulation_image, create_scale_pure_image, create_path_with_coverage, create_simulation_with_coverage, get_geotiff_corners.
	4. auth.rs: create_professor, change_pass, login, close_all, close_session.
	5. students.rs: create_new_student, get_all_students, delete_a_student, update_an_student.
	6. projects.rs: create, get_projects, delete_project, update_a_project.
	7. exams.rs: get_my_simulations, select_exam_simulation.
	8. files.rs: clean_temp_files. helpers/files.rs: clean_unused_images.
	9. handler.rs: se le paso el tx a todos los endpoints de arriba

Criterio que usamos para elegir nivel: 
- si se dejo un rastro afuera del endpoint (DB/archivo/sesion) -> Info. Si no -> Debug. 
- si el pedido fallo por algo esperado del lado del cliente (permisos, datos invalidos, limite alcanzado) -> Warn. Si el server no pudo hacer algo que en condiciones normales deberia poder -> Error.

## Modo Examen
- [x] Generar nueva tabla que guarde todos los parametros de simulacion + Fk alumno + Fk proyecto + Booleano de "Entrego este" + Valores estadisticos (maximos, desvios, etc)
- [x] Generar metodos de manipulacion del mismo en la DB. (Get, set, update)
- [x] Enpoints para obtenerlos
- [x] Que se guarde cada vez que tocas simular.
- [x] Enpoint de alumno para seleccionar y modififcar el bool.
- [x] Api que entregue el **mapa de diferencias** o ya devolverlo en la simulacion normal.
- [x] Agregar el BOOLEAN al proyecto que indique si es examen o no. (Modificar la Api y el front, para enviar ese dato)
- [x] Fecha limite? se pone cuando se crea, pasado, no se puede entregar.

## Backend
- [x] Api que entrega images, de carpeta de storage.
- [x] Modifico carpeta de ./uploads a ./storage. Ya que se almacenan mas cosas.

## Front
- [x] Info primero al entrar a la simulacion.
- [x] Mover en login desface de textos.
- [x] Detalles de modales
- [x] Interfaz de historial de intentos para el alumno, junto a su selección para entregar.
- [x] Interfaz de entregas obtenidas para el docente.
- [x] Toggle, para ver o **mapa de diferencias**. Osea que muestre o una foto o la otra, alterna.
- [x] Agregar el MODO EXAMEN o no al crear un proyecto.
- [x] Consistencias de borders
- [x] Consistencia de colores
- [x] Modificar el texto ese feo de image viewver en el medio. Cuando hay error. Copiar la ventana de loader, pero ponerla roja(?)
- [x] Mapa se expande.

## Cache
- [x] Separarlo

## Docker
- [x] Generar el compose.yaml

## Otros detalles
- [x] Mover metodos auxiales fuera de los archivos de endpoints.
	Hay que pensar a donde, para charlar
- [] Yo moveria los archivos de structs dentro server/structs a cada disciplana a la que pertenecnen
	Por ejemplo, para las settings, creo un carpeta.
	El del reqquests, genero en requests/ un structs.rs y asi.
- [x] Se chequean los intentos en back y front.
- [x] Agregado modo SIMPLIFIED para los logs de terminal.

## Important
No lo discutimos, pero no lo veo complicado, asi que lo haria.
- [x] Yo haria que se guarde la foto de la simulacion, junto al intento si es Examen.
	Luego, con el nombre, la expongo en un endpoint "url/images/<nombre>". Y asi se peuden mostrar.

	Y agrego un metodo para borrar las NO usadas en el CLI. O veo de generar una tarea periodica (cada unos dias.)

	1. Porque, porque seguro es facil gaurdarla, si ya la generan, Usen un nombre random o fecha actual + 5 letras random.
	2. Se guarda en el student_simulations, como un parametro texto.
	3. Es mucho mas facil para el profe y los alumnos verlas para elegir.

## Consultar a fer o expliquen
1. Que no ocurre en el modo examen? Solamente no deben/pueden entregar?
2. En NO examen: Es cuendo se muestra la comparacion? o es en ambos?


## Usar la carpeta de Helpers
- [x] Mover funciones de los endpoints ahi
- [] Mover funciones de las queries
- [] Mover otros helpers.


# Important Juanjo Front
- [X] Quitar Base64.
- [X] TODO1 en `IMGviewer` linea 24. (Habla de incorporar un booleano, para saber si simulamos, para mostrar la barra de colores, una var extra en ui_state?)
- [X] OPTIONAL: (Evitar si es de alta complejidad) Hacer que el modal de los profes de los intentos, NO proceseleccione la imagen de simulacion.
- [X] TODO3 descomentarlo, eso es quien se encarga de elegir la imagen a mostrar. Porque una vez borrado Base64, nada te va a mostrar al simular.

- [X] Login *****NO***** debe redirigir si pones mal la contrasenia, solo mostrar el mensaje. Porque ahora te la recarga. 
	
	Osea el 403/401 en otras paginas llevan al login, pero en el login, no lo llevas.

Este es un chiche.
- [X] Boton para exportar una Lista de `<Nombre grupo, CODIGO>`, asi lo enviar por slack o algo.
