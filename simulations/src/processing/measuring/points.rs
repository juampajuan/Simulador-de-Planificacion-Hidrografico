use crate::structs::depth_matrix::DepthMatrix;

pub fn find_measuring_points(
    path: &Vec<(usize, usize)>,
    distance_between_points: f64,
    matrix: &DepthMatrix,
) -> Vec<(usize, usize)> {
    //let distance_between_points_scaled = calculate_effective_measurement_distance(
    //    &distance_between_points,
    //    &matrix.size_x,
    //);

    let mut measuring_points: Vec<(usize, usize)> = Vec::new();
    let mut current_point = &path[0];
    let mut distance_progress: f64 = 0.0;

    measuring_points.push(*current_point);

    for next_point in path {
        let current_distance = calculate_distance_between_points(current_point, next_point, matrix);
        distance_progress += current_distance;

        if distance_progress >= distance_between_points {
            distance_progress -= distance_between_points;
            measuring_points.push(*next_point);
        }
        current_point = next_point;
    }

    measuring_points
}

pub fn calculate_distance_between_points(
    point_a: &(usize, usize),
    point_b: &(usize, usize),
    matrix: &DepthMatrix,
) -> f64 {
    let a_x = point_a.0 as f64 * matrix.size_x;
    let a_y = point_a.1 as f64 * matrix.size_y;
    let b_x = point_b.0 as f64 * matrix.size_x;
    let b_y = point_b.1 as f64 * matrix.size_y;

    ((a_x - b_x).powi(2) + (a_y - b_y).powi(2)).sqrt()
}

fn calculate_effective_measurement_distance(
    distance_between_points: &f64,
    pixel_size: &f64,
) -> f64 {
    let min_pixels = 50.0;
    let min_distance = min_pixels * pixel_size;
    distance_between_points * min_distance / 0.1
}