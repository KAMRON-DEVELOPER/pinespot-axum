use crate::features::listings::models::{ApartmentCondition, SaleType};
use crate::features::listings::schemas::{
    AddressOut, AmenityOut, ApartmentOut, ListingOut, PictureOut, TagOut,
};
use crate::features::schemas::Pagination;
use crate::features::users::models::{UserRole, UserStatus};
use crate::features::users::schemas::UserOut;
use chrono::{DateTime, Utc};

use sqlx::{FromRow, PgPool, types::BigDecimal, types::Json};
use uuid::Uuid;

#[derive(FromRow)]
pub struct ListingJoined {
    pub listing_id: Uuid,
    pub price: BigDecimal,
    pub currency: String,
    pub listing_created_at: DateTime<Utc>,
    pub listing_updated_at: DateTime<Utc>,

    // owner
    pub owner_id: Uuid,
    pub owner_full_name: String,
    pub owner_email: String,
    pub owner_phone: String,
    pub owner_picture: Option<String>,
    pub owner_role: UserRole,
    pub owner_status: UserStatus,
    pub owner_email_verified: bool,
    pub owner_oauth_user_id: Option<String>,
    pub owner_created_at: DateTime<Utc>,
    pub owner_updated_at: DateTime<Utc>,

    // apartment
    pub apartment_id: Option<Uuid>,
    pub apartment_title: Option<String>,
    pub apartment_description: Option<String>,
    pub apartment_rooms: Option<i32>,
    pub apartment_beds: Option<i32>,
    pub apartment_baths: Option<i32>,
    pub apartment_area: Option<f64>,
    pub apartment_floor: Option<i32>,
    pub apartment_has_elevator: Option<bool>,
    pub apartment_condition: Option<ApartmentCondition>,
    pub apartment_sale_type: Option<SaleType>,
    pub apartment_requirements: Option<String>,
    pub apartment_has_garden: Option<bool>,
    pub distance_to_kindergarten: Option<i32>,
    pub distance_to_school: Option<i32>,
    pub distance_to_hospital: Option<i32>,
    pub apartment_created_at: Option<DateTime<Utc>>,
    pub apartment_updated_at: Option<DateTime<Utc>>,

    // address
    pub address_id: Option<Uuid>,
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub state_or_region: Option<String>,
    pub county_or_district: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub address_created_at: Option<DateTime<Utc>>,
    pub address_updated_at: Option<DateTime<Utc>>,

    // arrays
    pub tags: Option<Json<Vec<TagOut>>>,
    pub amenities: Option<Json<Vec<AmenityOut>>>,
    pub pictures: Option<Json<Vec<PictureOut>>>,
    // pub tags: Option<Vec<String>>,
    // pub amenities: Option<Vec<String>>,
    // pub pictures: Option<Vec<String>>,
}

pub async fn get_all_listings(
    pool: &PgPool,
    pagination: &Pagination,
) -> Result<Vec<ListingOut>, sqlx::Error> {
    // let rows = sqlx::query_as!(
    //     ListingJoined,
    //     r#"
    //     SELECT
    //         l.id AS listing_id,
    //         l.price,
    //         l.currency,
    //         l.created_at AS listing_created_at,
    //         l.updated_at AS listing_updated_at,

    //         u.id AS owner_id,
    //         u.full_name AS owner_full_name,
    //         u.email AS owner_email,
    //         u.phone_number AS owner_phone,
    //         u.picture AS owner_picture,
    //         u.role AS "owner_role: UserRole",
    //         u.status AS "owner_status: UserStatus",
    //         u.email_verified AS owner_email_verified,
    //         u.oauth_user_id AS owner_oauth_user_id,
    //         u.created_at AS owner_created_at,
    //         u.updated_at AS owner_updated_at,

    //         a.id AS apartment_id,
    //         a.title AS apartment_title,
    //         a.description AS apartment_description,
    //         a.rooms AS apartment_rooms,
    //         a.beds AS apartment_beds,
    //         a.baths AS apartment_baths,
    //         a.area AS apartment_area,
    //         a.floor AS apartment_floor,
    //         a.has_elevator AS apartment_has_elevator,
    //         a.condition AS "apartment_condition: ApartmentCondition",
    //         a.sale_type AS "apartment_sale_type: SaleType",
    //         a.requirements AS apartment_requirements,
    //         a.has_garden AS apartment_has_garden,
    //         a.distance_to_kindergarten,
    //         a.distance_to_school,
    //         a.distance_to_hospital,
    //         a.created_at AS apartment_created_at,
    //         a.updated_at AS apartment_updated_at,

    //         ad.id AS address_id,
    //         ad.street_address,
    //         ad.city,
    //         ad.state_or_region,
    //         ad.county_or_district,
    //         ad.postal_code,
    //         ad.country,
    //         ad.latitude,
    //         ad.longitude,
    //         ad.created_at AS address_created_at,
    //         ad.updated_at AS address_updated_at,

    //         COALESCE(
    //             ARRAY_AGG(DISTINCT lt.tag) FILTER (WHERE lt.id IS NOT NULL),
    //             '{}'
    //         ) AS tags,

    //         COALESCE(
    //             ARRAY_AGG(DISTINCT aa.amenity) FILTER (WHERE aa.id IS NOT NULL),
    //             '{}'
    //         ) AS amenities,

    //         COALESCE(
    //             ARRAY_AGG(DISTINCT ap.url) FILTER (WHERE ap.id IS NOT NULL),
    //             '{}'
    //         ) AS pictures

    //     FROM listings l
    //     JOIN users u ON u.id = l.owner_id
    //     JOIN apartments a ON a.id = l.apartment_id
    //     LEFT JOIN addresses ad ON ad.apartment_id = a.id
    //     LEFT JOIN listing_tags lt ON lt.listing_id = l.id
    //     LEFT JOIN apartment_amenities aa ON aa.apartment_id = a.id
    //     LEFT JOIN apartment_pictures ap ON ap.apartment_id = a.id
    //     GROUP BY
    //         l.id, u.id, a.id, ad.id
    //     OFFSET $1 LIMIT $2
    //     "#,
    //     pagination.offset,
    //     pagination.limit
    // )
    // .fetch_all(pool)
    // .await?;

    let rows = sqlx::query_as!(
        ListingJoined,
        r#"
        SELECT
            l.id AS listing_id,
            l.price,
            l.currency,
            l.created_at AS listing_created_at,
            l.updated_at AS listing_updated_at,

            -- owner fields ...
            u.id AS owner_id,
            u.full_name AS owner_full_name,
            u.email AS owner_email,
            u.phone_number AS owner_phone,
            u.picture AS owner_picture,
            u.role AS "owner_role: UserRole",
            u.status AS "owner_status: UserStatus",
            u.email_verified AS owner_email_verified,
            u.oauth_user_id AS owner_oauth_user_id,
            u.created_at AS owner_created_at,
            u.updated_at AS owner_updated_at,

            -- apartment fields ...
            a.id AS apartment_id,
            a.title AS apartment_title,
            a.description AS apartment_description,
            a.rooms AS apartment_rooms,
            a.beds AS apartment_beds,
            a.baths AS apartment_baths,
            a.area AS apartment_area,
            a.floor AS apartment_floor,
            a.has_elevator AS apartment_has_elevator,
            a.condition AS "apartment_condition: ApartmentCondition",
            a.sale_type AS "apartment_sale_type: SaleType",
            a.requirements AS apartment_requirements,
            a.has_garden AS apartment_has_garden,
            a.distance_to_kindergarten,
            a.distance_to_school,
            a.distance_to_hospital,
            a.created_at AS apartment_created_at,
            a.updated_at AS apartment_updated_at,

            -- address fields ...
            ad.id AS address_id,
            ad.street_address,
            ad.city,
            ad.state_or_region,
            ad.county_or_district,
            ad.postal_code,
            ad.country,
            ad.latitude,
            ad.longitude,
            ad.created_at AS address_created_at,
            ad.updated_at AS address_updated_at,

            -- UPDATED JSON AGGREGATIONS --

            COALESCE(
                (SELECT jsonb_agg(jsonb_build_object(
                    'id', lt.id,
                    'listing_id', lt.listing_id,
                    'tag', lt.tag,
                    'created_at', lt.created_at,
                    'updated_at', lt.updated_at
                )) FROM listing_tags lt WHERE lt.listing_id = l.id),
                '[]'::jsonb
            ) AS "tags: Json<Vec<TagOut>>",

            COALESCE(
                (SELECT jsonb_agg(jsonb_build_object(
                    'id', aa.id,
                    'apartment_id', aa.apartment_id,
                    'amenity', aa.amenity,
                    'created_at', aa.created_at,
                    'updated_at', aa.updated_at
                )) FROM apartment_amenities aa WHERE aa.apartment_id = a.id),
                '[]'::jsonb
            ) AS "amenities: Json<Vec<AmenityOut>>",

            COALESCE(
                (SELECT jsonb_agg(jsonb_build_object(
                    'id', ap.id,
                    'apartment_id', ap.apartment_id,
                    'url', ap.url,
                    'is_primary', ap.is_primary,
                    'created_at', ap.created_at,
                    'updated_at', ap.updated_at
                )) FROM apartment_pictures ap WHERE ap.apartment_id = a.id),
                '[]'::jsonb
            ) AS "pictures: Json<Vec<PictureOut>>"

        FROM listings l
        JOIN users u ON u.id = l.owner_id
        JOIN apartments a ON a.id = l.apartment_id
        LEFT JOIN addresses ad ON ad.apartment_id = a.id
        GROUP BY
            l.id, u.id, a.id, ad.id
        ORDER BY l.created_at DESC
        OFFSET $1 LIMIT $2
        "#,
        pagination.offset,
        pagination.limit
    )
    .fetch_all(pool)
    .await?;

    let listings = rows
        .into_iter()
        .map(|row| ListingOut {
            id: row.listing_id,
            price: row.price,
            currency: row.currency,
            created_at: row.listing_created_at,
            updated_at: row.listing_updated_at,
            owner: UserOut {
                id: row.owner_id,
                full_name: row.owner_full_name,
                email: row.owner_email,
                phone_number: row.owner_phone,
                picture: row.owner_picture,
                role: row.owner_role,
                status: row.owner_status,
                email_verified: row.owner_email_verified,
                oauth_user_id: row.owner_oauth_user_id,
                created_at: row.owner_created_at,
                updated_at: row.owner_updated_at,
            },
            apartment: ApartmentOut {
                id: row.apartment_id,
                title: row.apartment_title,
                description: row.apartment_description,
                rooms: row.apartment_rooms,
                beds: row.apartment_beds,
                baths: row.apartment_baths,
                address: Some(AddressOut {
                    id: row.address_id,
                    apartment_id: row.apartment_id,
                    street_address: row.street_address,
                    city: row.city,
                    state_or_region: row.state_or_region,
                    county_or_district: row.county_or_district,
                    postal_code: row.postal_code,
                    country: row.country,
                    latitude: row.latitude,
                    longitude: row.longitude,
                    created_at: row.address_created_at,
                    updated_at: row.address_updated_at,
                }),
                pictures: row.pictures.map(|p| p.0).unwrap_or_default(),
                amenities: row.amenities.map(|a| a.0).unwrap_or_default(),
                area: row.apartment_area,
                floor: row.apartment_floor,
                has_elevator: row.apartment_has_elevator,
                condition: row.apartment_condition,
                sale_type: row.apartment_sale_type,
                requirements: row.apartment_requirements,
                has_garden: row.apartment_has_garden,
                distance_to_kindergarten: row.distance_to_kindergarten,
                distance_to_school: row.distance_to_school,
                distance_to_hospital: row.distance_to_hospital,
                created_at: row.apartment_created_at,
                updated_at: row.apartment_updated_at,
            },
            tags: row.tags.map(|t| t.0).unwrap_or_default(),
        })
        .collect();

    Ok(listings)
}
