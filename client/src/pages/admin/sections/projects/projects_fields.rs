// tiene lo comun entre edicion y creacion,
#[derive(Clone, PartialEq, Default)]
pub struct ProjectFormFields {
    pub attempts_limit: String,
    pub weather: String,
    pub seabed_hardness: String,
    pub budget: String,
    pub min_depth: String,
    pub max_depth: String,
}

impl ProjectFormFields {
    // Convierte un Proyecto existente a strings para la UI de Edición
    pub fn from_project(p: &crate::structs::project::Project) -> Self {
        Self {
            attempts_limit: p.attempts_limit.to_string(),
            weather: p.weather.clone(),
            seabed_hardness: p.seabed_hardness.clone(),
            budget: p.budget.to_string(),
            min_depth: p.geotiff_min_depth.to_string(),
            max_depth: p.geotiff_max_depth.to_string(),
        }
    }

    // Inicializa por defecto para la Creación (vacíos y selectores iniciales)
    pub fn new_empty() -> Self {
        Self {
            weather: "Favorable".to_string(),
            seabed_hardness: "Duro".to_string(),
            ..Self::default()
        }
    }
}
