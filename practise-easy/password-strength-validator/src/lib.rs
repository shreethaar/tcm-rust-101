use std::collections::HashSet;

pub struct PasswordValidator {
    min_length: usize,
    require_numbers: bool,
    require_special: bool,
}

impl PasswordValidator {
    pub fn new(min_length: usize, require_numbers: bool, require_special: bool) -> Self {
        PasswordValidator {
            min_length,
            require_numbers,
            require_special,
        }
    }

    pub fn validate_password(&self, password: &str) -> Result<(), String> {
        if password.len() < self.min_length {
            return Err(format!(
                "Password must be at least {} characters long",
                self.min_length
            ));
        }

        if self.require_numbers && !password.chars().any(|c| c.is_numeric()) {
            return Err("Password must contain at least one number".to_string());
        }

        if self.require_special {
            let special_chars: HashSet<char> = "!@#$%^&*()_+-=[]{}|;:,.<>?".chars().collect();
            if !password.chars().any(|c| special_chars.contains(&c)) {
                return Err("Password must contain at least one special character".to_string());
            }
        }

        Ok(())
    }

    pub fn is_compromised(&self, password: &str) -> bool {
        let common_passwords = ["password", "123456", "qwerty", "letmein", "admin"];
        common_passwords.contains(&password)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        let validator = PasswordValidator::new(8, true, true);
        assert!(validator.validate_password("Secure1!Pass").is_ok());
    }

    #[test]
    fn test_short_password() {
        let validator = PasswordValidator::new(8, true, true);
        assert!(validator.validate_password("Sh0rt!").is_err());
    }

    #[test]
    fn test_missing_number() {
        let validator = PasswordValidator::new(8, true, true);
        assert!(validator.validate_password("SecurePass!").is_err());
    }

    #[test]
    fn test_missing_special() {
        let validator = PasswordValidator::new(8, true, true);
        assert!(validator.validate_password("SecurePass1").is_err());
    }

    #[test]
    fn test_compromised_password() {
        let validator = PasswordValidator::new(8, true, true);
        assert!(validator.is_compromised("password"));
        assert!(!validator.is_compromised("Secure1!Pass"));
    }
}
