mod processing;
mod structs;

use common::{Transport, EcosondaMode, EchosounderParameters, StudentMeasuringParameters, TransportParameters};
use structs::student_measuring_parameters::EchosounderLogic;

fn main() {
    let geotiff_path = "Darsena_20cm_v2.tif";

    // --- Matriz de profundidades ---
    let matrix = match processing::geotiff::processing_geotiff(geotiff_path) {
        Ok(m) => m,
        Err(e) => { println!("Error al leer el GeoTIFF: {}", e); return; }
    };

    // --- Recorrido ---
    let path = processing::routing::generate_route(
        &matrix,
        40.0,  // azimut
        10.0,  // separación en metros
        1.0,   // offset GNSS
    );

    // --- Parámetros del alumno ---
    let mut echo = EchosounderParameters {
        mode: EcosondaMode::Monohaz,
        angle: 0.0,
        absortion_coefficient: 0.0,
        max_limit: 100.0,
        min_limit: 0.0,
        pulse_repetition_interval: 100.0,
        pulse_length: 1,
        uses_high_frecuency: true,
        transmited_potency: 220.0,
        gain: 0.0,
        echosounder_velocity: 1500,
        threshold: 0.1,
    };
    echo.create_echosounder();

    let params = StudentMeasuringParameters {
        echo_sounder_parameters: echo,
        transport_parameters: TransportParameters {
            transport: Transport::Ship,
            speed: 1.0,
            uses_mareograph: false,
            uses_sound_profiler: true,
            uses_inertial_sensor: false,
        },
    };

    let boat_speed = params.transport_parameters.speed;
    let distance_between_points =boat_speed * echo.pulse_repetition_interval/1000.0;
    // --- Puntos de medición ---
    let points_to_measure = processing::measuring::find_measuring_points(&path, distance_between_points, &matrix);

    // --- Mediciones ideales ---
    let measurements_ideal = processing::measuring::get_measures(
        processing::measuring::MeasureMode::Circular { angle: echo.angle },
        &matrix,
        &points_to_measure,
        echo.threshold,
    );

    // --- Mediciones sin errores (todos los parámetros en true) ---
    let mediciones_ideales: Vec<((usize, usize), f64)> = points_to_measure
        .iter()
        .map(|&p| (p, measurements_ideal[p.1][p.0]))
        .collect();

    let mediciones_observadas = processing::measuring::apply_disturbances(
        mediciones_ideales,
        &path,
        &params,
        &matrix,
    );

    // --- Reconstruir grilla e interpolar ---
    let mut measurements_final = vec![vec![0.0f64; matrix.width]; matrix.height];
    let mut points_validos: Vec<(usize, usize)> = Vec::new();
    for (punto, z_obs) in &mediciones_observadas {
        if let Some(z) = z_obs {
            measurements_final[punto.1][punto.0] = *z;
            points_validos.push(*punto);
        }
    }

    let interpolacion = processing::interpolation::interpolate(
        processing::interpolation::InterpolationMethod::Idw,
        &points_to_measure,
        &measurements_ideal,
        &matrix,
    );

    // --- Guardar imágenes ---
    let img_path = processing::images::makepng_with_matrix_and_path(&matrix, &path);
    img_path.save("recorrido.png").expect("Error al guardar recorrido.png");
    println!("recorrido.png guardado");

    let img_sim = processing::images::makepng_with_matrix_and_interpolation(&interpolacion, &matrix);
    img_sim.save("simulacion.png").expect("Error al guardar simulacion.png");
    println!("simulacion.png guardado");
}