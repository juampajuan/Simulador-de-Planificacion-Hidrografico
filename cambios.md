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
- [] Generar nueva tabla que guarde todos los parametros de simulacion + Fk alumno + Fk proyecto + Booleano de "Entrego este" + Valores estadisticos (maximos, desvios, etc)
- [] Enpoints para obtenerlos
- [] Que se guarde cada vez que tocas simular.
- [] Enpoint de alumno para seleccionar y modififcar el bool.
- [] Api que entregue el **mapa de diferencias** o ya devolverlo en la simulacion normal.

## Front
- [] Info primero al entrar a la simulacion.
- [] Mover en login desface de textos.
- [x] Detalles de modales
- [] Modal para alumnos, para ver las simulaciones y poder entregar
- [] Toggle, para ver o **mapa de diferencias**. Osea que muestre o una foto o la otra, alterna.
- [] Login NO debe redifirigir si pones mal la contrasenia, solo mostrar el mensaje.
- [] Boton para exportar una Lista de `<Nombre grupo, CODIGO>`, asi lo enviar  por slack o algo.
 

## Cache
- [] Separarlo (Por el amor de dios. 🙏)

## Docker
- [] Generar el compose.yaml