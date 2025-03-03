table! {
    brand (id) {
        id -> Unsigned<Integer>,
        brand_id -> Unsigned<Integer>,
        name -> Varchar,
        brief -> Varchar,
        description -> Varchar,
        logo -> Varchar,
        meta -> Varchar,
        status -> Integer,
        score -> Unsigned<Decimal>,
        recommend -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

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
    product_attribute (id) {
        id -> Unsigned<Integer>,
        product_id -> Unsigned<Integer>,
        lang_id -> Unsigned<Integer>,
        attr_key -> Varchar,
        attr_key_name -> Varchar,
        attr_value -> Varchar,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    product_category (id) {
        id -> Unsigned<Integer>,
        category_id -> Unsigned<Integer>,
        parent_id -> Unsigned<Integer>,
        level -> Unsigned<Integer>,
        order -> Unsigned<Integer>,
        name -> Varchar,
        icon -> Varchar,
        description -> Varchar,
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
        is_verified -> Unsigned<Tinyint>,
        is_official -> Unsigned<Tinyint>,
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
    shop_setting_delivery (id) {
        id -> Unsigned<Integer>,
        shop_id -> Unsigned<Integer>,
        delivery_id -> Unsigned<Integer>,
        name -> Varchar,
        description -> Varchar,
        is_default -> Bool,
        delivery_type -> Unsigned<Integer>,
        delivery_scope -> Unsigned<Integer>,
        delivery_fee -> Unsigned<Decimal>,
        delivery_free_fee -> Unsigned<Decimal>,
        delivery_free_fee_type -> Unsigned<Integer>,
        delivery_free_fee_count -> Unsigned<Integer>,
        delivery_free_fee_weight -> Unsigned<Decimal>,
        delivery_free_fee_volume -> Unsigned<Decimal>,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    shop_setting_financial (id) {
        id -> Unsigned<Integer>,
        shop_id -> Unsigned<Integer>,
        owner_id -> Unsigned<Integer>,
        bank_no -> Varchar,
        bank_name -> Varchar,
        bank_branch -> Varchar,
        bank_account_name -> Varchar,
        bank_account_address -> Varchar,
        bank_account_phone -> Varchar,
        bank_account_identity_no -> Varchar,
        alipay_account -> Varchar,
        alipay_name -> Varchar,
        wechat_account -> Varchar,
        wechat_name -> Varchar,
        paypal_account -> Varchar,
        paypal_name -> Varchar,
        eth_address -> Varchar,
        usdt_address -> Varchar,
        btc_address -> Varchar,
        doge_address -> Varchar,
        status -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    shop_verify (id) {
        id -> Unsigned<Integer>,
        shop_id -> Unsigned<Integer>,
        owner_id -> Unsigned<Integer>,
        verify_type -> Unsigned<Integer>,
        owner_corp -> Varchar,
        owner_corp_code -> Varchar,
        owner_corp_file -> Varchar,
        owner_corp_address -> Varchar,
        owner_corp_tel -> Varchar,
        owner_corp_fax -> Varchar,
        owner_name -> Varchar,
        owner_phone -> Varchar,
        owner_email -> Varchar,
        owner_id_card -> Varchar,
        owner_id_card_front -> Varchar,
        owner_id_card_back -> Varchar,
        owner_id_card_hand_front -> Varchar,
        owner_id_card_hand_back -> Varchar,
        verify_status -> Unsigned<Integer>,
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
    brand,
    cart,
    comment,
    license_capacity,
    order,
    order_item,
    order_template,
    product,
    product_attribute,
    product_category,
    shop,
    shop_bill_daily,
    shop_employee,
    shop_order,
    shop_product,
    shop_setting_delivery,
    shop_setting_financial,
    shop_verify,
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
