pub mod tables;

use mysql::*;
use mysql::prelude::*;
use tables::{
    verbs::Verbs,
    nouns::Nouns,
    determinants::Determinants,
    adverbes::Adverbes,
};

use self::tables::adjectives::Adjectives;

pub struct Connector {
    pub conn: Option<PooledConn>,
    pub pool: Option<Pool>,
}

pub struct Mysql {
    pub verbs: Verbs,
    pub nouns: Nouns,
    pub determinants: Determinants,
    pub adjectives: Adjectives,
    pub adverbes: Adverbes,
}

impl Mysql {
    pub fn new() -> Self {
        Mysql {
            verbs: Verbs::new(),
            nouns: Nouns::new(),
            determinants: Determinants::new(),
            adjectives: Adjectives::new(),
            adverbes: Adverbes::new(),
        }
    }
}

impl Connector {
    pub fn new() -> Self {
        Connector {
            conn: None,
            pool: None
        }
    }

    pub fn connect(&mut self) {
        self.pool = match Pool::new("mysql://Hannah:Hannahpwd@localhost:3306/HannahTTLSF") {
            Ok(pool) => Some(pool),
            Err(e) => panic!("Error connecting to database: {}", e),
        };
        self.conn = match &self.pool {
            Some(p) => match p.get_conn() {
                Ok(conn) => Some(conn),
                Err(e) => panic!("Error connecting to database: {}", e),
            },
            None => panic!("Error connecting to database: {}", "No connection pool"),
        };
    }

    pub fn check_connection(&mut self) {
        self.launch_query("SELECT 1");
    }

    pub fn launch_query(&mut self, query: &str) -> QueryResult<'_, '_, '_, Text> {
        match &mut self.conn {
            Some(conn) => {
                match conn.query_iter(query) {
                    Ok(r) => {
                        return r;
                    },
                    Err(e) => panic!("Error launching query: {}", e),
                }
            },
            None => panic!("Error launching query: {}", "No connection"),
        }
    }
}
