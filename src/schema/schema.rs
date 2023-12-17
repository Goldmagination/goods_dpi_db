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
    }
}

diesel::table! {
    appointment_assignments (id) {
        id -> Int4,
        appointment_id -> Int4,
        photo_id -> Nullable<Int4>,
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
    categories (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    chat (id) {
        id -> Int4,
        user_id -> Int4,
        professional_id -> Int4,
        title -> Varchar,
        last_message_time -> Timestamp,
    }
}

diesel::table! {
    message (id) {
        id -> Int4,
        chat_id -> Int4,
        sender_id -> Int4,
        text -> Text,
        timestamp -> Timestamp,
        is_read -> Bool,
    }
}

diesel::table! {
    message_assignments (id) {
        id -> Int4,
        message_id -> Int4,
        photo_id -> Int4,
    }
}

diesel::table! {
    order_subcategories (id) {
        id -> Int4,
        order_id -> Int4,
        subcategory_id -> Int4,
    }
}

diesel::table! {
    orders (id) {
        id -> Int4,
        user_id -> Int4,
        creation_time -> Timestamp,
        description -> Nullable<Text>,
        address_id -> Int4,
    }
}

diesel::table! {
    professional_profiles (id) {
        id -> Int4,
        professional_id -> Int4,
        category_id -> Int4,
        credentials -> Nullable<Text>,
        delivery_enabled -> Nullable<Bool>,
        photo_id -> Nullable<Int4>,
        average_rating -> Nullable<Numeric>,
    }
}

diesel::table! {
    professionals (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        user_uid -> Varchar,
        photo_id -> Nullable<Int4>,
    }
}

diesel::table! {
    review (id) {
        id -> Int4,
        user_id -> Int4,
        professional_profile_id -> Int4,
        message -> Text,
        rate -> Nullable<Numeric>,
    }
}

diesel::table! {
    review_content_assignments (id) {
        id -> Int4,
        review_id -> Int4,
        photo_id -> Int4,
    }
}

diesel::table! {
    service_offerings (id) {
        id -> Int4,
        professional_profile_id -> Int4,
        subcategory_id -> Int4,
        price -> Numeric,
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
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Varchar,
        user_uid -> Varchar,
        photo_id -> Nullable<Int4>,
    }
}

diesel::joinable!(address_assignments -> addresses (address_id));
diesel::joinable!(address_assignments -> professional_profiles (professional_profile_id));
diesel::joinable!(appointment_assignments -> appointments (appointment_id));
diesel::joinable!(appointments -> professional_profiles (professional_profile_id));
diesel::joinable!(appointments -> users (customer_id));
diesel::joinable!(chat -> professionals (professional_id));
diesel::joinable!(chat -> users (user_id));
diesel::joinable!(message -> chat (chat_id));
diesel::joinable!(message_assignments -> message (message_id));
diesel::joinable!(order_subcategories -> orders (order_id));
diesel::joinable!(order_subcategories -> subcategories (subcategory_id));
diesel::joinable!(orders -> addresses (address_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(professional_profiles -> professionals (professional_id));
diesel::joinable!(review -> professional_profiles (professional_profile_id));
diesel::joinable!(review -> users (user_id));
diesel::joinable!(review_content_assignments -> review (review_id));
diesel::joinable!(service_offerings -> professional_profiles (professional_profile_id));
diesel::joinable!(service_offerings -> subcategories (subcategory_id));
diesel::joinable!(subcategories -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    address_assignments,
    addresses,
    appointment_assignments,
    appointments,
    categories,
    chat,
    message,
    message_assignments,
    order_subcategories,
    orders,
    professional_profiles,
    professionals,
    review,
    review_content_assignments,
    service_offerings,
    subcategories,
    users,
);
