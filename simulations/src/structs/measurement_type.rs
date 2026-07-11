//La forma de medir y las aplicacion de errores varia segun el tipo de medicion que seleccione el alumno (Monohaz, Multihaz), por eso estas strucs

pub enum MeasurementsType {
    Monohaz {
        measurements: Vec<((usize, usize), f64)>,
    },
    Multihaz {
        central_measurments: Vec<((usize, usize), f64)>,
        paralel_measurment_1: Vec<((usize, usize), f64)>,
        paralel_measurment_2: Vec<((usize, usize), f64)>,
    },
}

pub enum MeasurementsTypeWithError {
    Monohaz {
        measurements: Vec<((usize, usize), Option<f64>)>,
    },
    Multihaz {
        central_measurments: Vec<((usize, usize), Option<f64>)>,
        paralel_measurment_1: Vec<((usize, usize), Option<f64>)>,
        paralel_measurment_2: Vec<((usize, usize), Option<f64>)>,
    },
}
