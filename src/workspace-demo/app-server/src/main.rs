use shared::{sample_users, Role, User};

/// Simulates a server that lists users and processes admin requests.
fn main() {
    println!("=== Server Starting ===");
    println!();

    let users = sample_users();
    println!("Registered users:");
    for user in &users {
        println!("  - {}", user.display());
    }

    println!();
    let admins: Vec<&User> = users.iter().filter(|u| matches!(u.role, Role::Admin)).collect();
    println!("Admin users ({}):", admins.len());
    for admin in admins {
        println!("  * {}", admin.name);
    }

    println!();
    println!("=== Server Ready ===");
}
