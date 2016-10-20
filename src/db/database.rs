use std::collections::HashMap;
use super::stream::Stream;
use super::parser::{parse_statement, Statement, ParseError};
use super::row_builder::RowBuilder;

#[derive(Debug, PartialEq)]
pub enum DatabaseError {
    TableExists,
    QueryParseError,
}

#[derive(Debug)]
pub enum QueryResult {
    ResultSet,
    Insert(u64),
}

impl From<ParseError> for DatabaseError {
    fn from(err: ParseError) -> DatabaseError {
        DatabaseError::QueryParseError
    }
}


pub struct Database {
    tables: HashMap<String, Stream>
}

impl Database {
    pub fn new() -> Database {
        Database{
            tables: HashMap::new()
        }
    }

    pub fn create_stream(&mut self, name: &str) -> Result<&mut Stream, DatabaseError> {
        let tmp = Stream::new();
        if self.tables.contains_key(name) {
            return Err(DatabaseError::TableExists);
        }
        self.tables.insert(name.to_string(), tmp);
        let stream = self.tables.get_mut(name).unwrap();
        Ok(stream)
    }

    fn get_stream_mut(&mut self, name: &str) -> Option<&mut Stream> {
        self.tables.get_mut(name)
    }

    pub fn execute(&mut self, query: &str) -> Result<QueryResult, DatabaseError> {
        let tmp = try!(parse_statement(query));

        let result = match tmp {
            Statement::Insert(stream, row_builder) =>
                self.insert(&stream, &row_builder),
            _ => Ok(QueryResult::ResultSet)
        };
        result
    }

    pub fn insert(&mut self, query: &str, row_builder: &RowBuilder) -> Result<QueryResult, DatabaseError> {
        Ok(QueryResult::Insert(0))
    }

}

#[cfg(test)]
mod tests {
    use super::Database;
    use super::DatabaseError;

    // returns a valid DB for use with testing with valid simple schema
    fn get_db_with_stream() -> Database {
        let mut db = Database::new();
        db.create_stream("Jon");

        db
    }

    #[test]
    fn create_table() {
        let mut db = Database::new();
        db.create_stream("Jon");
    }

    #[test]
    fn create_table_fails_when_table_exists() {
        let mut db = get_db_with_stream();
        if let Err(result) = db.create_stream("Jon") {
            assert_eq!(result, DatabaseError::TableExists);
        } else {
            panic!("Was expecting DatabaseError::TableExists, got an OK");
        }

    }
}
