use sigma_authentication::repository::UserRepository;

#[test]
fn test_create_and_find_one() {
    let mut ur = UserRepository::default();
    ur.create("asdf@gmail.com".to_string());

    let found = ur.find_one("asdf@gmail.com".to_string());
    assert_eq!(found.unwrap().email, "asdf@gmail.com");
}
