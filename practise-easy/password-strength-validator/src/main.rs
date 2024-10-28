use password_strength_validator::PasswordValidator;

fn main() {
    let validator = PasswordValidator::new(8, true, true);
    let test_passwords = vec![
        "short",
        "NoSpecialChar123",
        "NoNumber!@#",
        "Valid1!Password",
        "password",
    ];

    for password in test_passwords {
        println!("\nTesting password: {}", password);
        if validator.is_compromised(password) {
            println!("⚠️  Warning: This password appears in common password lists!");
            continue;
        }
        match validator.validate_password(password) {
            Ok(()) => println!("✅ Password is valid!"),
            Err(e) => println!("❌ Password is invalid: {}", e),
        }
    }
}


