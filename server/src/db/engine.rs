use crate::db::migrations;

/// Motor de la base de datos: envuelve la conexión a SQLite.
pub struct DBEngine {
    pub(crate) connection: sqlite::Connection,
}

impl DBEngine {
    /// Abre (o crea) el archivo de base de datos, activa las foreign keys y corre las
    /// migraciones para asegurar que el esquema exista. Devuelve el motor listo para usar.
    pub fn new(name: &String) -> Result<Self, sqlite::Error> {
        let connection = sqlite::open(name)?;
        connection.execute("PRAGMA foreign_keys = ON;")?;
        let engine = Self { connection };

        migrations::init(&engine)?;

        Ok(engine)
    }

    /// Prepara una sentencia SQL sobre la conexión y la devuelve lista para bindear parámetros
    /// y ejecutar. Es el punto único por el que pasan todas las queries del crate.
    pub(crate) fn run_query<'a>(
        &'a self,
        query: &str,
    ) -> Result<sqlite::Statement<'a>, sqlite::Error> {
        self.connection.prepare(query)
    }
}
