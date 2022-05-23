table! {
    license_capacity (id) {
        id -> Integer,
        license_id -> Text,
        license_key -> Text,
        product_id -> Text,
        capacity -> Integer,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    product (id) {
        id -> Integer,
        product_id -> Text,
        name -> Text,
        description -> Text,
        version -> Text,
        status -> Text,
        price -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
        password -> Text,
        role -> Text,
        is_active -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_address (id) {
        id -> Integer,
        user_id -> Integer,
        street -> Text,
        city -> Text,
        state -> Text,
        zip -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_license (id) {
        id -> Integer,
        license_key -> Text,
        license_id -> Text,
        user_id -> Integer,
        product_id -> Text,
        active_count -> Integer,
        used_count -> Integer,
        expire_at -> Timestamp,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_license_active_log (id) {
        id -> Integer,
        license_id -> Text,
        user_id -> Integer,
        device_id -> Text,
        device_type -> Text,
        device_meta -> Text,
        active_at -> Timestamp,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_order (id) {
        id -> Integer,
        order_id -> Integer,
        user_id -> Integer,
        payment_id -> Integer,
        total -> Integer,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_order_item (id) {
        id -> Integer,
        order_id -> Integer,
        product_id -> Integer,
        price -> Integer,
        amount -> Integer,
        discount -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_payment (id) {
        id -> Integer,
        payment_id -> Integer,
        order_id -> Integer,
        user_id -> Integer,
        pay_type -> Integer,
        pay_sn -> Text,
        total -> Integer,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_profile (id) {
        id -> Integer,
        user_id -> Integer,
        first_name -> Text,
        last_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    license_capacity,
    product,
    user,
    user_address,
    user_license,
    user_license_active_log,
    user_order,
    user_order_item,
    user_payment,
    user_profile,
);
