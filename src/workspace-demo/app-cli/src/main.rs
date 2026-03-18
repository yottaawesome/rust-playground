use shared::{sample_users, Role, User};

/// CLI tool that filters and displays users by role.
fn main() {
    let role_filter = std::env::args().nth(1).unwrap_or_default();
    let users = sample_users();

    let filtered: Vec<&User> = match role_filter.to_lowercase().as_str() {
        "admin" => users.iter().filter(|u| matches!(u.role, Role::Admin)).collect(),
        "member" => users.iter().filter(|u| matches!(u.role, Role::Member)).collect(),
        "guest" => users.iter().filter(|u| matches!(u.role, Role::Guest)).collect(),
        _ => {
            println!("Usage: app-cli <admin|member|guest>");
            println!();
            println!("All users:");
            for user in &users {
                println!("  {}", user.display());
            }
            return;
        }
    };

    println!("Users with role '{role_filter}':");
    if filtered.is_empty() {
        println!("  (none)");
    } else {
        for user in filtered {
            println!("  {}", user.display());
        }
    }
}
