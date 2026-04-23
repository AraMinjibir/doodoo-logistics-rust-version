#[derive(Debug, Clone)]
pub struct Address {
    street: String,
    city: String,
    state: String,
    country: String,
    postal_code: String,
}

impl Address {
    pub fn create(
        street: String,
        city: String,
        state: String,
        country: String,
        postal_code: String,
    ) -> Result<Self, Vec<String>> {
        let mut errors = Vec::new();

        if street.trim().is_empty() {
            errors.push(format!("Street name must not be empty: {}", street));
        }

        if city.trim().is_empty() {
            errors.push(format!("City name must not be empty: {}", city));
        }

        if state.trim().is_empty() {
            errors.push(format!("State must not be empty: {}", state));
        }

        if country.trim().is_empty() {
            errors.push(format!("Country name must not be empty: {}", country));
        }

        if postal_code.trim().is_empty() {
            errors.push(format!("Postal Code must not be empty: {}", postal_code));
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            street,
            city,
            state,
            country,
            postal_code,
        })
    }
}