//reemplazar nombre a calculoar puntos Y tomr las mediciones.
pub fn find_measuring_points(path: &Vec<(usize, usize)>, distance_between_points: f64) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    
    //convertir distancias a metros

    let mut measuring_points: Vec<(usize, usize)> = Vec::new();

    let mut current_point = &(path[0]);
    
    let mut distance_progress: f64 = 0.0;

    for next_point in path {

        let current_distance = calculate_distance_between_points(current_point, next_point);

        distance_progress += current_distance;

        if distance_progress >= distance_between_points{
            distance_progress -= distance_between_points;

            // Posible mejora para elegir cual de los dos pixeles tomar como punto segun a cuanto estan del punto real
            // let exceso = distance_progress - distance_between_points;
            // let falta = distance_between_points - (distance_progress - current_distance);
            //Aca quiza entra lo que habia dicho el porfesor, de elegir cual de los dos puntos esta mas cerca realmente.
            //Con esto me refiero a la charla del 13 de mayo, que Juampa estuvo casi solo.
            
            //Aca tambien se podria utilizar un metodo ajeno para tomar la medida directamente ya que tenemos la info de la 
            //posicion.

            //Funcion aqui:
            measuring_points.push(*current_point);
        }
        current_point = next_point;
    }

    let perpendicular_points = find_perpendicular_points(&measuring_points);

    (measuring_points, perpendicular_points)
}

fn calculate_distance_between_points(point_a: &(usize, usize), point_b: &(usize, usize)) -> f64 {
    
    let a_x = point_a.0 as f64;
    let a_y = point_a.1 as f64;
    let b_x = point_b.0 as f64;
    let b_y = point_b.1 as f64;

    ((a_x - b_x).powi(2) + (a_y - b_y).powi(2)).sqrt()
}

fn find_perpendicular_points(measure_points: &Vec<(usize, usize)>) -> Vec<(usize, usize)>{
    let mut perp: Vec<(usize, usize)> = Vec::new();

    let mut previous_point = &(measure_points[0]);
    let mut current_point = &(measure_points[0]);
    
    for point in measure_points {
        let iteration_points = get_points_perpendicular_to_this(previous_point, current_point, point, 10.0);
        for p in iteration_points{
            perp.push(p);
        }
        previous_point = current_point;
        current_point = point;
    }

    perp
}

fn get_points_perpendicular_to_this(prev_point: &(usize, usize),current_point: &(usize, usize),next_point: &(usize, usize),step_distance: f64) -> Vec<(usize, usize)> {
    let dist_to_prev = calculate_distance_between_points(current_point, prev_point);
    let dist_to_next = calculate_distance_between_points(current_point, next_point);

    //Tomamos el punto mas lejano, que estara en la misma recta - direccion que el punto en cuestion
    let reference = if dist_to_prev >= dist_to_next {
        prev_point
    } else {
        next_point
    };


    //Forma el vector
    let dx = reference.0 as f64 - current_point.0 as f64;
    let dy = reference.1 as f64 - current_point.1 as f64;
    let magnitude = (dx * dx + dy * dy).sqrt();
    if magnitude == 0.0 {
        return Vec::new();
    }
    let dx_norm = dx / magnitude;
    let dy_norm = dy / magnitude;

    //90 grados
    let perp_x = -dy_norm;
    let perp_y = dx_norm;

    //Coordenadas actuales
    let cx = current_point.0 as f64;
    let cy = current_point.1 as f64;

    let mut points = Vec::new();

    for i in 1..=4_i32 {
        let dist = i as f64 * step_distance;

        let lx = cx + perp_x * dist;
        let ly = cy + perp_y * dist;
        if lx >= 0.0 && ly >= 0.0 {
            points.push((lx.round() as usize, ly.round() as usize));
        }

        let rx = cx - perp_x * dist;
        let ry = cy - perp_y * dist;
        if rx >= 0.0 && ry >= 0.0 {
            points.push((rx.round() as usize, ry.round() as usize));
        }
    }

    points
}