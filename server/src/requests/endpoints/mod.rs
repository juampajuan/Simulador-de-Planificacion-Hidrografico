pub mod auth;
pub mod exams;
pub mod files;
pub mod generic;
pub mod limits;
pub mod projects;
pub mod simulation;
pub mod students;

// Crate con los metodos para manejar cada endpoint en particular
// Las requests las recibe `handle_request` y este delega
// al metodo que corresponde.
