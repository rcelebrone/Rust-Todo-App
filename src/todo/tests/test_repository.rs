use todo::todo_repository::TodoRepository;
use uuid::Uuid;

#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_get_not_exists() {
    let repo = TodoRepository::default();
    let todo = repo.get(&Uuid::new_v4());
    assert!(todo.is_err());
}
