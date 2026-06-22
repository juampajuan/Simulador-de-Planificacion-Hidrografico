pub mod student;
pub mod auth;
pub mod professor;
pub mod projects;

// Capa intermedia para acceder a la DB.
// Se encarga de tomar el lock del Mutex de la DB y delegar
// en las queries crudas de `queries::auth`.
// Si el lock está envenenado, devuelve error.