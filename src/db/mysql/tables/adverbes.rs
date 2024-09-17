use crate::db::mysql::Connector;

pub struct Adverbes {
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Adverbe {
    pub id: i32,
    pub value: String,
}

impl Adverbes {
    pub fn new() -> Self {
        Adverbes {
        }
    }

    pub fn get_all(&self) -> Vec<Adverbe> {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let res = connector.launch_query("SELECT * FROM Adverbes");
        let mut nouns: Vec<Adverbe> = Vec::new();

        for row in res {
            let r = row.unwrap();
            nouns.push(Adverbe {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
            });
        }
        return nouns;
    }

    pub fn find(&self, data: &str) -> Vec<Adverbe>   {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let mut query = "".to_string();
        if data.contains("%") {
            query = format!("SELECT * FROM Adverbes WHERE value LIKE '{}'", data);
        } else {
            query = format!("SELECT * FROM Adverbes WHERE value = '{}'", data);
        }
        let res = connector.launch_query(&query);
        let mut nouns: Vec<Adverbe> = Vec::new();

        for row in res {
            let r = row.unwrap();
            nouns.push(Adverbe {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
            });
        }
        return nouns;
    }

    pub fn find_in(&self, data: &Vec<String>) -> Vec<Adverbe> {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let mut query = "".to_string();
        let mut datas = "".to_string();
        for d in data {
            datas = format!("{}'{}',", datas, d);
        }
        datas.pop();
        query = format!("SELECT * FROM Adverbes WHERE value IN ({})", datas);
        let res = connector.launch_query(&query);
        let mut verbs: Vec<Adverbe> = Vec::new();

        for row in res {
            let r = row.unwrap();
            verbs.push(Adverbe {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
            });
        }
        return verbs;
    }
}
