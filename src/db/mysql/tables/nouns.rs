use crate::db::mysql::Connector;

pub struct Nouns {
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Noun {
    pub id: i32,
    pub value: String,
    pub _type: String,
}

impl Nouns {
    pub fn new() -> Self {
        Nouns {
        }
    }

    pub fn get_all(&self) -> Vec<Noun> {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let res = connector.launch_query("SELECT * FROM Nouns");
        let mut nouns: Vec<Noun> = Vec::new();

        for row in res {
            let r = row.unwrap();
            nouns.push(Noun {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
                _type: r.get(2).unwrap(),
            });
        }
        return nouns;
    }

    pub fn find(&self, data: &str) -> Vec<Noun>   {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let mut query = "".to_string();
        if data.contains("%") {
            query = format!("SELECT * FROM Nouns WHERE value LIKE '{}'", data);
        } else {
            query = format!("SELECT * FROM Nouns WHERE value = '{}'", data);
        }
        let res = connector.launch_query(&query);
        let mut nouns: Vec<Noun> = Vec::new();

        for row in res {
            let r = row.unwrap();
            nouns.push(Noun {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
                _type: r.get(2).unwrap(),
            });
        }
        return nouns;
    }

    pub fn find_with_type(&self, data: &str, t: &str) -> Vec<Noun>   {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let mut query = "".to_string();
        if data.contains("%") {
            query = format!("SELECT * FROM Nouns WHERE value LIKE '{}' AND type='{}'", data, t);
        } else {
            query = format!("SELECT * FROM Nouns WHERE value = '{}' AND type='{}'", data, t);
        }
        let res = connector.launch_query(&query);
        let mut nouns: Vec<Noun> = Vec::new();

        for row in res {
            let r = row.unwrap();
            nouns.push(Noun {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
                _type: r.get(2).unwrap(),
            });
        }
        return nouns;
    }

    pub fn find_in(&self, data: &Vec<String>) -> Vec<Noun> {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let mut query = "".to_string();
        let mut datas = "".to_string();
        for d in data {
            datas = format!("{}'{}',", datas, d);
        }
        datas.pop();
        query = format!("SELECT * FROM Nouns WHERE value IN ({})", datas);
        let res = connector.launch_query(&query);
        let mut verbs: Vec<Noun> = Vec::new();

        for row in res {
            let r = row.unwrap();
            verbs.push(Noun {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
                _type: r.get(2).unwrap(),
            });
        }
        return verbs;
    }
}
