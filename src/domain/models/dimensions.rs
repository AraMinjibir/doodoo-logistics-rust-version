#[derive(Debug, Clone)]
pub struct Dimensions {
    length_in_centimeters: f64,
    width_in_centimeters: f64,
    height_in_centimeters: f64,
}

impl Dimensions {
    pub fn create(
        length: f64,
        width: f64,
        height: f64,
    ) -> Result<Self, Vec<String>> {
        let mut errors = Vec::new();

        if length <= 0.0 {
            errors.push(format!("Length must be > 0: {}", length));
        }
        if width <= 0.0 {
            errors.push(format!("Width must be > 0: {}", width));
        }
        if height <= 0.0 {
            errors.push(format!("Height must be > 0: {}", height));
        }

        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(Self {
            length_in_centimeters: length,
            width_in_centimeters: width,
            height_in_centimeters: height,
        })
    }
}