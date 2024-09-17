use serde_json;

#[derive(Clone)]
pub struct Json {
    pub path: String,
    pub content: String,
    pub json: serde_json::Value,
}

impl<'a> Json {
    pub fn new(path: String) -> Json {
        Json {
            path: path,
            content: String::new(),
            json: serde_json::Value::Null,
        }
    }

    pub fn read(&mut self) -> bool {
        match std::fs::read_to_string(&self.path) {
            Ok(content) => {
                self.content = content;
                match serde_json::from_str(&self.content) {
                    Ok(json) => {
                        self.json = json;
                        true
                    },
                    Err(e) => panic!("Error parsing json: {}", e),
                }
            },
            Err(e) => panic!("Error reading file: {}", e),
        }
    }

    pub fn keys(&self) -> Vec<String> {
        if self.json.is_null() {
            panic!("Json is null");
        }

        let mut keys = Vec::new();
        for key in self.json.as_object().unwrap().keys() {
            keys.push(key.to_string());
        }
        keys
    }
}