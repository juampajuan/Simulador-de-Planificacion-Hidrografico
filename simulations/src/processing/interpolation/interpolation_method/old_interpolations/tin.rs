use crate::processing::interpolation::interpolation_method::old_interpolations::old_helpers::build_kdtree;
use crate::structs::depth_matrix::DepthMatrix;
use spade::{DelaunayTriangulation, HasPosition, Point2, PositionInTriangulation, Triangulation};

#[derive(Clone, Copy, Debug)]

// ------------------------------------------------------------
//  Implementacion Vieja de Tin, no se usa
// ------------------------------------------------------------
struct TinVertex {
    position: Point2<f64>,
    depth: f64,
}

impl HasPosition for TinVertex {
    type Scalar = f64;
    fn position(&self) -> Point2<f64> {
        self.position
    }
}

pub fn interpolation_tin(
    measuring_points: &[(usize, usize)],
    matrix: &[Vec<f64>],
    geotiff: &DepthMatrix,
) -> Vec<Vec<f64>> {
    let no_data = geotiff.no_data.unwrap_or(f64::MAX);
    let mut result = vec![vec![0.0_f64; geotiff.width]; geotiff.height];

    let (_kdtree, depth_values, indices) = build_kdtree(measuring_points, matrix, no_data);

    if depth_values.is_empty() {
        return result;
    }

    let mut triangulation: DelaunayTriangulation<TinVertex> = DelaunayTriangulation::new();

    for (i, &idx) in indices.iter().enumerate() {
        let point = measuring_points[idx];
        let vertex = TinVertex {
            position: Point2::new(point.0 as f64, point.1 as f64),
            depth: depth_values[i],
        };
        let _ = triangulation.insert(vertex);
    }

    if triangulation.num_inner_faces() == 0 {
        return result;
    }

    for j in 0..geotiff.height {
        for i in 0..geotiff.width {
            if geotiff.data[j][i] == no_data {
                result[j][i] = no_data;
                continue;
            }
            if matrix[j][i] != 0.0 {
                result[j][i] = matrix[j][i];
                continue;
            }

            let query = Point2::new(i as f64, j as f64);

            match triangulation.locate(query) {
                PositionInTriangulation::OutsideOfConvexHull(_) => {
                    result[j][i] = no_data;
                }
                _ => {
                    if let Some(z) = triangulation
                        .natural_neighbor()
                        .interpolate(|v| v.data().depth, query)
                    {
                        result[j][i] = z;
                    }
                }
            }
        }
    }

    result
}
