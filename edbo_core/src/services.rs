use crate::database::Database;

pub trait Service<'a>: Send + Sync {
    fn new(database: &'a Database) -> Self
    where
        Self: Sized;
}

pub mod enum_service;
pub mod institutions;
pub mod offer_university;
