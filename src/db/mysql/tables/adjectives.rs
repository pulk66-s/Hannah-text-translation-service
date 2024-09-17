use crate::db::mysql::Connector;

pub struct Adjectives {
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Adjective {
    pub id: i32,
    pub value: String,
}

impl Adjectives {
    pub fn new() -> Self {
        Adjectives {
        }
    }

    pub fn get_all(&self) -> Vec<Adjective> {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let res = connector.launch_query("SELECT * FROM Adjectives");
        let mut nouns: Vec<Adjective> = Vec::new();

        for row in res {
            let r = row.unwrap();
            nouns.push(Adjective {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
            });
        }
        return nouns;
    }

    pub fn find(&self, data: &str) -> Vec<Adjective>   {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let mut query = "".to_string();
        if data.contains("%") {
            query = format!("SELECT * FROM Adjectives WHERE value LIKE '{}'", data);
        } else {
            query = format!("SELECT * FROM Adjectives WHERE value = '{}'", data);
        }
        let res = connector.launch_query(&query);
        let mut nouns: Vec<Adjective> = Vec::new();

        for row in res {
            let r = row.unwrap();
            nouns.push(Adjective {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
            });
        }
        return nouns;
    }

    pub fn find_in(&self, data: &Vec<String>) -> Vec<Adjective> {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let mut query = "".to_string();
        let mut datas = "".to_string();
        for d in data {
            datas = format!("{}'{}',", datas, d);
        }
        datas.pop();
        query = format!("SELECT * FROM Adjectives WHERE value IN ({})", datas);
        let res = connector.launch_query(&query);
        let mut verbs: Vec<Adjective> = Vec::new();

        for row in res {
            let r = row.unwrap();
            verbs.push(Adjective {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
            });
        }
        return verbs;
    }
}
