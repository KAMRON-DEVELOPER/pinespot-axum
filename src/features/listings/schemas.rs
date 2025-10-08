use crate::features::{
    listings::models::{ApartmentCondition, SaleType},
    users::schemas::UserOut,
};
use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct AmenityOut {
    pub id: Uuid,
    pub apartment_id: Uuid,
    pub amenity: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct PictureOut {
    pub id: Option<Uuid>,
    pub apartment_id: Option<Uuid>,
    pub url: Option<String>,
    pub is_primary: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct ApartmentOut {
    pub id: Option<Uuid>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub rooms: Option<i32>,
    pub beds: Option<i32>,
    pub baths: Option<i32>,
    pub address: Option<AddressOut>,
    pub pictures: Vec<PictureOut>,
    pub amenities: Vec<AmenityOut>,
    pub area: Option<f64>,
    pub floor: Option<i32>,
    pub has_elevator: Option<bool>,
    pub condition: Option<ApartmentCondition>,
    pub sale_type: Option<SaleType>,
    pub requirements: Option<String>,
    pub has_garden: Option<bool>,
    pub distance_to_kindergarten: Option<i32>,
    pub distance_to_school: Option<i32>,
    pub distance_to_hospital: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct TagOut {
    pub id: Uuid,
    pub listing_id: Uuid,
    pub tag: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct ListingOut {
    pub id: Uuid,
    pub owner: UserOut,
    pub apartment: ApartmentOut,
    pub price: BigDecimal,
    pub currency: String,
    pub tags: Vec<TagOut>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct AddressOut {
    pub id: Option<Uuid>,
    pub apartment_id: Option<Uuid>,
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub state_or_region: Option<String>,
    pub county_or_district: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
pub struct ListingResponse {
    pub listings: Vec<ListingOut>,
    pub total: i64,
}
