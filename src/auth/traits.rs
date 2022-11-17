use std::collections::BTreeMap;

use chrono::NaiveDateTime;

pub trait ClaimsHelper {
    fn get_user_id(&self) -> u32;
    fn get_created_at(&self) -> NaiveDateTime;
}

impl ClaimsHelper for BTreeMap<String, String> {
    fn get_user_id(&self) -> u32 {
        self["id"].parse().unwrap()
    }
    fn get_created_at(&self) -> NaiveDateTime {
        let timestamp = self["created_at"].parse().unwrap();
        NaiveDateTime::from_timestamp(timestamp, 0)
    }
}
