use crate::processing::interpolation::helpers::create_matrix_with_measurments_and_eliminate_none_points;
use crate::processing::interpolation::interpolation_method::old_interpolations::idw::interpolation_idw_kdtrees;
use crate::processing::interpolation::interpolation_method::old_interpolations::kriging::interpolation_kriging;
use crate::processing::interpolation::interpolation_method::old_interpolations::tin::interpolation_tin;
use crate::processing::interpolation::interpolation_method::tin_gdal_grid::interpolation_gdal_tin;
use crate::structs::depth_matrix::DepthMatrix;
use crate::structs::interpolation_type::InterpolationMethod;
use crate::structs::measurement_type::MeasurementsTypeWithError;


pub fn interpolate(
    method: InterpolationMethod,
    measuring_points: MeasurementsTypeWithError,
    geotiff: &DepthMatrix,
) -> Vec<Vec<f64>> {

    let (new_points, new_matrix) = match measuring_points {
        MeasurementsTypeWithError::Monohaz { measurements } => {
            create_matrix_with_measurments_and_eliminate_none_points(&measurements, geotiff)
        },
    
        MeasurementsTypeWithError::Multihaz { central_measurments, paralel_measurment_1, paralel_measurment_2 } => {
            let (points_central, matrix_central) = create_matrix_with_measurments_and_eliminate_none_points(&central_measurments, geotiff);
            let (points_left,    matrix_left)    = create_matrix_with_measurments_and_eliminate_none_points(&paralel_measurment_1, geotiff);
            let (points_right,   matrix_right)   = create_matrix_with_measurments_and_eliminate_none_points(&paralel_measurment_2, geotiff);

            let mut new_matrix = vec![vec![0.0; geotiff.width]; geotiff.height];

            for &(x, y) in &points_central {
                new_matrix[y][x] = matrix_central[y][x];
            }
            for &(x, y) in &points_left {
                new_matrix[y][x] = matrix_left[y][x];
            }
            for &(x, y) in &points_right {
                new_matrix[y][x] = matrix_right[y][x];
            }

            let mut new_points = points_central;
            new_points.extend(points_left);
            new_points.extend(points_right);

            (new_points, new_matrix)
        },
    };

    match method {
        InterpolationMethod::Idw     => interpolation_idw_kdtrees(&new_points, &new_matrix, geotiff),
        InterpolationMethod::Kriging => interpolation_kriging(&new_points, &new_matrix, geotiff),
        InterpolationMethod::Tin     => interpolation_tin(&new_points, &new_matrix, geotiff),
        InterpolationMethod::GdalTin => {
            match interpolation_gdal_tin(&new_points, &new_matrix, geotiff) {
                Ok(result) => result,
                Err(e) => {
                    println!("gdal_grid falló ({e}), usando TIN como fallback");
                    interpolation_tin(&new_points, &new_matrix, geotiff)
                }
            }
        }
    }
}



