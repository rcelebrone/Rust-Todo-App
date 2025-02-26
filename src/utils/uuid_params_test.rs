#[cfg(test)]
mod uuid_params_test {
    use crate::utils::uuid_params::UuidParam;
    use rocket::request::FromParam;
    use uuid::Uuid;

    #[test]
    fn test_valid_uuid() {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_string();
        let result = UuidParam::from_param(&uuid_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().0, uuid);
    }

    #[test]
    fn test_invalid_uuid() {
        let invalid_uuid_str = "invalid-uuid";
        let result = UuidParam::from_param(invalid_uuid_str);
        assert!(result.is_err());
    }
}
