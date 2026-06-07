use crate::db::migrations;

pub struct DBEngine {
    pub(crate) connection: sqlite::Connection,
}

impl DBEngine {

    pub fn new(
        name: &String
    ) -> Result<Self, sqlite::Error> {

        let connection = sqlite::open(name)?;

        let engine = Self {
            connection
        };

        migrations::init(&engine)?;

        Ok(engine)
    }

    pub(crate) fn run_query<'a>(
        &'a self,
        query: &str
    ) -> Result<sqlite::Statement<'a>, sqlite::Error> {

        self.connection.prepare(query)
    }
}