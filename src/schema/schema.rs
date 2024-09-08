// @generated automatically by Diesel CLI.

diesel::table! {
    address_assignments (id) {
        id -> Int4,
        professional_profile_id -> Int4,
        address_id -> Int4,
    }
}

diesel::table! {
    addresses (id) {
        id -> Int4,
        street -> Varchar,
        city -> Varchar,
        state -> Varchar,
        zip -> Varchar,
        lng -> Float8,
        lat -> Float8,
    }
}

diesel::table! {
    booking_assignments (id) {
        id -> Int4,
        appointment_id -> Int4,
        #[max_length = 255]
        image_url -> Varchar,
    }
}

diesel::table! {
    booking_status (id) {
        id -> Int4,
        #[max_length = 255]
        description -> Varchar,
    }
}

diesel::table! {
    bookings (id) {
        id -> Int4,
        #[max_length = 255]
        customer_uid -> Varchar,
        #[max_length = 255]
        professional_profile_uid -> Varchar,
        date_time -> Nullable<Timestamptz>,
        status -> Int4,
        description -> Nullable<Text>,
        category_id -> Int4,
        end_time -> Nullable<Timestamptz>,
        service_offering_id -> Nullable<Int4>,
        offering_price -> Float8,
        chat_id -> Int4,
        creation_time -> Timestamptz,
    }
}

diesel::table! {
    business_hours (id) {
        id -> Int4,
        professional_profile_id -> Int4,
        day_of_week -> Int4,
        opening_time -> Nullable<Time>,
        closing_time -> Nullable<Time>,
        is_available -> Bool,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    chat (id) {
        id -> Int4,
        last_message_time -> Timestamp,
        #[max_length = 255]
        user_uid -> Varchar,
        #[max_length = 255]
        professional_profile_uid -> Varchar,
    }
}

diesel::table! {
    message (id) {
        id -> Int4,
        chat_id -> Int4,
        text -> Text,
        timestamp -> Timestamp,
        is_read -> Bool,
        #[max_length = 255]
        receiver_uid -> Varchar,
        #[max_length = 255]
        sender_uid -> Varchar,
    }
}

diesel::table! {
    message_assignments (id) {
        id -> Int4,
        message_id -> Int4,
        #[max_length = 255]
        image_url -> Varchar,
    }
}

diesel::table! {
    professional_profiles (id) {
        id -> Int4,
        professional_id -> Int4,
        category_id -> Int4,
        credentials -> Nullable<Text>,
        delivery_enabled -> Bool,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        average_rating -> Nullable<Float8>,
        remote_available -> Bool,
        professional_name -> Varchar,
        #[max_length = 255]
        professional_profile_uid -> Varchar,
    }
}

diesel::table! {
    professionals (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        #[max_length = 255]
        user_uid -> Varchar,
    }
}

diesel::table! {
    review (id) {
        id -> Int4,
        user_id -> Int4,
        professional_profile_id -> Int4,
        message -> Text,
        rate -> Float8,
        #[max_length = 255]
        user_name -> Varchar,
        published_at -> Timestamptz,
    }
}

diesel::table! {
    review_content_assignments (id) {
        id -> Int4,
        review_id -> Int4,
        #[max_length = 255]
        image_url -> Varchar,
    }
}

diesel::table! {
    service_offerings (id) {
        id -> Int4,
        professional_profile_id -> Int4,
        subcategory_id -> Int4,
        price -> Float8,
        #[max_length = 255]
        subcategory_name -> Varchar,
    }
}

diesel::table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::table! {
    subcategories (id) {
        id -> Int4,
        name -> Varchar,
        category_id -> Int4,
    }
}

diesel::table! {
    task (id) {
        id -> Int4,
        #[max_length = 255]
        user_uid -> Varchar,
        creation_time -> Timestamptz,
        description -> Nullable<Text>,
        address_id -> Nullable<Int4>,
        title -> Varchar,
        min_price -> Nullable<Float8>,
        max_price -> Nullable<Float8>,
        is_flexible_timing -> Bool,
        scheduled_date -> Nullable<Date>,
        scheduled_time -> Nullable<Time>,
        category_id -> Int4,
    }
}

diesel::table! {
    task_assignments (id) {
        id -> Int4,
        task_id -> Int4,
        image_url -> Varchar,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        #[max_length = 255]
        user_uid -> Varchar,
    }
}

diesel::joinable!(address_assignments -> addresses (address_id));
diesel::joinable!(address_assignments -> professional_profiles (professional_profile_id));
diesel::joinable!(booking_assignments -> bookings (appointment_id));
diesel::joinable!(bookings -> chat (chat_id));
diesel::joinable!(bookings -> service_offerings (service_offering_id));
diesel::joinable!(business_hours -> professional_profiles (professional_profile_id));
diesel::joinable!(message -> chat (chat_id));
diesel::joinable!(message_assignments -> message (message_id));
diesel::joinable!(professional_profiles -> professionals (professional_id));
diesel::joinable!(review -> professional_profiles (professional_profile_id));
diesel::joinable!(review -> users (user_id));
diesel::joinable!(review_content_assignments -> review (review_id));
diesel::joinable!(service_offerings -> professional_profiles (professional_profile_id));
diesel::joinable!(service_offerings -> subcategories (subcategory_id));
diesel::joinable!(subcategories -> categories (category_id));
diesel::joinable!(task -> addresses (address_id));
diesel::joinable!(task_assignments -> task (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    address_assignments,
    addresses,
    booking_assignments,
    booking_status,
    bookings,
    business_hours,
    categories,
    chat,
    message,
    message_assignments,
    professional_profiles,
    professionals,
    review,
    review_content_assignments,
    service_offerings,
    spatial_ref_sys,
    subcategories,
    task,
    task_assignments,
    users,
);
