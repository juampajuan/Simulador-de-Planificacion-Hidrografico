use common::{
    EchosounderParameters, EcosondaMode, GnssType, StudentMeasuringParameters, Transport,
    TransportParameters,
};
use simulations::structs::simulation_constants::{SimulationConstants, EchosounderConstants, EnvironmentConstants};

const GEOTIFF_PATH: &str = "storage/geotiffs/Darsena_20cm_v2_1783364203.tif";

fn main() {

    let log_debug = |msg: &str| println!("[DEBUG] {msg}");

    let matrix = simulations::create_depth_matrix(GEOTIFF_PATH, &log_debug)
        .expect("No se pudo cargar el geotiff — revisa GEOTIFF_PATH");

    let path = simulations::create_path(
        &matrix,
        90.0, // azimuth_deg
        90.0, // separation_meters
        GnssType::PhaseCorrection,
        &log_debug,
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

    let constants = SimulationConstants {
        echosounder: EchosounderConstants {
            diameter: 0.20,
            high_freq_hz: 200000.0,
            high_freq_alpha: 0.060,
            low_freq_hz: 20000.0,
            low_freq_alpha: 0.004,
            beam_width_factor: 60.0,
            multihaz_angle_deg: 120.0,
            detection_threshold: 40.0,
            max_gain: 36.0,
        },
        environment: EnvironmentConstants {
            sound_velocity: 1500.0,
            tide_amplitude: 1.5,
            tide_period_h: 12.4,
            tide_phase: 0.0,
        },
    };

    let student_interpolation = simulations::run_simulation(&matrix, &path, params, constants, &log_debug).expect("La simulacion fallo");

    let (img, min_val, max_val) =
        simulations::create_simulation_with_coverage(&matrix, &student_interpolation, &path, params, constants, &log_debug);

    println!("Rango de profundidad: {min_val:.2} a {max_val:.2}");

    img.save("preview_coverage.png").expect("No se pudo guardar el PNG");

    println!("Listo: preview_coverage.png");

    let difference_matrix = simulations::generate_difference_matrix(&matrix, student_interpolation);
    let difference_img = simulations::generate_difference_png(&matrix, difference_matrix);

    difference_img.save("difference.png").expect("No se pudo guardar el PNG");

    println!("Listo: diference.png");
}