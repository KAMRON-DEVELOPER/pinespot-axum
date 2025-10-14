use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Type, Deserialize, Serialize, PartialEq, Eq, Default, Debug)]
#[sqlx(type_name = "apartment_condition", rename_all = "lowercase")]
#[sqlx(default)]
pub enum ApartmentCondition {
    #[default]
    New,
    Repaired,
    Old,
}

#[derive(Type, Deserialize, Serialize, PartialEq, Eq, Default, Debug)]
#[sqlx(type_name = "sale_type", rename_all = "lowercase")]
#[sqlx(default)]
pub enum SaleType {
    #[default]
    Buy,
    Rent,
}

#[derive(FromRow, Deserialize, Serialize, PartialEq, Default, Debug)]
#[sqlx(default)]
pub struct Listing {
    pub id: Uuid,
    pub apartment_id: Uuid,
    pub owner_id: Uuid,
    pub price: BigDecimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Deserialize, Serialize, PartialEq, Default, Debug)]
#[sqlx(default)]
pub struct ListingTag {
    pub id: Uuid,
    pub listing_id: Uuid,
    pub tag: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Deserialize, Serialize, PartialEq, Default, Debug)]
#[sqlx(default)]
pub struct Apartment {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub rooms: i64,
    pub beds: i64,
    pub baths: i64,
    pub area: f64,
    pub floor: i64,
    pub has_elevator: bool,
    pub condition: ApartmentCondition,
    pub sale_type: SaleType,
    pub requirements: Option<String>,
    pub has_garden: bool,
    pub distance_to_kindergarten: i64,
    pub distance_to_school: i64,
    pub distance_to_hospital: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Deserialize, Serialize, PartialEq, Default, Debug)]
#[sqlx(default)]
pub struct ApartmentPicture {
    pub id: Uuid,
    pub apartment_id: Uuid,
    pub url: String,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Deserialize, Serialize, PartialEq, Default, Debug)]
#[sqlx(default)]
pub struct ApartmentAmenity {
    pub id: Uuid,
    pub apartment_id: Uuid,
    pub amenity: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Deserialize, Serialize, PartialEq, Default, Debug)]
#[sqlx(default)]
pub struct Address {
    pub id: Uuid,
    pub apartment_id: Uuid,
    pub street_address: String,
    pub city: String,
    pub state_or_region: String,
    pub county_or_district: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub latitude: f64,
    pub longitude: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Deserialize, Serialize, PartialEq, Eq, Default, Debug)]
#[sqlx(default)]
pub struct Favorite {
    pub id: Uuid,
    pub user_id: Uuid,
    pub listing_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
