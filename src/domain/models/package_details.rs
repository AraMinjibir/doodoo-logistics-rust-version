use crate::domain::models::dimensions::Dimensions;


#[derive(Debug, Clone)]
pub struct PackageDetails {
    weight_in_kilograms: f64,
    dimensions: Dimensions,
    contents: String,
}

impl PackageDetails {
    pub fn create(
        weight: f64,
        dimensions: Dimensions,
        contents: String,
    ) -> Result<Self, Vec<String>> {
        let mut errors = Vec::new();

        if weight <= 0.0 {
            errors.push(format!("Weight must be greater than 0: {}", weight));
        }

        if contents.trim().is_empty() {
            errors.push(format!("Content must not be empty: {}", contents));
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            weight_in_kilograms: weight,
            dimensions,
            contents,
        })
    }

    pub fn weight(&self) -> f64 {
        self.weight_in_kilograms
    }
}
