use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub struct Identity {
    first_name: String,
    last_name: String,
    email: String,
    comment: Option<String>,
}

impl Identity {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        comment: Option<String>,
    ) -> Self {
        Self {
            first_name,
            last_name,
            email,
            comment,
        }
    }

    pub fn get_first_name(&self) -> &String {
        &self.first_name
    }

    pub fn get_last_name(&self) -> &String {
        &self.last_name
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }

    pub fn get_comment(&self) -> &Option<String> {
        &self.comment
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_new() {
        let identity = Identity::new(
            String::from("Alice"),
            String::from("Smith"),
            String::from("alice@example.com"),
            Some(String::from("Test Comment")),
        );

        assert_eq!(identity.first_name, "Alice");
        assert_eq!(identity.last_name, "Smith");
        assert_eq!(identity.email, "alice@example.com");
        assert_eq!(identity.comment, Some(String::from("Test Comment")));
    }

    #[test]
    fn test_identity_new_with_no_comment() {
        let identity = Identity::new(
            String::from("Bob"),
            String::from("Johnson"),
            String::from("bob@example.com"),
            None,
        );

        assert_eq!(identity.first_name, "Bob");
        assert_eq!(identity.last_name, "Johnson");
        assert_eq!(identity.email, "bob@example.com");
        assert_eq!(identity.comment, None);
    }

    #[test]
    fn test_get_first_name() {
        let identity = Identity::new(
            String::from("Charlie"),
            String::from("Brown"),
            String::from("charlie@example.com"),
            None,
        );

        assert_eq!(identity.get_first_name(), "Charlie");
    }

    #[test]
    fn test_get_last_name() {
        let identity = Identity::new(
            String::from("David"),
            String::from("Clark"),
            String::from("david@example.com"),
            None,
        );

        assert_eq!(identity.get_last_name(), "Clark");
    }

    #[test]
    fn test_get_email() {
        let identity = Identity::new(
            String::from("Emma"),
            String::from("Davis"),
            String::from("emma@example.com"),
            None,
        );

        assert_eq!(identity.get_email(), "emma@example.com");
    }

    #[test]
    fn test_get_comment_some() {
        let identity = Identity::new(
            String::from("Fiona"),
            String::from("Hill"),
            String::from("fiona@example.com"),
            Some(String::from("Hello there")),
        );

        assert_eq!(identity.get_comment(), &Some(String::from("Hello there")));
    }

    #[test]
    fn test_get_comment_none() {
        let identity = Identity::new(
            String::from("George"),
            String::from("White"),
            String::from("george@example.com"),
            None,
        );

        assert_eq!(identity.get_comment(), &None);
    }
}
