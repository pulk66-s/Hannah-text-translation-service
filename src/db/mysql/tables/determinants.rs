use crate::db::mysql::Connector;

pub struct Determinants {
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Determinant {
    pub id: i32,
    pub value: String,
}

impl Determinants {
    pub fn new() -> Self {
        Determinants {
        }
    }

    pub fn get_all(&self) -> Vec<Determinant> {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let res = connector.launch_query("SELECT * FROM Determinants");
        let mut nouns: Vec<Determinant> = Vec::new();

        for row in res {
            let r = row.unwrap();
            nouns.push(Determinant {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
            });
        }
        return nouns;
    }

    pub fn find(&self, data: &str) -> Vec<Determinant>   {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let mut query = "".to_string();
        if data.contains("%") {
            query = format!("SELECT * FROM Determinants WHERE value LIKE '{}'", data);
        } else {
            query = format!("SELECT * FROM Determinants WHERE value = '{}'", data);
        }
        let res = connector.launch_query(&query);
        let mut nouns: Vec<Determinant> = Vec::new();

        for row in res {
            let r = row.unwrap();
            nouns.push(Determinant {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
            });
        }
        return nouns;
    }
    pub fn find_in(&self, data: &Vec<String>) -> Vec<Determinant> {
        let connector = &mut Connector::new();

        connector.connect();
        connector.check_connection();

        let mut query = "".to_string();
        let mut datas = "".to_string();
        for d in data {
            datas = format!("{}'{}',", datas, d);
        }
        datas.pop();
        query = format!("SELECT * FROM Determinants WHERE value IN ({})", datas);
        let res = connector.launch_query(&query);
        let mut verbs: Vec<Determinant> = Vec::new();

        for row in res {
            let r = row.unwrap();
            verbs.push(Determinant {
                id: r.get(0).unwrap(),
                value: r.get(1).unwrap(),
            });
        }
        return verbs;
    }
}
