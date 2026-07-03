

use common::{
    EchosounderParameters, EcosondaMode, GnssType, StudentMeasuringParameters, Transport,
    TransportParameters,
};

const GEOTIFF_PATH: &str = "uploads/geotiffs/Darsena_20cm_v2_1781927635.tif";

fn main() {
    let matrix = simulations::create_depth_matrix(GEOTIFF_PATH)
        .expect("No se pudo cargar el geotiff — revisa GEOTIFF_PATH");

    let path = simulations::create_path(
        &matrix,
        90.0,              // azimuth_deg
        90.0,               // separation_meters
        GnssType::PhaseCorrection,
    );

    let params = StudentMeasuringParameters {
        echo_sounder_parameters: EchosounderParameters {
            mode: EcosondaMode::Multihaz,
            angle: 40.0,
            absortion_coefficient: 0.0,
            max_limit: 100.0,
            min_limit: 1.0,
            pulse_repetition_interval: 20.0,
            uses_high_frecuency: false,
            transmited_potency: 50.0,
            gain: 12.0,
            threshold: 10.0,
            sound_speed: 1450.0,
        },
        transport_parameters: TransportParameters {
            transport: Transport::Boat,
            speed: 1.0,
            uses_mareograph: false,
            uses_sound_profiler: false,
            uses_inertial_sensor: false,
        },
    };

    let student_interpolation = simulations::run_simulation(&matrix, &path, params);

    let (img, min_val, max_val) =
        simulations::create_simulation_with_coverage(&matrix, &student_interpolation, &path, params);

    println!("Rango de profundidad: {min_val:.2} a {max_val:.2}");

    img.save("preview_coverage.png").expect("No se pudo guardar el PNG");

    println!("Listo: preview_coverage.png");
}