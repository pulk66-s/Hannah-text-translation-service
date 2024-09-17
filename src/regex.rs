pub struct Regex {
    pub regex: regex::Regex,
}

impl<'a> Regex {
    pub fn new(pattern: String) -> Regex {
        Regex {
            regex: regex::Regex::new(&pattern).unwrap(),
        }
    }

    pub fn replace(&self, text: String, replacement: String) -> String {
        self.regex.replace_all(&text, &replacement).to_string()
    }

    pub fn matches(&self, text: String) -> bool {
        self.regex.is_match(&text)
    }
}