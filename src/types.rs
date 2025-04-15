/*
 * Copyright (c) David Lin
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const COLOR_LENGTH: usize = 7;
const ALPHANUMERIC_COUNT: usize = 6;
use crate::errors::ValidationError;

// implementing a validation trait to help ensure that all strings input by the user of this crate
// will be in the #hex color code format
pub trait Validation {
    fn validate_color_code(&self) -> Result<bool, ValidationError>;
}

impl Validation for &str {
    fn validate_color_code(&self) -> Result<bool, ValidationError> {
        // to check if our color starts with a # (i.e hex code format)
        if self.chars().nth(0) != Some('#') {
            return Err(ValidationError::new(
                "Color should be in the #hex format (e.g #000000) for black",
            ));
        }

        // color should have a length of 7 (e.g #123456)
        if self.len() != COLOR_LENGTH {
            return Err(ValidationError::new(
                "Color should be in the #hex format (e.g #000000) for black",
            ));
        }

        let mut split_color = self.split("#");
        split_color.next();
        let color_code_chars = split_color
            .next()
            .expect("Invalid color. It should be in the #hex format (e.g #000000 for black)")
            .chars();

        // we filter away any character(s) here that isn't alphanumeric
        let filtered_chars = color_code_chars
            .filter(|char| char.is_alphanumeric())
            .collect::<Vec<_>>();
        if filtered_chars.len() != ALPHANUMERIC_COUNT {
            return Err(ValidationError::new(
                "Color should be in the #hex format (e.g #000000) for black",
            ));
        }

        // if all validations passed for this color code
        Ok(true)
    }
}
