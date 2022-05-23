table! {
    cart (id) {
        id -> Unsigned<Integer>,
        buyer_id -> Unsigned<Integer>,
        store_id -> Unsigned<Integer>,
        store_name -> Varchar,
        product_id -> Unsigned<Integer>,
        product_title -> Varchar,
        product_image -> Varchar,
        product_price -> Unsigned<Decimal>,
        product_amount -> Unsigned<Integer>,
        product_discount -> Unsigned<Decimal>,
        product_total -> Unsigned<Decimal>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    comment (id) {
        id -> Unsigned<Integer>,
        comment_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        relate_id -> Unsigned<Integer>,
        #[sql_name = "type"]
        type_ -> Unsigned<Integer>,
        content -> Varchar,
        score -> Unsigned<Integer>,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
    order (id) {
        id -> Unsigned<Integer>,
        order_id -> Unsigned<Integer>,
        buyer_id -> Unsigned<Integer>,
        payment_id -> Unsigned<Integer>,
        total -> Unsigned<Decimal>,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    order_item (id) {
        id -> Unsigned<Integer>,
        item_id -> Unsigned<Integer>,
        order_id -> Unsigned<Integer>,
        buyer_id -> Unsigned<Integer>,
        shop_id -> Unsigned<Integer>,
        product_id -> Unsigned<Integer>,
        quantity -> Unsigned<Integer>,
        price -> Unsigned<Decimal>,
        discount -> Unsigned<Decimal>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    order_template (id) {
        id -> Unsigned<Integer>,
        mid -> Unsigned<Integer>,
        order_no -> Varchar,
        merchant_id -> Unsigned<Integer>,
        product_id -> Unsigned<Integer>,
        product_type -> Unsigned<Tinyint>,
        product_sku -> Unsigned<Integer>,
        product_title -> Varchar,
        product_num -> Unsigned<Integer>,
        product_price -> Decimal,
        product_discount -> Decimal,
        cost -> Decimal,
        status -> Integer,
        ver -> Integer,
        ctime -> Timestamp,
        mtime -> Timestamp,
    }
}

table! {
    product (id) {
        id -> Unsigned<Integer>,
        product_id -> Unsigned<Integer>,
        title -> Varchar,
        brief -> Varchar,
        description -> Varchar,
        version -> Varchar,
        price -> Unsigned<Decimal>,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    shop (id) {
        id -> Unsigned<Integer>,
        shop_id -> Unsigned<Integer>,
        owner_id -> Unsigned<Integer>,
        name -> Varchar,
        title -> Varchar,
        level -> Unsigned<Integer>,
        description -> Varchar,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    shop_bill_daily (id) {
        id -> Unsigned<Integer>,
        shop_id -> Unsigned<Integer>,
        income -> Decimal,
        date -> Date,
        ver -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    shop_employee (id) {
        id -> Unsigned<Integer>,
        shop_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        name -> Varchar,
        remark -> Varchar,
        role -> Unsigned<Integer>,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    shop_order (id) {
        id -> Unsigned<Integer>,
        shop_id -> Unsigned<Integer>,
        order_id -> Unsigned<Integer>,
        order_item_id -> Unsigned<Integer>,
        income -> Decimal,
        ver -> Integer,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    shop_product (id) {
        id -> Unsigned<Integer>,
        shop_id -> Unsigned<Integer>,
        product_id -> Unsigned<Integer>,
        stock -> Unsigned<Integer>,
        ver -> Integer,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        role -> Integer,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_3rd_account (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        provider -> Varchar,
        openid -> Varchar,
        access_token -> Varchar,
        refresh_token -> Varchar,
        expires_in -> Unsigned<Integer>,
        scope -> Varchar,
        meta -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_address (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        order -> Unsigned<Integer>,
        label -> Varchar,
        name -> Varchar,
        mobile -> Varchar,
        desc -> Varchar,
        country -> Varchar,
        province -> Varchar,
        city -> Varchar,
        district -> Varchar,
        street -> Varchar,
        address -> Varchar,
        zipcode -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_auth (id) {
        id -> Unsigned<Integer>,
        auth_id -> Varchar,
        auth_provider -> Unsigned<Tinyint>,
        user_id -> Unsigned<Integer>,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_auth_log (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        ip -> Varchar,
        device -> Varchar,
        meta -> Varchar,
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
    user_order_refund (id) {
        id -> Unsigned<Integer>,
        refund_id -> Unsigned<Integer>,
        order_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        refund_type -> Unsigned<Integer>,
        refund_sn -> Varchar,
        total -> Unsigned<Decimal>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_payment (id) {
        id -> Unsigned<Integer>,
        payment_id -> Unsigned<Integer>,
        order_id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        pay_type -> Unsigned<Integer>,
        pay_sn -> Varchar,
        total -> Unsigned<Decimal>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    user_profile (id) {
        id -> Unsigned<Integer>,
        user_id -> Unsigned<Integer>,
        username -> Varchar,
        email -> Varchar,
        mobile -> Varchar,
        nickname -> Varchar,
        avatar -> Varchar,
        real_name -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    cart,
    comment,
    license_capacity,
    order,
    order_item,
    order_template,
    product,
    shop,
    shop_bill_daily,
    shop_employee,
    shop_order,
    shop_product,
    user,
    user_3rd_account,
    user_address,
    user_auth,
    user_auth_log,
    user_license,
    user_license_active_log,
    user_order_refund,
    user_payment,
    user_profile,
);
