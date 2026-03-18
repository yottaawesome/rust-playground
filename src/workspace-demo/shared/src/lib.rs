/// A simple user struct shared across workspace crates.
#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub role: Role,
}

#[derive(Debug, Clone)]
pub enum Role {
    Admin,
    Member,
    Guest,
}

impl User {
    pub fn new(name: &str, role: Role) -> Self {
        Self {
            name: name.to_string(),
            role,
        }
    }

    pub fn display(&self) -> String {
        format!("{} ({:?})", self.name, self.role)
    }
}

/// Returns a list of sample users for demonstration.
pub fn sample_users() -> Vec<User> {
    vec![
        User::new("Alice", Role::Admin),
        User::new("Bob", Role::Member),
        User::new("Charlie", Role::Guest),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_display() {
        let user = User::new("Alice", Role::Admin);
        assert_eq!(user.display(), "Alice (Admin)");
    }

    #[test]
    fn test_sample_users() {
        let users = sample_users();
        assert_eq!(users.len(), 3);
    }
}
