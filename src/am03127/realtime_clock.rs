#![allow(dead_code)]

use super::CommandAble;
use core::fmt::Display;

#[derive(Default)]
pub struct DateTime {
    year: u8,
    week: u8,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl Display for DateTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.year, self.month, self.day, self.hour, self.minute
        )
    }
}

#[derive(Default)]
pub struct RealTimeClock {
    date_time: DateTime,
}

impl CommandAble for RealTimeClock {}

impl RealTimeClock {
    pub fn year(mut self, year: u8) -> Self {
        self.date_time.year = year;
        self
    }
    pub fn week(mut self, week: u8) -> Self {
        self.date_time.week = week;
        self
    }
    pub fn month(mut self, month: u8) -> Self {
        self.date_time.month = month;
        self
    }
    pub fn day(mut self, day: u8) -> Self {
        self.date_time.day = day;
        self
    }
    pub fn hour(mut self, hour: u8) -> Self {
        self.date_time.hour = hour;
        self
    }
    pub fn minute(mut self, minute: u8) -> Self {
        self.date_time.minute = minute;
        self
    }
    pub fn second(mut self, second: u8) -> Self {
        self.date_time.second = second;
        self
    }
}

impl Display for RealTimeClock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "<SC>{:02}{:02}{:02}{:02}{:02}{:02}{:02}",
            self.date_time.year,
            self.date_time.week,
            self.date_time.month,
            self.date_time.day,
            self.date_time.hour,
            self.date_time.minute,
            self.date_time.second
        )
    }
}
