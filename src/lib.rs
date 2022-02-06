use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs, io};
use thiserror::Error;

pub const DB: &str = "./data/db.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Pet {
    pub id: usize,
    pub name: String,
    pub category: String,
    pub age: usize,
    pub created_at: DateTime<Utc>,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error while reading database file: {0}")]
    DbReadingError(#[from] io::Error),
    #[error("Error while parsing database file: {0}")]
    DbParsingError(#[from] serde_json::Error),
}

pub enum Event<I> {
    Input(I),
    Tick,
}

#[derive(Copy, Clone, Debug)]
pub enum Menu {
    Home,
    Pets,
}

impl From<Menu> for usize {
    fn from(input: Menu) -> usize {
        match input {
            Menu::Home => 0,
            Menu::Pets => 1,
        }
    }
}

pub fn read_db() -> Result<Vec<Pet>, Error> {
    let db = fs::read_to_string(DB)?;
    let parsed: Vec<Pet> = serde_json::from_str(&db)?;
    Ok(parsed)
}
