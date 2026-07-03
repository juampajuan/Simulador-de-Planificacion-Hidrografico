use crate::DepthMatrix;
use common::{EcosondaMode, StudentMeasuringParameters};
use crate::processing::geotiff::get_matrix_avg_depth;
use crate::processing::measuring::{find_measuring_points, beam::{calculate_covered_radius, get_points_in_radius, get_points_circular_to_this}, MeasureMode, get_measures};
use crate::structs::measurement_type::MeasurementsType;
use crate::structs::student_measuring_parameters::EchosounderLogic;

pub(super) fn get_covered_points(
    matrix: &DepthMatrix,
    path: &Vec<(usize, usize)>,
    params: &mut StudentMeasuringParameters,
    use_real_depth: bool,
) -> Vec<((usize, usize), f64)> {
    let boat_speed = params.transport_parameters.speed;
    let distance_between_points = boat_speed * params.echo_sounder_parameters.pulse_repetition_interval.recip();
 
    let points_to_measure = find_measuring_points(
        path,
        distance_between_points,
        matrix,
    );
 
    params.echo_sounder_parameters.create_echosounder();
    
    let avg_depth = get_matrix_avg_depth(matrix).unwrap_or(0.0);

    // Si no es profundidad real, el radio es el mismo para todos
    // los puntos (uniforme, en base al promedio).
 
    match params.echo_sounder_parameters.mode {
        EcosondaMode::Monohaz => {
            // Para monohaz mostramos todos los píxeles dentro del círculo del haz,
            // no solo el punto central — así se ve el área real cubierta.
            let mut covered = Vec::new();
            let uniform_radius = if use_real_depth {
                None
            } else {
                Some(calculate_covered_radius(avg_depth, params.echo_sounder_parameters.angle, matrix))
            };
            for &point in &points_to_measure {
                let circle_points = match uniform_radius {
                    Some(radius) => get_points_in_radius(&point, radius, matrix),
                    None => get_points_circular_to_this(&point, params.echo_sounder_parameters.angle, matrix),
                };
                for p in circle_points {
                    covered.push((p, matrix.data[p.1][p.0]));
                }
            }
            covered
        },
        EcosondaMode::Multihaz => {
            let measurements = get_measures(MeasureMode::Perpendicular{ avg_depth: Some(avg_depth) }, matrix, &points_to_measure);
            match measurements {
                MeasurementsType::Multihaz { central_measurments, paralel_measurment_1, paralel_measurment_2 } => {
                    let mut all = central_measurments;
                    all.extend(paralel_measurment_1);
                    all.extend(paralel_measurment_2);
                    all
                },
                MeasurementsType::Monohaz { measurements } => measurements,
            }
        },
    }
}