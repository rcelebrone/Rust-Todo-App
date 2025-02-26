#[cfg(test)]
mod todo_model_test {
    use crate::todo::todo_model::{Todo, TodoImpl, TodoListFilter, TodoToggleAction};
    use std::time::SystemTime;
    use uuid::Uuid;

    #[test]
    fn test_new_todo() {
        let text = "test";
        let todo = Todo::new(text);
        assert_eq!(todo.text, text);
        assert_eq!(todo.is_completed, false);
        assert!(todo.created_at <= SystemTime::now());
        assert!(todo.id.to_string().len() > 0);
    }

    #[test]
    fn test_todo_list_filter_display() {
        assert_eq!(TodoListFilter::All.to_string(), "all");
        assert_eq!(TodoListFilter::Active.to_string(), "active");
        assert_eq!(TodoListFilter::Completed.to_string(), "completed");
    }

    #[test]
    fn test_todo_toggle_action_display() {
        assert_eq!(TodoToggleAction::Check.to_string(), "check");
        assert_eq!(TodoToggleAction::Uncheck.to_string(), "uncheck");
    }

    #[test]
    fn test_todo_clone() {
        let todo = Todo {
            is_completed: false,
            created_at: SystemTime::now(),
            text: String::from("test"),
            id: Uuid::new_v4(),
        };
        let cloned_todo = todo.clone();
        assert_eq!(todo.is_completed, cloned_todo.is_completed);
        assert_eq!(todo.text, cloned_todo.text);
        assert_eq!(todo.id, cloned_todo.id);
    }
}
