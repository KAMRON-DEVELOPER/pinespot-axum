use crate::utilities::errors::AppError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Sort {
    #[default]
    Newest,
    Cheap,
    Expensive,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Condition {
    #[default]
    Any,
    Old,
    Repaired,
    New,
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default, rename_all = "camelCase")]
pub struct SeachParams {
    pub min_beds: Option<i32>,
    pub min_baths: Option<i32>,
    pub max_price: Option<u64>,
    pub sort: Option<Sort>,
    pub condition: Option<Condition>,
    pub q: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Pagination {
    #[serde(default = "default_offset")]
    pub offset: i64,
    #[serde(default = "default_limit")]
    pub limit: i64,
}

fn default_offset() -> i64 {
    0
}

fn default_limit() -> i64 {
    20
}

impl Pagination {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.offset < 0 {
            return Err(AppError::ValidationError(
                "Offset must be positive".to_string(),
            ));
        }

        if self.limit < 0 {
            return Err(AppError::ValidationError("Limit must positive".to_string()));
        } else if self.limit == 0 {
            return Err(AppError::ValidationError(
                "Limit must not be zero!".to_string(),
            ));
        }

        if self.limit > 100 {
            return Err(AppError::ValidationError(
                "Limit cannot exceed 100".to_string(),
            ));
        }

        Ok(())
    }
}
