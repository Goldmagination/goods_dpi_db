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
    appointment_assignments (id) {
        id -> Int4,
        appointment_id -> Int4,
        #[max_length = 255]
        image_url -> Varchar,
    }
}

diesel::table! {
    appointments (id) {
        id -> Int4,
        customer_id -> Int4,
        professional_profile_id -> Int4,
        date_time -> Timestamp,
        status -> Int4,
        message -> Nullable<Text>,
        category_id -> Int4,
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
        user_uid -> Uuid,
        professional_profile_uid -> Uuid,
    }
}

diesel::table! {
    message (id) {
        id -> Int4,
        chat_id -> Int4,
        text -> Text,
        timestamp -> Timestamp,
        is_read -> Bool,
        receiver_uid -> Uuid,
        sender_uid -> Uuid,
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
        professional_profile_uid -> Uuid,
    }
}

diesel::table! {
    professionals (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        user_uid -> Varchar,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
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
    tasks (id) {
        id -> Int4,
        user_id -> Int4,
        creation_time -> Timestamp,
        description -> Nullable<Text>,
        address_id -> Nullable<Int4>,
        price -> Nullable<Int4>,
    }
}

diesel::table! {
    tasks_subcategories (id) {
        id -> Int4,
        task_id -> Int4,
        subcategory_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        #[max_length = 255]
        image_url -> Nullable<Varchar>,
        user_uid -> Uuid,
    }
}

diesel::joinable!(address_assignments -> addresses (address_id));
diesel::joinable!(address_assignments -> professional_profiles (professional_profile_id));
diesel::joinable!(appointment_assignments -> appointments (appointment_id));
diesel::joinable!(appointments -> professional_profiles (professional_profile_id));
diesel::joinable!(appointments -> users (customer_id));
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
diesel::joinable!(tasks -> addresses (address_id));
diesel::joinable!(tasks -> users (user_id));
diesel::joinable!(tasks_subcategories -> subcategories (subcategory_id));
diesel::joinable!(tasks_subcategories -> tasks (task_id));

diesel::allow_tables_to_appear_in_same_query!(
    address_assignments,
    addresses,
    appointment_assignments,
    appointments,
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
    tasks,
    tasks_subcategories,
    users,
);
