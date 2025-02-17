use uuid::Uuid;
use std::collections::HashMap;
use super::todo_model::{Todo, TodoImpl, TodoListFilter, TodoToggleAction};

pub enum TodoRepoError {
    NotFound
}

#[derive(Default, Debug)]
pub struct TodoRepository {
    num_completed: u32,
    num_active: u32,
    num_all: u32,
    items: HashMap<Uuid, Todo>
}

pub trait TodoRepositoryImpl {
    fn get(&self, id: &Uuid) -> Result<Todo, TodoRepoError>;
    fn list(&self, filter: &TodoListFilter) -> Vec<Todo>;
    fn create(&mut self, text: &str) -> Result<Todo, TodoRepoError>;
    fn delete(&mut self, id: &Uuid) -> Result<(), TodoRepoError>;
    fn update(&mut self, id: &Uuid, text: Option<String>, is_completed: Option<bool>) -> Result<Todo, TodoRepoError>;
    fn delete_completed(&mut self) -> Result<(), TodoRepoError>;
    fn toggle_completed(&mut self, action: &TodoToggleAction) -> Result<(), TodoRepoError>;
}

impl TodoRepositoryImpl for TodoRepository {
    fn get(&self, id: &Uuid) -> Result<Todo, TodoRepoError> {
        self.items.get(id).cloned().ok_or(TodoRepoError::NotFound)
    }
    fn list(&self, filter: &TodoListFilter) -> Vec<Todo> {
        let mut todos = self.items.values().filter(|t| {
            match filter {
                TodoListFilter::All => true,
                TodoListFilter::Active => !t.is_completed,
                TodoListFilter::Completed => t.is_completed
            }
        }).cloned().collect::<Vec<Todo>>();
        
        todos.sort_by(|a,b| a.created_at.cmp(&b.created_at));
        
        todos
    }
    fn create(&mut self, text: &str) -> Result<Todo, TodoRepoError> {
        let todo:Todo = TodoImpl::new(text);
        self.items.insert(todo.id, todo.clone());
        self.num_active += 1;
        self.num_all += 1;
        Ok(todo)
    }
    fn delete(&mut self, id: &Uuid) -> Result<(), TodoRepoError> {
        let todo = self.items.remove(id).ok_or(TodoRepoError::NotFound)?;
        
        if todo.is_completed {
            self.num_completed -= 1;
        } else {
            self.num_active -= 1;
        }

        self.num_all -= 1;

        Ok(())
    }

    fn update(&mut self, id: &Uuid, text: Option<String>, is_completed: Option<bool>) -> Result<Todo, TodoRepoError> {
        let todo = self.items.get_mut(id).ok_or(TodoRepoError::NotFound)?;
        
        if let Some(is_completed) = is_completed {
            todo.is_completed = is_completed;

            if todo.is_completed {
                self.num_completed += 1;
                self.num_active -= 1;
            } 
        }

        if let Some(text) = text {
            todo.text = text;
        }

        Ok(todo.clone())
    }

    fn delete_completed(&mut self) -> Result<(), TodoRepoError> {
        self.items.retain(|_, todo| !todo.is_completed);
        self.num_all -= self.num_completed;
        self.num_completed = 0;
        
        Ok(())
    }

    fn toggle_completed(&mut self, action: &TodoToggleAction) -> Result<(), TodoRepoError> {
        let is_completed: bool;
        match action {
            TodoToggleAction::Check => {
                self.num_completed = self.num_all; 
                self.num_active = 0;
                is_completed = true; 
            },
            TodoToggleAction::Uncheck => {
                self.num_completed = 0; 
                self.num_active = self.num_all;
                is_completed = false; 
            }
        }

        for todo in self.items.values_mut() {
            todo.is_completed = is_completed;
        }
        
        Ok(())
    }
}
