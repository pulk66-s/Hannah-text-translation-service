use crate::db::mysql::Connector;

pub struct Verbs {
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Verb {
    pub id: i32,
    pub value: String,
    pub tense: String,
}

impl Verbs {
    pub fn new() -> Self {
        Verbs {
        }
    }

    pub fn get_all(&self) -> Vec<Verb> {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();
        let res = connector.launch_query("SELECT * FROM Verbs");
        let mut verbs: Vec<Verb> = Vec::new();

        for row in res {
            let r = row.unwrap();
            verbs.push(Verb {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
                tense: r.get(2).unwrap(),
            });
        }
        return verbs;
    }

    pub fn find(&self, data: &str) -> Vec<Verb>   {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let mut query = "".to_string();
        if data.contains("%") {
            query = format!("SELECT * FROM Verbs WHERE value LIKE '{}'", data);
        } else {
            query = format!("SELECT * FROM Verbs WHERE value = '{}'", data);
        }
        let res = connector.launch_query(&query);
        let mut verbs: Vec<Verb> = Vec::new();

        for row in res {
            let r = row.unwrap();
            verbs.push(Verb {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
                tense: r.get(2).unwrap(),
            });
        }
        return verbs;
    }

    pub fn find_in(&self, data: &Vec<String>) -> Vec<Verb> {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let mut query = "".to_string();
        let mut datas = "".to_string();
        for d in data {
            datas = format!("{}'{}',", datas, d);
        }
        datas.pop();
        query = format!("SELECT * FROM Verbs WHERE value IN ({})", datas);
        let res = connector.launch_query(&query);
        let mut verbs: Vec<Verb> = Vec::new();

        for row in res {
            let r = row.unwrap();
            verbs.push(Verb {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
                tense: r.get(2).unwrap(),
            });
        }
        return verbs;
    }
}
