use crate::structs::depth_matrix::DepthMatrix;

// Genera la ruta dada por la separacion indicada por el usuario.
pub fn generate_route(separation: usize, matrix: &DepthMatrix) -> Vec<(usize, usize)> {
    let mut path_points = Vec::new();
    let mut y = 0;
    let mut left_to_right = true;
    let width = matrix.width;
    let height = matrix.height;

    while y < height {
        let mut last_useful_x: Option<usize> = None;

        // Definimos el rango y la dirección del iterador según el sentido actual
        let x_range: Box<dyn Iterator<Item = usize>> = if left_to_right {
            Box::new(0..width)
        } else {
            Box::new((0..width).rev())
        };

        for x in x_range {
            if Some(matrix.data[y][x]) != matrix.no_data {
                // Punto válido en la fila actual
                path_points.push((x, y));

                // Verificamos si este punto sirve como "puente" para la siguiente fila
                if y + separation < height && Some(matrix.data[y + separation][x]) != matrix.no_data {
                    last_useful_x = Some(x);
                }
            } else if let Some(last_x) = last_useful_x {
                // Encontramos el final de los datos en esta fila, subimos verticalmente acorde a la separacion que el usuario indicó.
                for i in 1..separation {
                    let next_y = y + i;
                    if next_y < height && Some(matrix.data[next_y][last_x]) != matrix.no_data {
                        path_points.push((last_x, next_y)); // Obviamente vamos a agregar los puntos del puente, porque el barco pasa por ahí tambien
                    }
                }
                break; // Salimos del for x para pasar a la siguiente fila y
            }
        }

        y += separation;
        left_to_right = !left_to_right;
    }

    path_points
}