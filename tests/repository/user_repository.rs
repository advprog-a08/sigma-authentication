use sigma_authentication::repository::UserRepository;

#[test]
fn test_create_and_find_one() {
    let mut ur = UserRepository::default();
    let user = ur.create("asdf@gmail.com".to_string());

    let found = ur.find_one(user.id.to_string());
    assert_eq!(found.unwrap().email, "asdf@gmail.com");
}
