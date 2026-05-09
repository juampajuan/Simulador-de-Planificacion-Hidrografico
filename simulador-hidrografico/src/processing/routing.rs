use crate::structs::depth_matrix::DepthMatrix;

// Genera un recorrido sobre la matriz de profundidad, con un azimut y separación dados. El resultado es un vector de coordenadas (x, y) que representan el recorrido.
// El azimut se mide en grados, con 0° apuntando hacia el norte y aumentando en sentido horario. La separación se mide en metros y determina la distancia entre las piernas del zig-zag.
// Devuelvo todo el recorrido mas que nada porque puede servir para el front
pub fn generate_route(matrix: &DepthMatrix, azimuth_deg: f64, separation_meters: f64) -> Vec<(usize, usize)> {

    let mut path = Vec::new();

    let w = matrix.width as f64;
    let h = matrix.height as f64;

    let center_x = w / 2.0;
    let center_y = h / 2.0;

    let diagonal = (w.powi(2) + h.powi(2)).sqrt();

    // Aca obtengo el angulo
    let angle = azimuth_deg.to_radians();
    let (sin_a, cos_a) = angle.sin_cos();

    // Cuanto mide un pixel en metros
    let size_x = matrix.size_x;
    let size_y = matrix.size_y;

    // Dirección principal normalizada para poder usarla mas adelante como vector director. 
    // La divido por el tamaño del pixel para que la dirección esté en unidades de píxeles, y luego la normalizo para que tenga magnitud 1.
    let dir_x_px = sin_a / size_x;
    let dir_y_px = -cos_a / size_y;

    let mag_dir = (dir_x_px.powi(2) + dir_y_px.powi(2)).sqrt();

    let dir_x = dir_x_px / mag_dir;
    let dir_y = dir_y_px / mag_dir;

    // Esta es la dicc perpendicular dividida por el tamaño del pixel.
    let perpendicular_x_px = cos_a / size_x;
    let perpendicular_y_px = sin_a / size_y;

    let mag_perpendicular = (perpendicular_x_px.powi(2) + perpendicular_y_px.powi(2)).sqrt();

    // Esta es la direccion perpendicular normalizada.
    let perpendicular_x = perpendicular_x_px / mag_perpendicular;
    let perpendicular_y = perpendicular_y_px / mag_perpendicular;

    // Separación entre piernas en píxeles. Osea digamos la cantidad de píxeles que tengo que avanzar en la dirección perpendicular para lograr la separación deseada en metros.
    let separation_px = separation_meters * mag_perpendicular;

    let legs = (diagonal / separation_px).ceil() as i32;

    let previous_end: Option<(f64, f64)> = None;

    for leg in -legs / 2..=legs / 2 {

        let mut line = build_leg(matrix, center_x, center_y, perpendicular_x, perpendicular_y, dir_x, dir_y, diagonal, separation_px,  leg);

        if line.is_empty() {
            continue;
        }

        // Para el zig-zag. Despues conectaria la punta de esta pata con la anterior para que quedo un camino continuo
        // Por ahora lo dejo así para probar.
        if leg % 2 != 0 {
            line.reverse();
        }

        // Conecto la pierna con la otra
        if let Some(prev) = previous_end {
            connect(matrix, prev, line[0], &mut path);
        }        

        // Agregar pierna actual
        path.extend(
            line.iter().map(|(x, y)| {
                (x.round() as usize, y.round() as usize)
            })
        );
    }

    path
}

fn build_leg(matrix: &DepthMatrix, center_x: f64, center_y: f64, perpendicular_x: f64, perpendicular_y: f64, dir_x: f64, dir_y: f64, diagonal: f64, separation_px: f64, leg: i32) -> Vec<(f64, f64)> {

    let offset = leg as f64 * separation_px;

    // Este es el punto de origen de la pierna, que se desplaza a lo largo de la dirección perpendicular. Tanto para X como para Y.
    let origin_x = center_x + perpendicular_x * offset;
    let origin_y = center_y + perpendicular_y * offset;

    let mut line = Vec::new();

    let mut d = -diagonal / 2.0;

    while d <= diagonal / 2.0 {

        // Sobre el punto de origen obtenido en las lineas 80 y 81, avanzo en la dirección del azimut para generar el recorrido de la pierna. Tanto para X como para Y.
        let x = origin_x + dir_x * d;
        let y = origin_y + dir_y * d;

        if valid(matrix, x, y) {
            line.push((x, y));
        }

        d += 1.0;
    }

    line
}

fn connect(_matrix: &DepthMatrix, start: (f64, f64), end: (f64, f64), _path: &mut Vec<(usize, usize)>,) {
    let (x0, y0) = start;
    let (x1, y1) = end;

    let dx = x1 - x0;
    let dy = y1 - y0;

    let _steps = dx.abs().max(dy.abs()).ceil() as i32;
}
//Despues habria que recorrer de a i/steps pasos, y agregar los puntos de las rectas que generen dx y dy
// Y usar valid

fn valid(matrix: &DepthMatrix, x: f64, y: f64) -> bool {

    let xi = x.round() as isize;
    let yi = y.round() as isize;

    xi >= 0 && yi >= 0 && xi < matrix.width as isize && yi < matrix.height as isize && Some(matrix.data[yi as usize][xi as usize]) != matrix.no_data
}
