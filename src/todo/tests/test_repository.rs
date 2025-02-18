#[cfg(test)]
mod test_repository {
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
}
