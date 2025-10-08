-- We define CTEs to gather the arrays of related items first.
WITH -- 1. Aggregate all pictures for each apartment
apartment_pictures_agg AS (
    SELECT apartment_id,
        JSON_AGG(
            JSON_BUILD_OBJECT(
                'id',
                id,
                'apartmentId',
                apartment_id,
                'url',
                url,
                'isPrimary',
                is_primary,
                'createdAt',
                created_at,
                'updatedAt',
                updated_at
            )
            ORDER BY is_primary DESC,
                created_at
        ) AS pictures
    FROM apartment_pictures
    GROUP BY apartment_id
),
-- 2. Aggregate all amenities for each apartment
apartment_amenities_agg AS (
    SELECT apartment_id,
        JSON_AGG(
            JSON_BUILD_OBJECT(
                'id',
                id,
                'apartmentId',
                apartment_id,
                'amenity',
                amenity,
                'createdAt',
                created_at,
                'updatedAt',
                updated_at
            )
            ORDER BY created_at
        ) AS amenities
    FROM apartment_amenities
    GROUP BY apartment_id
),
-- 3. Aggregate all tags for each listing
listing_tags_agg AS (
    SELECT listing_id,
        JSON_AGG(
            JSON_BUILD_OBJECT(
                'id',
                id,
                'listingId',
                listing_id,
                'tag',
                tag,
                'createdAt',
                created_at,
                'updatedAt',
                updated_at
            )
            ORDER BY created_at
        ) AS tags
    FROM listing_tags
    GROUP BY listing_id
) -- 4. The main query to build the final ListingOut object
SELECT JSON_BUILD_OBJECT(
        'id',
        l.id,
        'price',
        l.price,
        'currency',
        l.currency,
        'createdAt',
        l.created_at,
        'updatedAt',
        l.updated_at,
        -- Owner (UserOut) object
        'owner',
        JSON_BUILD_OBJECT(
            'id',
            o.id,
            'fullName',
            o.full_name,
            'email',
            o.email,
            'phoneNumber',
            o.phone_number,
            'picture',
            o.picture,
            'role',
            o.role,
            'status',
            o.status,
            'emailVerified',
            o.email_verified,
            'oauthUserId',
            o.oauth_user_id,
            'createdAt',
            o.created_at,
            'updatedAt',
            o.updated_at
        ),
        -- Apartment (ApartmentOut) object
        'apartment',
        JSON_BUILD_OBJECT(
            'id',
            a.id,
            'title',
            a.title,
            'description',
            a.description,
            'rooms',
            a.rooms,
            'beds',
            a.beds,
            'baths',
            a.baths,
            'area',
            a.area,
            'floor',
            a.floor,
            'hasElevator',
            a.has_elevator,
            'condition',
            a.condition,
            'saleType',
            a.sale_type,
            'requirements',
            a.requirements,
            'hasGarden',
            a.has_garden,
            'distanceToKindergarten',
            a.distance_to_kindergarten,
            'distanceToSchool',
            a.distance_to_school,
            'distanceToHospital',
            a.distance_to_hospital,
            'createdAt',
            a.created_at,
            'updatedAt',
            a.updated_at,
            -- Address object (nested inside apartment)
            'address',
            JSON_BUILD_OBJECT(
                'id',
                addr.id,
                'apartmentId',
                addr.apartment_id,
                'streetAddress',
                addr.street_address,
                'city',
                addr.city,
                'stateOrRegion',
                addr.state_or_region,
                'countyOrDistrict',
                addr.county_or_district,
                'postalCode',
                addr.postal_code,
                'country',
                addr.country,
                'latitude',
                addr.latitude,
                'longitude',
                addr.longitude,
                'createdAt',
                addr.created_at,
                'updatedAt',
                addr.updated_at
            ),
            -- Arrays from our CTEs
            'pictures',
            COALESCE(apa.pictures, '[]'::json),
            'amenities',
            COALESCE(ama.amenities, '[]'::json)
        ),
        -- Tags array from our CTE
        'tags',
        COALESCE(lta.tags, '[]'::json)
    ) AS listing_data
FROM listings l
    INNER JOIN users o ON l.owner_id = o.id
    INNER JOIN apartments a ON l.apartment_id = a.id
    INNER JOIN addresses addr ON a.id = addr.apartment_id
    LEFT JOIN apartment_pictures_agg apa ON a.id = apa.apartment_id
    LEFT JOIN apartment_amenities_agg ama ON a.id = ama.apartment_id
    LEFT JOIN listing_tags_agg lta ON l.id = lta.listing_id -- Add WHERE, LIMIT, OFFSET for filtering and pagination
    -- For fetching a single listing:
    -- WHERE l.id = $1
    -- For fetching a list:
    -- LIMIT $1 OFFSET $2