## Simulacion
- [] recuerdro de zona en los PNG.
- [] Cambiar la cobertura a un valor medio, para no spoilear.
- [] Lib que genere imagen superpuesta de simulacion con cobertura arriba.
- [] Porque hardcodeado esas datos? (Los 10cm o 200k hz)
	- Revisar el angulo
		Mismo transductos = diametro.
			- Osea esto en parametro de CONFIG.
	- Cambias SOLO la frecuencia.
- [] Alta frecuena, deberia ser mas chico el angulo.
	10, 15, 20%.
- [] Generar un GEOTIFF de las simulaciones??

## Logger
- [x] Generar la struct
- [x] Generar la logica de thread
- [] Agregarlo en todos lados
	1. Ya agregado en la respuesta del las requests.
	2. __TODO: Pongan aca donde lo fueron agregando, asi ya esta__.

## Modo Examen
- [x] Generar nueva tabla que guarde todos los parametros de simulacion + Fk alumno + Fk proyecto + Booleano de "Entrego este" + Valores estadisticos (maximos, desvios, etc)
- [x] Generar metodos de manipulacion del mismo en la DB. (Get, set, update)
- [] Enpoints para obtenerlos
- [] Que se guarde cada vez que tocas simular.
- [] Enpoint de alumno para seleccionar y modififcar el bool.
- [] Api que entregue el **mapa de diferencias** o ya devolverlo en la simulacion normal.
- [] Agregar el BOOLEAN al proyecto que indique si es examen o no. (Modificar la Api y el front, para enviar ese dato)

## Front
- [] Info primero al entrar a la simulacion.
- [] Mover en login desface de textos.
- [x] Detalles de modales
- [] Modal para ver las simulaciones y poder entregar. __MISMO MODAL PARA EL DOCENTE__
	Es decir, solo deberia de cambiar botones, pero la idea seria codearlo 1 vez, como un compoenente generico, voy a hacer que ande la misma api.
- [] Toggle, para ver o **mapa de diferencias**. Osea que muestre o una foto o la otra, alterna.
- [] Login NO debe redifirigir si pones mal la contrasenia, solo mostrar el mensaje.
- [] Boton para exportar una Lista de `<Nombre grupo, CODIGO>`, asi lo enviar  por slack o algo.
- [] Agregar el MODO EXAMEN o no al crear un proyecto.
- [] Quitar lo de los intentos, si no es modo examen??


## Cache
- [] Separarlo (Por el amor de dios. 🙏)

## Docker
- [] Generar el compose.yaml

## Otros detalles
- [] Mover metodos auxiales fuera de los archivos de endpoints.
	Hay que pensar a donde, para charlar
- [] Yo moveria los archivos de structs dentro server/structs a cada disciplana a la que pertenecnen
	Por ejemplo, para las settings, creo un carpeta.
	El del reqquests, genero en requests/ un structs.rs y asi.
- [] Se chequean los intentos en back? o solo front?

## Consultar a fer o expliquen
1. Que no ocurre en el modo examen? Solamente no deben/pueden entregar?
2. En NO examen: Es cuendo se muestra la comparacion? o es en ambos?