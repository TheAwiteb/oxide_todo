use std::collections::BTreeMap;

pub trait ClaimsHelper {
    fn get_user_id(&self) -> u32;
    fn get_created_at(&self) -> i64;
}

impl ClaimsHelper for BTreeMap<String, String> {
    fn get_user_id(&self) -> u32 {
        self["id"].parse().unwrap()
    }
    fn get_created_at(&self) -> i64 {
        self["created_at"].parse().unwrap()
    }
}
