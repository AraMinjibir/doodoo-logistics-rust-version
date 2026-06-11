use crate::domain::models::address::Address;

#[derive(Debug, Clone)]
pub struct Recipient {
    name: String,
    contact: String,
    address: Address,
}

impl Recipient {
    pub fn create(name: String, contact: String, address: Address) -> Result<Self, Vec<String>> {
        let mut errors = Vec::new();

        if name.trim().is_empty() {
            errors.push(format!("Name must not be empty: {}", name));
        }

        if contact.trim().is_empty() {
            errors.push(format!("Contact cannot be empty: {}", contact));
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            name,
            contact,
            address,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn contact(&self) -> &str {
        &self.contact
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}
