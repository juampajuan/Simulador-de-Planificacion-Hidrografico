pub mod webpage;
pub mod simulation;
pub mod auth;
pub mod generic;
pub mod limits;
pub mod projects;
pub mod students;

// Crate con los metodos para manejar cada endpoint en particular
// Las requests las recibe `handle_request` y este delega
// al metodo que corresponde.