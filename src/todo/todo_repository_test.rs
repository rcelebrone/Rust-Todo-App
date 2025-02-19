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
}
