pub mod auth;
pub mod professor;
pub mod projects;
pub mod student;
pub mod student_simulations;

// Capa intermedia para acceder a la DB.
// Se encarga de tomar el lock del Mutex de la DB y delegar
// en las queries crudas de `queries::auth`.
// Si el lock está envenenado, devuelve error.
