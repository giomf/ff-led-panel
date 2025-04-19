use heapless::String;

use super::{CommandAble, DEFAULT_SCHEDULE, realtime_clock::DateTime};
use core::fmt::Display;

const PAGES_MAX_CHARS: usize = 32;

pub struct Schedule {
    id: char,
    from: DateTime,
    to: DateTime,
    pages: [char; PAGES_MAX_CHARS],
}

impl CommandAble for Schedule {}

impl Schedule {
    pub fn id(mut self, schedule_id: char) -> Self {
        self.id = schedule_id;
        self
    }
    pub fn from(mut self, from: DateTime) -> Self {
        self.from = from;
        self
    }
    pub fn to(mut self, to: DateTime) -> Self {
        self.to = to;
        self
    }
    pub fn pages(mut self, pages: [char; PAGES_MAX_CHARS]) -> Self {
        self.pages = pages;
        self
    }
}

impl Display for Schedule {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut schedule = String::<PAGES_MAX_CHARS>::new();
        for page_id in self.pages {
            schedule.push(page_id).unwrap();
        }
        write!(f, "<T{}>{}{}{}", self.id, self.from, self.to, schedule)
    }
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            id: DEFAULT_SCHEDULE,
            from: Default::default(),
            to: Default::default(),
            pages: Default::default(),
        }
    }
}
