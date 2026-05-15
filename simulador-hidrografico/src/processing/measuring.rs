//reemplazar nombre a calculoar puntos Y tomr las mediciones.
pub fn find_measuring_points(path: &Vec<(usize, usize)>, distance_between_points: f64) -> Vec<(usize, usize)> {
    
    //convertir distancias a metros

    let mut measuring_points: Vec<(usize, usize)> = Vec::new();

    let origin = path[0];
    
    let mut current_x = origin.0 as f64;
    let mut current_y = origin.1 as f64;

    let mut distance_progress: f64 = 0.0;

    for point in path {

        let next_x = point.0 as f64;
        let next_y = point.1 as f64;

        let current_distance = ((next_x - current_x).powi(2) + (next_y - current_y).powi(2)).sqrt();

        distance_progress += current_distance;

        if distance_progress >= distance_between_points{
            distance_progress -= distance_between_points;

            //Aca quiza entra lo que habia dicho el porfesor, de elegir cual de los dos puntos esta mas cerca realmente.
            //Con esto me refiero a la charla del 13 de mayo, que Juampa estuvo casi solo.
            
            //Aca tambien se podria utilizar un metodo ajeno para tomar la medida directamente ya que tenemos la info de la 
            //posicion.
            measuring_points.push(*point);
        }

        current_x = next_x;
        current_y = next_y;

    }

    measuring_points
}