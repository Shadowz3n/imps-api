table! {
    cities (city_id) {
        city_id -> Int8,
        city_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    countries (country_id) {
        country_id -> Int8,
        country_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    estates (estate_id) {
        estate_id -> Int8,
        estate_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    logs (id) {
        id -> Int8,
        log_action -> Varchar,
        ip -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        user_id -> Int8,
    }
}

table! {
    roles (role_id) {
        role_id -> Int8,
        role_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int8,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        pass -> Varchar,
        phone -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    users_addresses (id) {
        id -> Int8,
        address -> Varchar,
        address_number -> Nullable<Varchar>,
        address_complement -> Nullable<Varchar>,
        city_id -> Int8,
        estate_id -> Int8,
        country_id -> Int8,
        zip_code -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        user_id -> Int8,
    }
}

table! {
    users_roles (id) {
        id -> Int8,
        user_id -> Int8,
        role_id -> Int8,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    whois (whois_id) {
        whois_id -> Int8,
        domain -> Varchar,
        ip -> Varchar,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    whois_infos (whois_info_id) {
        whois_info_id -> Int8,
        owner -> Nullable<Varchar>,
        owner_account -> Nullable<Varchar>,
        admin_account -> Nullable<Varchar>,
        tech_account -> Nullable<Varchar>,
        billing_account -> Nullable<Varchar>,
        n_server -> Nullable<Varchar>,
        n_stat -> Nullable<Varchar>,
        n_last_stat -> Nullable<Varchar>,
        n_created -> Nullable<Varchar>,
        n_updated -> Nullable<Varchar>,
        status -> Nullable<Varchar>,
        registrar -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        whois_id -> Int8,
    }
}

table! {
    whois_users (whois_user_id) {
        whois_user_id -> Int8,
        role -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
        nic -> Nullable<Varchar>,
        organization -> Nullable<Varchar>,
        street -> Nullable<Varchar>,
        city -> Nullable<Varchar>,
        state -> Nullable<Varchar>,
        postal_code -> Nullable<Varchar>,
        country -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        phone_ext -> Nullable<Varchar>,
        fax -> Nullable<Varchar>,
        fax_ext -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        whois_info_id -> Int8,
    }
}

joinable!(logs -> users (id));
joinable!(users_addresses -> users (id));
joinable!(users_roles -> roles (role_id));
joinable!(users_roles -> users (id));
joinable!(whois_infos -> whois (whois_id));
joinable!(whois_users -> whois_infos (whois_info_id));

allow_tables_to_appear_in_same_query!(
    cities,
    countries,
    estates,
    logs,
    roles,
    users,
    users_addresses,
    users_roles,
    whois,
    whois_infos,
    whois_users,
);
