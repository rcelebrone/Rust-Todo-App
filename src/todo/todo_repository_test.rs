#[cfg(test)]
mod todo_repository_test {
    use crate::todo::todo_repository::{TodoRepository, TodoRepoError, TodoRepositoryImpl};
    use uuid::Uuid; 

    #[test]
    fn test_get_not_exists() {
        let repo = TodoRepository::default();
        let todo = repo.get(&Uuid::new_v4());
        assert_eq!(todo, Err(TodoRepoError::NotFound));
    }

    #[test]
    fn test_create_todo() {
        let mut repo = TodoRepository::default();
        let num_all = repo.get_num_all();
        let _todo = repo.create("test");
        let num_all_after = repo.get_num_all();
        assert!(num_all_after == num_all + 1);
    }

    #[test]
    fn test_delete_todo() {
        let mut repo = TodoRepository::default();
        let todo = repo.create("test");
        let num_all = repo.get_num_all();
        let _todo = repo.delete(&todo.unwrap().id);
        let num_all_after = repo.get_num_all();
        assert!(num_all_after == num_all - 1);
    }

    #[test]
    fn test_update_todo() {
        let mut repo = TodoRepository::default();
        let todo = repo.create("test");
        let id = &todo.unwrap().id;
        let text = "test2";
        let _todo = repo.update(&id, Some(text.to_string()), None);
        let todo_updated = repo.get(&id);
        assert_eq!(&todo_updated.unwrap().text, &text);
    }

    #[test]
    fn test_update_completed_todo() {
        let mut repo = TodoRepository::default();
        let todo = repo.create("test");
        let id = &todo.unwrap().id;
        let _todo = repo.update(&id, None, Some(true));
        let todo_updated = repo.get(&id);
        assert!(todo_updated.unwrap().is_completed);
    }

    #[test]
    fn test_get_all() {
        let mut repo = TodoRepository::default();
        let todo1 = repo.create("test1").unwrap();
        let todo2 = repo.create("test2").unwrap();
        let todo3 = repo.create("test3").unwrap();

        let all_todos = repo.get_all();
        assert_eq!(all_todos.len(), 3);

        assert!(all_todos.iter().any(|t| t.id == todo1.id));
        assert!(all_todos.iter().any(|t| t.id == todo2.id));
        assert!(all_todos.iter().any(|t| t.id == todo3.id));
    }

    #[test]
    fn test_get_all_empty() {
        let repo = TodoRepository::default();
        let all_todos = repo.get_all();
        assert!(all_todos.is_empty());
    }

    #[test]
    fn test_get_all_with_one() {
        let mut repo = TodoRepository::default();
        let todo = repo.create("test").unwrap();
        let all_todos = repo.get_all();

        assert_eq!(all_todos.len(), 1);
        assert_eq!(all_todos[0].id, todo.id);
    }

}
