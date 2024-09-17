use std::fmt;

pub enum Time {
    Past,
    Present,
    Future,
    Infinitive,
    Unknown,
}

impl<'a> fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Time::Past => write!(f, "Past"),
            Time::Present => write!(f, "Present"),
            Time::Future => write!(f, "Future"),
            Time::Infinitive => write!(f, "Infinitive"),
            Time::Unknown => write!(f, "Unknown"),
        }
    }
}
