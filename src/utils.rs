// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(clippy::unnecessary_wraps, clippy::trivially_copy_pass_by_ref)]

use inquire::{CustomType, InquireError, Select, Text, validator::Validation};
use std::fmt::Display;

pub trait InquireExt<T> {
    fn prompt_ext(self) -> Result<Option<T>, InquireError>;
}

impl InquireExt<String> for Text<'_, '_> {
    fn prompt_ext(self) -> Result<Option<String>, InquireError> {
        match self.prompt() {
            Ok(t) => Ok(Some(t)),
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl<T> InquireExt<T> for CustomType<'_, T>
where
    T: Clone,
{
    fn prompt_ext(self) -> Result<Option<T>, InquireError> {
        match self.prompt() {
            Ok(t) => Ok(Some(t)),
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

impl<T> InquireExt<T> for Select<'_, T>
where
    T: Display,
{
    fn prompt_ext(self) -> Result<Option<T>, InquireError> {
        match self.prompt() {
            Ok(t) => Ok(Some(t)),
            Err(InquireError::OperationCanceled | InquireError::OperationInterrupted) => Ok(None),
            Err(e) => Err(e),
        }
    }
}

pub fn validate_text(input: &str) -> Result<Validation, Box<dyn std::error::Error + Send + Sync>> {
    if input.trim().is_empty() {
        Ok(Validation::Invalid("Description cannot be empty".into()))
    } else {
        Ok(Validation::Valid)
    }
}

pub fn validate_port(input: &u16) -> Result<Validation, Box<dyn std::error::Error + Send + Sync>> {
    if *input == 0 {
        Ok(Validation::Invalid("Port cannot be 0".into()))
    } else {
        Ok(Validation::Valid)
    }
}
