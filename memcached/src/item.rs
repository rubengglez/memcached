use chrono::prelude::*;
use std::{
    ops::{Add, Sub},
    time::Duration,
};

#[derive(Debug)]
pub struct Item {
    pub flags: u16,
    exptime: i64,
    pub value: String,
    pub value_length: usize,
}

impl Item {
    pub fn new(flags: u16, exptime: isize, value_length: usize, value: String) -> Self {
        let mut will_expire_on = Utc::now();

        if exptime < 0 {
            will_expire_on = will_expire_on.sub(Duration::new(1, 0));
        } else {
            will_expire_on = will_expire_on.add(Duration::new(exptime as u64, 0));
        }

        Item {
            flags,
            exptime: will_expire_on.timestamp(),
            value_length,
            value,
        }
    }

    pub fn expired(&self) -> bool {
        let now = Utc::now().timestamp();

        self.exptime < now
    }
}
