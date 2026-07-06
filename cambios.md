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
- [] Generar un GEOTIFF de las simulaciones??

## Logger
- [x] Generar la struct
- [x] Generar la logica de thread
- [] Agregarlo en todos lados
	1. Ya agregado en la respuesta del las requests.
	2. __TODO: Pongan aca donde lo fueron agregando, asi ya esta__.
		simulation.rs: cache hit/miss de matrix y de path, calculo de recorrido nuevo (Debug), simulacion completada (Info), error al registrar el intento en la DB (Error), imagen de cobertura generada (Debug).
	3. simulations (lib.rs), Cubre: create_depth_matrix, create_path, run_simulation (puntos de medicion, mediciones tomadas, si se aplicaron errores, interpolacion completada), create_path_image, create_simulation_image, create_scale_pure_image, create_path_with_coverage, create_simulation_with_coverage, get_geotiff_corners.

## Modo Examen
- [x] Generar nueva tabla que guarde todos los parametros de simulacion + Fk alumno + Fk proyecto + Booleano de "Entrego este" + Valores estadisticos (maximos, desvios, etc)
- [x] Generar metodos de manipulacion del mismo en la DB. (Get, set, update)
- [x] Enpoints para obtenerlos
- [x] Que se guarde cada vez que tocas simular.
- [x] Enpoint de alumno para seleccionar y modififcar el bool.
- [] Api que entregue el **mapa de diferencias** o ya devolverlo en la simulacion normal.
- [x] Agregar el BOOLEAN al proyecto que indique si es examen o no. (Modificar la Api y el front, para enviar ese dato)

## Front
- [x] Info primero al entrar a la simulacion.
- [] Mover en login desface de textos.
- [x] Detalles de modales
- [x] Interfaz de historial de intentos para el alumno, junto a su selección para entregar.
- [] Interfaz de entregas obtenidas para el docente.
- [] Toggle, para ver o **mapa de diferencias**. Osea que muestre o una foto o la otra, alterna.
- [] Login NO debe redifirigir si pones mal la contrasenia, solo mostrar el mensaje.
- [] Boton para exportar una Lista de `<Nombre grupo, CODIGO>`, asi lo enviar  por slack o algo.
- [x] Agregar el MODO EXAMEN o no al crear un proyecto.

## Cache
- [x] Separarlo

## Docker
- [x] Generar el compose.yaml

## Otros detalles
- [] Mover metodos auxiales fuera de los archivos de endpoints.
	Hay que pensar a donde, para charlar
- [] Yo moveria los archivos de structs dentro server/structs a cada disciplana a la que pertenecnen
	Por ejemplo, para las settings, creo un carpeta.
	El del reqquests, genero en requests/ un structs.rs y asi.
- [x] Se chequean los intentos en back y front.

## Important
No lo discutimos, pero no lo veo complicado, asi que lo haria.
- [] Yo haria que se guarde la foto de la simulacion, junto al intento si es Examen.
	Luego, con el nombre, la expongo en un endpoint "url/images/<nombre>". Y asi se peuden mostrar.

	Y agrego un metodo para borrar las NO usadas en el CLI. O veo de generar una tarea periodica (cada unos dias.)

	1. Porque, porque seguro es facil gaurdarla, si ya la generan, Usen un nombre random o fecha actual + 5 letras random.
	2. Se guarda en el student_simulations, como un parametro texto.
	3. Es mucho mas facil para el profe y los alumnos verlas para elegir.

## Consultar a fer o expliquen
1. Que no ocurre en el modo examen? Solamente no deben/pueden entregar?
2. En NO examen: Es cuendo se muestra la comparacion? o es en ambos?