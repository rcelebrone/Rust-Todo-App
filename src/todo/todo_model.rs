use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Todo {
    pub is_completed: bool,
    pub created_at: SystemTime,
    pub text: String,
    pub id: uuid::Uuid
}

pub trait TodoImpl {
    fn new(text: &str) -> Self;
}

impl TodoImpl for Todo {
    fn new(text: &str) -> Self {
        Self {
            is_completed: false,
            created_at: SystemTime::now(),
            text: String::from(text),
            id: uuid::Uuid::new_v4()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodoListFilter {
    All,
    Active,
    Completed
}

impl Display for TodoListFilter {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TodoListFilter::All => write!(f, "all"),
            TodoListFilter::Active => write!(f, "active"),
            TodoListFilter::Completed => write!(f, "completed")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodoToggleAction {
    Check,
    Uncheck
}

impl Display for TodoToggleAction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            TodoToggleAction::Check => write!(f, "check"),
            TodoToggleAction::Uncheck => write!(f, "uncheck")
        }
    }
}